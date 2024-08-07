use {
    crate::error::{into_anyhow, Result},
    crate::{
        config::TelegramAuthConfig,
        entity::telegram::LoginWidgetData,
        helpers::telegram::{authorize, get_init_data_integrity_web},
    },
    chrono::Utc,
    grammers_client::{Client, Config, SignInError},
    grammers_session::Session,
    openapi_logger::debug,
    openapi_proto::authtelegram_service::{auth_telegram_server::AuthTelegram, *},
    std::fs,
    tonic::{Request, Response, Status},
    uuid::Uuid,
};

#[derive(Debug, Clone)]
pub struct AuthTelegramService {
    pub cfg: TelegramAuthConfig,
}

impl AuthTelegramService {
    pub fn new(telegram_auth: TelegramAuthConfig) -> Self {
        Self { cfg: telegram_auth }
    }
}

#[tonic::async_trait]
impl AuthTelegram for AuthTelegramService {
    async fn send_code_telegram(
        &self,
        req: Request<SendCodeTelegramRequest>,
    ) -> Result<Response<SendCodeTelegramResponse>> {
        debug!("{req:?}");
        let SendCodeTelegramRequest { phone_number } = req.into_inner();
        //
        let telegram_api_id = self.cfg.telegram_api_id;
        let telegram_api_hash = &self.cfg.telegram_api_hash;
        //
        let session_uuid = Uuid::new_v4();
        let session_file: String = format!(
            "{}/session_{}",
            self.cfg.session_path,
            session_uuid.to_string()
        );
        let client = Client::connect(Config {
            session: Session::load_file_or_create(&session_file)?,
            api_id: telegram_api_id.clone(),
            api_hash: telegram_api_hash.clone(),
            params: Default::default(),
        })
        .await
        .map_err(|e| into_anyhow(e.into()))?;
        // If we can't save the session, sign out once we're done.
        if !client
            .is_authorized()
            .await
            .map_err(|e| into_anyhow(e.into()))?
        {
            client
                .request_login_code(&phone_number)
                .await
                .map_err(|e| into_anyhow(e.into()))?;
            match client.session().save_to_file(&session_file) {
                Ok(_) => {}
                Err(e) => {
                    return Err(into_anyhow(e.into()));
                }
            }
        }

        Ok(Response::new(SendCodeTelegramResponse {
            phone_number: phone_number,
            session_uuid: session_uuid.to_string(),
        }))
    }

