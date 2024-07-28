use {
    super::{
        reverted_error::*,
        utils::{into_anyhow, Result},
    },
    anyhow::anyhow,
    ethers::{types::Address, utils::parse_ether},
    ethers_contract::{ContractError, ContractFactory},
    grammers_client::{types::LoginToken, Client, Config, SignInError},
    grammers_session::Session,
    openapi_ethers::{
        client::Client as EthereumClient,
        erc20::{self as erc20_etherman, ERC20 as ERC20Contract, ERC20_ABI},
    },
    openapi_logger::debug,
    openapi_proto::authtelegram_service::{auth_telegram_server::AuthTelegram, *},
    serde_json::json,
    std::fs,
    std::{env, ptr::null, sync::Arc},
    tonic::{Request, Response, Status},
    uuid::Uuid,
};

#[derive(Debug, Clone)]
pub struct AuthTelegramService {
    client: Arc<EthereumClient>,
}

impl AuthTelegramService {
    pub fn new(client: Arc<EthereumClient>) -> Self {
        Self { client }
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
        println!("{}", phone_number);
        //
        let api_id = env::var("TELEGRAM_API_ID").map_err(|e| into_anyhow(e.into()))?;
        let api_hash = env::var("TELEGRAM_API_HASH").map_err(|e| into_anyhow(e.into()))?;
        println!("{}", api_id);
        println!("{}", api_hash);
        //
        println!("Connecting to Telegram...");
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
        println!("Connected!");
        // If we can't save the session, sign out once we're done.
        if !client
            .is_authorized()
            .await
            .map_err(|e| into_anyhow(e.into()))?
        {
            println!("Sending Code ...");
            client
                .request_login_code(&phone_number)
                .await
                .map_err(|e| into_anyhow(e.into()))?;
            println!("Sent Code ...");
            match client.session().save_to_file(&session_file) {
                Ok(_) => {}
                Err(e) => {
                    println!("NOTE: failed to save the session, will sign out when done: {e}");
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
        let SignInTelegramRequest {
            phone_number,
            session_uuid,
            code,
            code2_fa,
        } = req.into_inner();

        println!("{}", phone_number);
        //
        let api_id = env::var("TELEGRAM_API_ID").map_err(|e| into_anyhow(e.into()))?;
        let api_hash = env::var("TELEGRAM_API_HASH").map_err(|e| into_anyhow(e.into()))?;
        println!("{}", api_id);
        println!("{}", api_hash);
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
        println!("Connected!");

        if !client
            .is_authorized()
            .await
            .map_err(|e| into_anyhow(e.into()))?
        {
            println!("Sending Code ...");
            let token = client
                .request_login_code(&phone_number)
                .await
                .map_err(|e| into_anyhow(e.into()))?;
            println!("Sent Code ...");
            let signed_in = client.sign_in(&token, &code).await;
            match signed_in {
                Err(SignInError::PasswordRequired(password_token)) => {
                    // Note: this `prompt` method will echo the password in the console.
                    //       Real code might want to use a better way to handle this.
                    let hint = password_token.hint().unwrap_or("None");
                    let password = code2_fa.clone();

                    client
                        .check_password(password_token, password.trim())
                        .await
                        .map_err(|e| into_anyhow(e.into()))?;
                }
                Ok(_) => (),
                Err(e) => {
                    return Err(into_anyhow(e.into()));
                }
            };
            println!("Signed in!");
            match client.session().save_to_file(&session_file) {
                Ok(_) => {}
                Err(e) => {
                    println!("NOTE: failed to save the session, will sign out when done: {e}");
                    return Err(into_anyhow(e.into()));
                }
            }
            //
        }

        // Get JWT
        Ok(Response::new(SignInTelegramResponse {
            jwt: "".to_string(),
        }))
    }
    async fn log_out_telegram(
        &self,
        req: Request<LogOutTelegramRequest>,
    ) -> Result<Response<LogOutTelegramResponse>> {
        let LogOutTelegramRequest {
            phone_number,
            session_uuid,
        } = req.into_inner();

        println!("{}", phone_number);
        //
        let api_id = env::var("TELEGRAM_API_ID").map_err(|e| into_anyhow(e.into()))?;
        let api_hash = env::var("TELEGRAM_API_HASH").map_err(|e| into_anyhow(e.into()))?;
        println!("{}", api_id);
        println!("{}", api_hash);
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
        println!("Connected!");

        if client
            .is_authorized()
            .await
            .map_err(|e| into_anyhow(e.into()))?
        {
            println!("Log out ...");
            client.sign_out().await.map_err(|e| into_anyhow(e.into()))?;
            println!("Logged out!");

            // Attempt to remove the file
            match fs::remove_file(&session_file) {
                Ok(_) => {}
                Err(e) => {
                    println!("NOTE: failed to save the session, will sign out when done: {e}");
                    return Err(into_anyhow(e.into()));
                }
            }
            //
        }

        Ok(Response::new(LogOutTelegramResponse {
            phone_number: phone_number,
            session_uuid: session_uuid,
            message: "Logged out".to_string(),
        }))
    }

    async fn get_data_request_for_zion(
        &self,
        req: Request<GetDataRequestForZionRequest>,
    ) -> Result<Response<GetDataRequestForZionResponse>> {
        let proof_points = ProofPoints {
            protocol: "example_protocol".to_string(),
            pi_a: vec!["a".to_string(), "b".to_string()],
            pi_b: vec![
                StringArray {
                    values: vec!["x".to_string(), "y".to_string()],
                },
                StringArray {
                    values: vec!["z".to_string()],
                },
            ],
            pi_c: vec!["c1".to_string(), "c2".to_string()],
        };
        let response = GetDataRequestForZionResponse {
            salt: "some_salt".to_string(),
            proof: Some(proof_points), // Wrapping the proof_points in Some
            ephemeral_key_pair: "ephemeral_key".to_string(),
            beneficiaries: vec!["beneficiary1".to_string(), "beneficiary2".to_string()],
        };
        Ok(Response::new(response))
    }
}
