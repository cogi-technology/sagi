use {
    super::utils::{into_anyhow, Result},
    crate::{
        entity::telegram::LoginWidgetData,
        helpers::telegram::{authorize, get_init_data_integrity_web},
    },
    chrono::Utc,
    grammers_client::{Client, Config, SignInError},
    grammers_session::Session,
    openapi_logger::debug,
    openapi_proto::authtelegram_service::{auth_telegram_server::AuthTelegram, *},
    std::{env, fs},
    tonic::{Request, Response, Status},
    uuid::Uuid,
};

#[derive(Debug, Clone)]
pub struct AuthTelegramService {}

impl AuthTelegramService {
    pub fn new() -> Self {
        Self {}
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
        let api_id = env::var("TELEGRAM_API_ID").map_err(|e| into_anyhow(e.into()))?;
        let api_hash = env::var("TELEGRAM_API_HASH").map_err(|e| into_anyhow(e.into()))?;
        //
        let session_uuid = Uuid::new_v4();
        // Extract the directory of the executable
        let session_file: String = format!("session_{}", session_uuid.to_string());
        let client = Client::connect(Config {
            session: Session::load_file_or_create(&session_file)?,
            api_id: api_id.parse::<i32>().expect("API_ID missing"),
            api_hash: api_hash.clone(),
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
            //
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
        let api_id = env::var("TELEGRAM_API_ID").map_err(|e| into_anyhow(e.into()))?;
        let api_hash = env::var("TELEGRAM_API_HASH").map_err(|e| into_anyhow(e.into()))?;
        //
        let session_file: String = format!("session_{}", session_uuid.to_string());
        let client = Client::connect(Config {
            session: Session::load_file_or_create(&session_file)?,
            api_id: api_id.parse::<i32>().expect("1"),
            api_hash: api_hash.clone(),
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
            let token_auth_bot = env::var("TOKEN_AUTH_BOT").map_err(|e| into_anyhow(e.into()))?;
            let data_user_get_info: LoginWidgetData =
                get_init_data_integrity_web(&data_user, &token_auth_bot);
            let base_url =
                env::var("NEXT_PUBLIC_SERVER_LOGIN_AUTHOR").map_err(|e| into_anyhow(e.into()))?;
            let client_id = env::var("CLIENT_ID").map_err(|e| into_anyhow(e.into()))?;
            match authorize(&base_url, &client_id, data_user_get_info.clone()).await {
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
        let api_id = env::var("TELEGRAM_API_ID").map_err(|e| into_anyhow(e.into()))?;
        let api_hash = env::var("TELEGRAM_API_HASH").map_err(|e| into_anyhow(e.into()))?;
        //
        let session_file: String = format!("session_{}", session_uuid.to_string());
        let client = Client::connect(Config {
            session: Session::load_file_or_create(&session_file)?,
            api_id: api_id.parse::<i32>().expect("1"),
            api_hash: api_hash.clone(),
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
        let api_id = env::var("TELEGRAM_API_ID").map_err(|e| into_anyhow(e.into()))?;
        let api_hash = env::var("TELEGRAM_API_HASH").map_err(|e| into_anyhow(e.into()))?;
        //
        let session_uuid = Uuid::new_v4();
        let session_file: String = format!("session_{}", session_uuid.to_string());
        let client = Client::connect(Config {
            session: Session::load_file_or_create(&session_file)?,
            api_id: api_id.parse::<i32>().expect("1"),
            api_hash: api_hash.clone(),
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
            let token_auth_bot = env::var("TOKEN_AUTH_BOT").map_err(|e| into_anyhow(e.into()))?;
            let data_user_get_info: LoginWidgetData =
                get_init_data_integrity_web(&data_user, &token_auth_bot);
            let base_url =
                env::var("NEXT_PUBLIC_SERVER_LOGIN_AUTHOR").map_err(|e| into_anyhow(e.into()))?;
            let client_id = env::var("CLIENT_ID").map_err(|e| into_anyhow(e.into()))?;
            match authorize(&base_url, &client_id, data_user_get_info.clone()).await {
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
        let api_id = env::var("TELEGRAM_API_ID").map_err(|e| into_anyhow(e.into()))?;
        let api_hash = env::var("TELEGRAM_API_HASH").map_err(|e| into_anyhow(e.into()))?;
        //
        let session_file: String = format!("session_{}", session_uuid.to_string());
        let client = Client::connect(Config {
            session: Session::load_file_or_create(&session_file)?,
            api_id: api_id.parse::<i32>().expect("1"),
            api_hash: api_hash.clone(),
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