    async fn sign_in_telegram(
        &self,
        req: Request<SignInTelegramRequest>,
    ) -> Result<Response<SignInTelegramResponse>> {
        let mut jwt: String = "".to_string();
        let SignInTelegramRequest {
            phone_number,
            session_uuid,
            code,
            code2_fa,
        } = req.into_inner();

        //
        let telegram_api_id = self.cfg.telegram_api_id.clone();
        let telegram_api_hash = self.cfg.telegram_api_hash.clone();
        //
        let session_file: String = format!(
            "{}/session_{}",
            self.cfg.session_path,
            session_uuid.to_string()
        );
        let client = Client::connect(Config {
            session: Session::load_file_or_create(&session_file)?,
            api_id: telegram_api_id.clone(),
            api_hash: telegram_api_hash.clone(),
            params: Default::default(),
        })
        .await
        .map_err(|e| into_anyhow(e.into()))?;

        if !client
            .is_authorized()
            .await
            .map_err(|e| into_anyhow(e.into()))?
        {
            let token = client
                .request_login_code(&phone_number)
                .await
                .map_err(|e| into_anyhow(e.into()))?;
            let signed_in = client.sign_in(&token, &code).await;
            match signed_in {
                Err(SignInError::PasswordRequired(password_token)) => {
                    // Note: this `prompt` method will echo the password in the console.
                    //       Real code might want to use a better way to handle this.
                    // let hint = password_token.hint().unwrap_or("None");
                    let password = match code2_fa {
                        Some(code2_fa) => code2_fa,
                        None => {
                            return Err(Status::new(
                                tonic::Code::InvalidArgument,
                                "2FA code is required",
                            ));
                        }
                    };
                    client
                        .check_password(password_token, password.clone().trim())
                        .await
                        .map_err(|e| into_anyhow(e.into()))?;
                }
                Ok(_) => (),
                Err(e) => {
                    return Err(into_anyhow(e.into()));
                }
            };
            match client.session().save_to_file(&session_file) {
                Ok(_) => {}
                Err(e) => {
                    return Err(into_anyhow(e.into()));
                }
            }

            // Get JWT
            let user = client.get_me().await.map_err(|e| into_anyhow(e.into()))?;
            let now = Utc::now();
            // Number of seconds since the Unix epoch
            let auth_date: u32 = now.timestamp() as u32;
            // Convert Option<&str> to Option<String>
            let data_user: LoginWidgetData = LoginWidgetData {
                id: user.id(),
                first_name: user.first_name().to_string(),
                last_name: user
                    .last_name()
                    .map(|name| name.to_string())
                    .or_else(|| Some("".to_string())),
                username: user.username().map(|name| name.to_string()),
                photo_url: Some("".to_string()),
                auth_date: auth_date,
                hash: Some("".to_string()),
            };
            let token_auth_bot = self.cfg.token_auth_bot.clone();
            let data_user_get_info: LoginWidgetData =
                get_init_data_integrity_web(&data_user, &token_auth_bot);
            let base_url = self.cfg.next_public_server_login_author.clone();
            let client_id = self.cfg.client_id.clone();
            match authorize(
                &base_url,
                &client_id,
                data_user_get_info.clone(),
                &session_uuid,
            )
            .await
            {
                Ok(response) => {
                    if let Some(data) = response.id_token {
                        jwt = data
                    }
                    if let Some(error) = response.error {
                        println!("Error: {}", error);
                        return Err(Status::new(tonic::Code::Aborted, format!("{}", error)));
                    }
                }
                Err(e) => {
                    return Err(into_anyhow(e.into()));
                }
            }
            //
        }

        // Get JWT
        Ok(Response::new(SignInTelegramResponse {
            jwt: jwt.to_string(),
            session_uuid: session_uuid,
        }))
    }
    async fn log_out_telegram(
        &self,
        req: Request<LogOutTelegramRequest>,
    ) -> Result<Response<LogOutTelegramResponse>> {
        let LogOutTelegramRequest { session_uuid } = req.into_inner();
        //
        let telegram_api_id = self.cfg.telegram_api_id.clone();
        let telegram_api_hash = self.cfg.telegram_api_hash.clone();
        //
        let session_file: String = format!(
            "{}/session_{}",
            self.cfg.session_path,
            session_uuid.to_string()
        );
        let client = Client::connect(Config {
            session: Session::load_file_or_create(&session_file)?,
            api_id: telegram_api_id.clone(),
            api_hash: telegram_api_hash.clone(),
            params: Default::default(),
        })
        .await
        .map_err(|e| into_anyhow(e.into()))?;

        if client
            .is_authorized()
            .await
            .map_err(|e| into_anyhow(e.into()))?
        {
            client.sign_out().await.map_err(|e| into_anyhow(e.into()))?;
            // Attempt to remove the file
            match fs::remove_file(&session_file) {
                Ok(_) => {}
                Err(e) => {
                    return Err(into_anyhow(e.into()));
                }
            }
            //
        }

        Ok(Response::new(LogOutTelegramResponse {
            session_uuid: session_uuid,
            message: "Logged out".to_string(),
        }))
    }

    async fn sign_in_telegram_as_bot(
        &self,
        req: Request<SignInTelegramAsBotRequest>,
    ) -> Result<Response<SignInTelegramAsBotResponse>> {
        let mut jwt: String = "".to_string();
        let SignInTelegramAsBotRequest { token_auth } = req.into_inner();
        //
        let telegram_api_id = self.cfg.telegram_api_id.clone();
        let telegram_api_hash = self.cfg.telegram_api_hash.clone();
        //
        let session_uuid = Uuid::new_v4();
        let session_file: String = format!(
            "{}/session_{}",
            self.cfg.session_path,
            session_uuid.to_string()
        );
        let client = Client::connect(Config {
            session: Session::load_file_or_create(&session_file)?,
            api_id: telegram_api_id.clone(),
            api_hash: telegram_api_hash.clone(),
            params: Default::default(),
        })
        .await
        .map_err(|e| into_anyhow(e.into()))?;

        if !client
            .is_authorized()
            .await
            .map_err(|e| into_anyhow(e.into()))?
        {
            let _ = client
                .bot_sign_in(&token_auth)
                .await
                .map_err(|e| into_anyhow(e.into()))?;
            match client.session().save_to_file(&session_file) {
                Ok(_) => {}
                Err(e) => {
                    return Err(into_anyhow(e.into()));
                }
            }
            // Get JWT
            let user = client.get_me().await.map_err(|e| into_anyhow(e.into()))?;
            let now = Utc::now();
            // Number of seconds since the Unix epoch
            let auth_date: u32 = now.timestamp() as u32;
            // Convert Option<&str> to Option<String>
            let data_user: LoginWidgetData = LoginWidgetData {
                id: user.id(),
                first_name: user.first_name().to_string(),
                last_name: user
                    .last_name()
                    .map(|name| name.to_string())
                    .or_else(|| Some("".to_string())),
                username: user.username().map(|name| name.to_string()),
                photo_url: Some("".to_string()),
                auth_date: auth_date,
                hash: Some("".to_string()),
            };
            let token_auth_bot = self.cfg.token_auth_bot.clone();
            let data_user_get_info: LoginWidgetData =
                get_init_data_integrity_web(&data_user, &token_auth_bot);
            let base_url = self.cfg.next_public_server_login_author.clone();
            let client_id = self.cfg.client_id.clone();
            match authorize(
                &base_url,
                &client_id,
                data_user_get_info.clone(),
                &session_uuid.to_string(),
            )
            .await
            {
                Ok(response) => {
                    if let Some(data) = response.id_token {
                        jwt = data
                    }
                    if let Some(error) = response.error {
                        println!("Error: {}", error);
                        return Err(Status::new(tonic::Code::Aborted, format!("{}", error)));
                    }
                }
                Err(e) => {
                    return Err(into_anyhow(e.into()));
                }
            }
        }

        // Get JWT
        Ok(Response::new(SignInTelegramAsBotResponse {
            jwt: jwt.to_string(),
            session_uuid: session_uuid.to_string(),
        }))
    }
    async fn log_out_telegram_as_bot(
        &self,
        req: Request<LogOutTelegramAsbotRequest>,
    ) -> Result<Response<LogOutTelegramAsBotResponse>> {
        let LogOutTelegramAsbotRequest { session_uuid } = req.into_inner();
        //
        let telegram_api_id = self.cfg.telegram_api_id.clone();
        let telegram_api_hash = self.cfg.telegram_api_hash.clone();
        //
        let session_file: String = format!(
            "{}/session_{}",
            self.cfg.session_path,
            session_uuid.to_string()
        );
        let client = Client::connect(Config {
            session: Session::load_file_or_create(&session_file)?,
            api_id: telegram_api_id.clone(),
            api_hash: telegram_api_hash.clone(),
            params: Default::default(),
        })
        .await
        .map_err(|e| into_anyhow(e.into()))?;

        if client
            .is_authorized()
            .await
            .map_err(|e| into_anyhow(e.into()))?
        {
            client.sign_out().await.map_err(|e| into_anyhow(e.into()))?;

            // Attempt to remove the file
            match fs::remove_file(&session_file) {
                Ok(_) => {}
                Err(e) => {
                    return Err(into_anyhow(e.into()));
                }
            }
            //
        }

        Ok(Response::new(LogOutTelegramAsBotResponse {
            session_uuid: session_uuid,
            message: "Logged out".to_string(),
        }))
    }
}
