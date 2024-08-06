/// Define messages for the requests and responses for ERC404
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeployRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub initial_supply: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub units: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "5")]
    pub ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "6")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub pin_code: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeployResponse {
    #[prost(string, tag = "1")]
    pub contract: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TotalSupplyRequest {
    #[prost(string, tag = "1")]
    pub contract: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TotalSupplyResponse {
    #[prost(string, tag = "1")]
    pub total_supply: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceOfRequest {
    #[prost(string, tag = "1")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub account: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceOfResponse {
    #[prost(string, tag = "1")]
    pub amount: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllowanceRequest {
    #[prost(string, tag = "1")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub spender: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllowanceResponse {
    #[prost(string, tag = "1")]
    pub remaining_amount: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApproveRequest {
    #[prost(string, tag = "1")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub spender: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub pin_code: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApproveResponse {
    #[prost(string, tag = "1")]
    pub txhash: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferRequest {
    #[prost(string, tag = "1")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub pin_code: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferResponse {
    #[prost(string, tag = "1")]
    pub txhash: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferFromRequest {
    #[prost(string, tag = "1")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub value: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub pin_code: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferFromResponse {
    #[prost(string, tag = "1")]
    pub txhash: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceOfBatchRequest {
    #[prost(string, tag = "1")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "3")]
    pub token_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceOfBatchResponse {
    #[prost(string, repeated, tag = "1")]
    pub batch_balances: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetApprovalForAllRequest {
    #[prost(string, tag = "1")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub operator: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub approved: bool,
    #[prost(string, tag = "4")]
    pub pin_code: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetApprovalForAllResponse {
    #[prost(string, tag = "1")]
    pub txhash: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsApprovedForAllRequest {
    #[prost(string, tag = "1")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub operator: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsApprovedForAllResponse {
    #[prost(bool, tag = "1")]
    pub result: bool,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SafeTransferFromRequest {
    #[prost(string, tag = "1")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub token_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub value: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub data: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub pin_code: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SafeTransferFromResponse {
    #[prost(string, tag = "1")]
    pub txhash: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SafeBatchTransferFromRequest {
    #[prost(string, tag = "1")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "4")]
    pub token_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "5")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bytes = "bytes", tag = "6")]
    pub data: ::prost::bytes::Bytes,
    #[prost(string, tag = "7")]
    pub pin_code: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SafeBatchTransferFromResponse {
    #[prost(string, tag = "1")]
    pub txhash: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc1155BalanceOfRequest {
    #[prost(string, tag = "1")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub token_id: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc1155BalanceOfResponse {
    #[prost(string, tag = "1")]
    pub amount: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc20BalanceOfRequest {
    #[prost(string, tag = "1")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub account: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc20BalanceOfResponse {
    #[prost(string, tag = "1")]
    pub amount: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc1155TransferExemptRequest {
    #[prost(string, tag = "1")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub target: ::prost::alloc::string::String,
}
#[actix_prost_macros::serde(rename_all = "snake_case")]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc1155TransferExemptResponse {
    #[prost(bool, tag = "1")]
    pub result: bool,
}
pub mod erc404_actix {
    #![allow(unused_variables, dead_code, missing_docs)]
    use super::*;
    use super::erc404_server::Erc404;
    use std::sync::Arc;
    /// Define messages for the requests and responses for ERC404
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DeployJson {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub symbol: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub initial_supply: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub units: ::prost::alloc::string::String,
        #[prost(string, repeated, tag = "5")]
        pub ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(string, tag = "6")]
        pub uri: ::prost::alloc::string::String,
        #[prost(string, tag = "7")]
        pub pin_code: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TotalSupplyQuery {
        #[prost(string, tag = "1")]
        pub contract: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BalanceOfQuery {
        #[prost(string, tag = "1")]
        pub contract: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub account: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AllowanceQuery {
        #[prost(string, tag = "1")]
        pub contract: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub owner: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub spender: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ApproveJson {
        #[prost(string, tag = "1")]
        pub contract: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub spender: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub amount: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub pin_code: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TransferJson {
        #[prost(string, tag = "1")]
        pub contract: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub recipient: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub amount: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub pin_code: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TransferFromJson {
        #[prost(string, tag = "1")]
        pub contract: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub from: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub to: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub value: ::prost::alloc::string::String,
        #[prost(string, tag = "5")]
        pub pin_code: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BalanceOfBatchQuery {
        #[prost(string, tag = "1")]
        pub contract: ::prost::alloc::string::String,
        #[prost(string, repeated, tag = "2")]
        pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(string, repeated, tag = "3")]
        pub token_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetApprovalForAllJson {
        #[prost(string, tag = "1")]
        pub contract: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub operator: ::prost::alloc::string::String,
        #[prost(bool, tag = "3")]
        pub approved: bool,
        #[prost(string, tag = "4")]
        pub pin_code: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IsApprovedForAllQuery {
        #[prost(string, tag = "1")]
        pub contract: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub owner: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub operator: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SafeTransferFromJson {
        #[prost(string, tag = "1")]
        pub contract: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub from: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub to: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub token_id: ::prost::alloc::string::String,
        #[prost(string, tag = "5")]
        pub value: ::prost::alloc::string::String,
        #[prost(string, tag = "6")]
        pub data: ::prost::alloc::string::String,
        #[prost(string, tag = "7")]
        pub pin_code: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SafeBatchTransferFromJson {
        #[prost(string, tag = "1")]
        pub contract: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub from: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub to: ::prost::alloc::string::String,
        #[prost(string, repeated, tag = "4")]
        pub token_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(string, repeated, tag = "5")]
        pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(bytes = "bytes", tag = "6")]
        pub data: ::prost::bytes::Bytes,
        #[prost(string, tag = "7")]
        pub pin_code: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ERC1155BalanceOfQuery {
        #[prost(string, tag = "1")]
        pub contract: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub account: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub token_id: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ERC20BalanceOfQuery {
        #[prost(string, tag = "1")]
        pub contract: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub account: ::prost::alloc::string::String,
    }
    #[actix_prost_macros::serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ERC1155TransferExemptQuery {
        #[prost(string, tag = "1")]
        pub contract: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub target: ::prost::alloc::string::String,
    }
    async fn call_deploy(
        service: ::actix_web::web::Data<dyn Erc404 + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<DeployResponse>, ::actix_prost::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            DeployJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = DeployRequest {
            name: json.name,
            symbol: json.symbol,
            initial_supply: json.initial_supply,
            units: json.units,
            ids: json.ids,
            uri: json.uri,
            pin_code: json.pin_code,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.deploy(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_total_supply(
        service: ::actix_web::web::Data<dyn Erc404 + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
    ) -> Result<::actix_web::web::Json<TotalSupplyResponse>, ::actix_prost::Error> {
        let query = <::actix_web::web::Query<
            TotalSupplyQuery,
        > as ::actix_web::FromRequest>::extract(&http_request)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = TotalSupplyRequest {
            contract: query.contract,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.total_supply(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_balance_of(
        service: ::actix_web::web::Data<dyn Erc404 + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
    ) -> Result<::actix_web::web::Json<BalanceOfResponse>, ::actix_prost::Error> {
        let query = <::actix_web::web::Query<
            BalanceOfQuery,
        > as ::actix_web::FromRequest>::extract(&http_request)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = BalanceOfRequest {
            contract: query.contract,
            account: query.account,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.balance_of(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_allowance(
        service: ::actix_web::web::Data<dyn Erc404 + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
    ) -> Result<::actix_web::web::Json<AllowanceResponse>, ::actix_prost::Error> {
        let query = <::actix_web::web::Query<
            AllowanceQuery,
        > as ::actix_web::FromRequest>::extract(&http_request)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = AllowanceRequest {
            contract: query.contract,
            owner: query.owner,
            spender: query.spender,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.allowance(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_approve(
        service: ::actix_web::web::Data<dyn Erc404 + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<ApproveResponse>, ::actix_prost::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            ApproveJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = ApproveRequest {
            contract: json.contract,
            spender: json.spender,
            amount: json.amount,
            pin_code: json.pin_code,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.approve(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_transfer(
        service: ::actix_web::web::Data<dyn Erc404 + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<TransferResponse>, ::actix_prost::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            TransferJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = TransferRequest {
            contract: json.contract,
            recipient: json.recipient,
            amount: json.amount,
            pin_code: json.pin_code,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.transfer(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_transfer_from(
        service: ::actix_web::web::Data<dyn Erc404 + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<TransferFromResponse>, ::actix_prost::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            TransferFromJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = TransferFromRequest {
            contract: json.contract,
            from: json.from,
            to: json.to,
            value: json.value,
            pin_code: json.pin_code,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.transfer_from(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_balance_of_batch(
        service: ::actix_web::web::Data<dyn Erc404 + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
    ) -> Result<::actix_web::web::Json<BalanceOfBatchResponse>, ::actix_prost::Error> {
        let query = <::actix_web::web::Query<
            BalanceOfBatchQuery,
        > as ::actix_web::FromRequest>::extract(&http_request)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = BalanceOfBatchRequest {
            contract: query.contract,
            accounts: query.accounts,
            token_ids: query.token_ids,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.balance_of_batch(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_set_approval_for_all(
        service: ::actix_web::web::Data<dyn Erc404 + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<
        ::actix_web::web::Json<SetApprovalForAllResponse>,
        ::actix_prost::Error,
    > {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            SetApprovalForAllJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = SetApprovalForAllRequest {
            contract: json.contract,
            operator: json.operator,
            approved: json.approved,
            pin_code: json.pin_code,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.set_approval_for_all(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_is_approved_for_all(
        service: ::actix_web::web::Data<dyn Erc404 + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
    ) -> Result<::actix_web::web::Json<IsApprovedForAllResponse>, ::actix_prost::Error> {
        let query = <::actix_web::web::Query<
            IsApprovedForAllQuery,
        > as ::actix_web::FromRequest>::extract(&http_request)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = IsApprovedForAllRequest {
            contract: query.contract,
            owner: query.owner,
            operator: query.operator,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.is_approved_for_all(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_safe_transfer_from(
        service: ::actix_web::web::Data<dyn Erc404 + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<::actix_web::web::Json<SafeTransferFromResponse>, ::actix_prost::Error> {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            SafeTransferFromJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = SafeTransferFromRequest {
            contract: json.contract,
            from: json.from,
            to: json.to,
            token_id: json.token_id,
            value: json.value,
            data: json.data,
            pin_code: json.pin_code,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.safe_transfer_from(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_safe_batch_transfer_from(
        service: ::actix_web::web::Data<dyn Erc404 + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
        payload: ::actix_web::web::Payload,
    ) -> Result<
        ::actix_web::web::Json<SafeBatchTransferFromResponse>,
        ::actix_prost::Error,
    > {
        let mut payload = payload.into_inner();
        let json = <::actix_web::web::Json<
            SafeBatchTransferFromJson,
        > as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = SafeBatchTransferFromRequest {
            contract: json.contract,
            from: json.from,
            to: json.to,
            token_ids: json.token_ids,
            values: json.values,
            data: json.data,
            pin_code: json.pin_code,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.safe_batch_transfer_from(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_erc1155_balance_of(
        service: ::actix_web::web::Data<dyn Erc404 + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
    ) -> Result<::actix_web::web::Json<Erc1155BalanceOfResponse>, ::actix_prost::Error> {
        let query = <::actix_web::web::Query<
            ERC1155BalanceOfQuery,
        > as ::actix_web::FromRequest>::extract(&http_request)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = Erc1155BalanceOfRequest {
            contract: query.contract,
            account: query.account,
            token_id: query.token_id,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.erc1155_balance_of(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_erc20_balance_of(
        service: ::actix_web::web::Data<dyn Erc404 + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
    ) -> Result<::actix_web::web::Json<Erc20BalanceOfResponse>, ::actix_prost::Error> {
        let query = <::actix_web::web::Query<
            ERC20BalanceOfQuery,
        > as ::actix_web::FromRequest>::extract(&http_request)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = Erc20BalanceOfRequest {
            contract: query.contract,
            account: query.account,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.erc20_balance_of(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    async fn call_erc1155_transfer_exempt(
        service: ::actix_web::web::Data<dyn Erc404 + Sync + Send + 'static>,
        http_request: ::actix_web::HttpRequest,
    ) -> Result<
        ::actix_web::web::Json<Erc1155TransferExemptResponse>,
        ::actix_prost::Error,
    > {
        let query = <::actix_web::web::Query<
            ERC1155TransferExemptQuery,
        > as ::actix_web::FromRequest>::extract(&http_request)
            .await
            .map_err(|err| ::actix_prost::Error::from_actix(
                err,
                ::tonic::Code::InvalidArgument,
            ))?
            .into_inner();
        let request = Erc1155TransferExemptRequest {
            contract: query.contract,
            target: query.target,
        };
        let request = ::actix_prost::new_request(request, &http_request);
        let response = service.erc1155_transfer_exempt(request).await?;
        let response = response.into_inner();
        Ok(::actix_web::web::Json(response))
    }
    pub fn route_erc404(
        config: &mut ::actix_web::web::ServiceConfig,
        service: Arc<dyn Erc404 + Send + Sync + 'static>,
    ) {
        config.app_data(::actix_web::web::Data::from(service));
        config.route("/api/erc404/deploy", ::actix_web::web::post().to(call_deploy));
        config
            .route(
                "/api/erc404/totalSupply",
                ::actix_web::web::get().to(call_total_supply),
            );
        config
            .route("/api/erc404/balanceOf", ::actix_web::web::get().to(call_balance_of));
        config
            .route("/api/erc404/allowance", ::actix_web::web::get().to(call_allowance));
        config.route("/api/erc404/approve", ::actix_web::web::post().to(call_approve));
        config.route("/api/erc404/transfer", ::actix_web::web::post().to(call_transfer));
        config
            .route(
                "/api/erc404/transferFrom",
                ::actix_web::web::post().to(call_transfer_from),
            );
        config
            .route(
                "/api/erc404/balanceOfBatch",
                ::actix_web::web::get().to(call_balance_of_batch),
            );
        config
            .route(
                "/api/erc404/setApprovalForAll",
                ::actix_web::web::post().to(call_set_approval_for_all),
            );
        config
            .route(
                "/api/erc404/isApprovedForAll",
                ::actix_web::web::get().to(call_is_approved_for_all),
            );
        config
            .route(
                "/api/erc404/safeTransferFrom",
                ::actix_web::web::post().to(call_safe_transfer_from),
            );
        config
            .route(
                "/api/erc404/safeBatchTransferFrom",
                ::actix_web::web::post().to(call_safe_batch_transfer_from),
            );
        config
            .route(
                "/api/erc404/erc1155BalanceOf",
                ::actix_web::web::get().to(call_erc1155_balance_of),
            );
        config
            .route(
                "/api/erc404/erc20BalanceOf",
                ::actix_web::web::get().to(call_erc20_balance_of),
            );
        config
            .route(
                "/api/erc404/erc1155TransferExempt",
                ::actix_web::web::get().to(call_erc1155_transfer_exempt),
            );
    }
}
/// Generated client implementations.
pub mod erc404_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Define the service for
    #[derive(Debug, Clone)]
    pub struct Erc404Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl Erc404Client<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> Erc404Client<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> Erc404Client<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            Erc404Client::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        pub async fn deploy(
            &mut self,
            request: impl tonic::IntoRequest<super::DeployRequest>,
        ) -> Result<tonic::Response<super::DeployResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/erc404.ERC404/Deploy");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn total_supply(
            &mut self,
            request: impl tonic::IntoRequest<super::TotalSupplyRequest>,
        ) -> Result<tonic::Response<super::TotalSupplyResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/erc404.ERC404/TotalSupply",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn balance_of(
            &mut self,
            request: impl tonic::IntoRequest<super::BalanceOfRequest>,
        ) -> Result<tonic::Response<super::BalanceOfResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/erc404.ERC404/BalanceOf");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn allowance(
            &mut self,
            request: impl tonic::IntoRequest<super::AllowanceRequest>,
        ) -> Result<tonic::Response<super::AllowanceResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/erc404.ERC404/Allowance");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn approve(
            &mut self,
            request: impl tonic::IntoRequest<super::ApproveRequest>,
        ) -> Result<tonic::Response<super::ApproveResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/erc404.ERC404/Approve");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn transfer(
            &mut self,
            request: impl tonic::IntoRequest<super::TransferRequest>,
        ) -> Result<tonic::Response<super::TransferResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/erc404.ERC404/Transfer");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn transfer_from(
            &mut self,
            request: impl tonic::IntoRequest<super::TransferFromRequest>,
        ) -> Result<tonic::Response<super::TransferFromResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/erc404.ERC404/TransferFrom",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn balance_of_batch(
            &mut self,
            request: impl tonic::IntoRequest<super::BalanceOfBatchRequest>,
        ) -> Result<tonic::Response<super::BalanceOfBatchResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/erc404.ERC404/BalanceOfBatch",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn set_approval_for_all(
            &mut self,
            request: impl tonic::IntoRequest<super::SetApprovalForAllRequest>,
        ) -> Result<tonic::Response<super::SetApprovalForAllResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/erc404.ERC404/SetApprovalForAll",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn is_approved_for_all(
            &mut self,
            request: impl tonic::IntoRequest<super::IsApprovedForAllRequest>,
        ) -> Result<tonic::Response<super::IsApprovedForAllResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/erc404.ERC404/IsApprovedForAll",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn safe_transfer_from(
            &mut self,
            request: impl tonic::IntoRequest<super::SafeTransferFromRequest>,
        ) -> Result<tonic::Response<super::SafeTransferFromResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/erc404.ERC404/SafeTransferFrom",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn safe_batch_transfer_from(
            &mut self,
            request: impl tonic::IntoRequest<super::SafeBatchTransferFromRequest>,
        ) -> Result<
            tonic::Response<super::SafeBatchTransferFromResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/erc404.ERC404/SafeBatchTransferFrom",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn erc1155_balance_of(
            &mut self,
            request: impl tonic::IntoRequest<super::Erc1155BalanceOfRequest>,
        ) -> Result<tonic::Response<super::Erc1155BalanceOfResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/erc404.ERC404/ERC1155BalanceOf",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn erc20_balance_of(
            &mut self,
            request: impl tonic::IntoRequest<super::Erc20BalanceOfRequest>,
        ) -> Result<tonic::Response<super::Erc20BalanceOfResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/erc404.ERC404/ERC20BalanceOf",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn erc1155_transfer_exempt(
            &mut self,
            request: impl tonic::IntoRequest<super::Erc1155TransferExemptRequest>,
        ) -> Result<
            tonic::Response<super::Erc1155TransferExemptResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/erc404.ERC404/ERC1155TransferExempt",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod erc404_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with Erc404Server.
    #[async_trait]
    pub trait Erc404: Send + Sync + 'static {
        async fn deploy(
            &self,
            request: tonic::Request<super::DeployRequest>,
        ) -> Result<tonic::Response<super::DeployResponse>, tonic::Status>;
        async fn total_supply(
            &self,
            request: tonic::Request<super::TotalSupplyRequest>,
        ) -> Result<tonic::Response<super::TotalSupplyResponse>, tonic::Status>;
        async fn balance_of(
            &self,
            request: tonic::Request<super::BalanceOfRequest>,
        ) -> Result<tonic::Response<super::BalanceOfResponse>, tonic::Status>;
        async fn allowance(
            &self,
            request: tonic::Request<super::AllowanceRequest>,
        ) -> Result<tonic::Response<super::AllowanceResponse>, tonic::Status>;
        async fn approve(
            &self,
            request: tonic::Request<super::ApproveRequest>,
        ) -> Result<tonic::Response<super::ApproveResponse>, tonic::Status>;
        async fn transfer(
            &self,
            request: tonic::Request<super::TransferRequest>,
        ) -> Result<tonic::Response<super::TransferResponse>, tonic::Status>;
        async fn transfer_from(
            &self,
            request: tonic::Request<super::TransferFromRequest>,
        ) -> Result<tonic::Response<super::TransferFromResponse>, tonic::Status>;
        async fn balance_of_batch(
            &self,
            request: tonic::Request<super::BalanceOfBatchRequest>,
        ) -> Result<tonic::Response<super::BalanceOfBatchResponse>, tonic::Status>;
        async fn set_approval_for_all(
            &self,
            request: tonic::Request<super::SetApprovalForAllRequest>,
        ) -> Result<tonic::Response<super::SetApprovalForAllResponse>, tonic::Status>;
        async fn is_approved_for_all(
            &self,
            request: tonic::Request<super::IsApprovedForAllRequest>,
        ) -> Result<tonic::Response<super::IsApprovedForAllResponse>, tonic::Status>;
        async fn safe_transfer_from(
            &self,
            request: tonic::Request<super::SafeTransferFromRequest>,
        ) -> Result<tonic::Response<super::SafeTransferFromResponse>, tonic::Status>;
        async fn safe_batch_transfer_from(
            &self,
            request: tonic::Request<super::SafeBatchTransferFromRequest>,
        ) -> Result<
            tonic::Response<super::SafeBatchTransferFromResponse>,
            tonic::Status,
        >;
        async fn erc1155_balance_of(
            &self,
            request: tonic::Request<super::Erc1155BalanceOfRequest>,
        ) -> Result<tonic::Response<super::Erc1155BalanceOfResponse>, tonic::Status>;
        async fn erc20_balance_of(
            &self,
            request: tonic::Request<super::Erc20BalanceOfRequest>,
        ) -> Result<tonic::Response<super::Erc20BalanceOfResponse>, tonic::Status>;
        async fn erc1155_transfer_exempt(
            &self,
            request: tonic::Request<super::Erc1155TransferExemptRequest>,
        ) -> Result<
            tonic::Response<super::Erc1155TransferExemptResponse>,
            tonic::Status,
        >;
    }
    /// Define the service for
    #[derive(Debug)]
    pub struct Erc404Server<T: Erc404> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Erc404> Erc404Server<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for Erc404Server<T>
    where
        T: Erc404,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/erc404.ERC404/Deploy" => {
                    #[allow(non_camel_case_types)]
                    struct DeploySvc<T: Erc404>(pub Arc<T>);
                    impl<T: Erc404> tonic::server::UnaryService<super::DeployRequest>
                    for DeploySvc<T> {
                        type Response = super::DeployResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeployRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).deploy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeploySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/erc404.ERC404/TotalSupply" => {
                    #[allow(non_camel_case_types)]
                    struct TotalSupplySvc<T: Erc404>(pub Arc<T>);
                    impl<
                        T: Erc404,
                    > tonic::server::UnaryService<super::TotalSupplyRequest>
                    for TotalSupplySvc<T> {
                        type Response = super::TotalSupplyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TotalSupplyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).total_supply(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TotalSupplySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/erc404.ERC404/BalanceOf" => {
                    #[allow(non_camel_case_types)]
                    struct BalanceOfSvc<T: Erc404>(pub Arc<T>);
                    impl<T: Erc404> tonic::server::UnaryService<super::BalanceOfRequest>
                    for BalanceOfSvc<T> {
                        type Response = super::BalanceOfResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BalanceOfRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).balance_of(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BalanceOfSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/erc404.ERC404/Allowance" => {
                    #[allow(non_camel_case_types)]
                    struct AllowanceSvc<T: Erc404>(pub Arc<T>);
                    impl<T: Erc404> tonic::server::UnaryService<super::AllowanceRequest>
                    for AllowanceSvc<T> {
                        type Response = super::AllowanceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AllowanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).allowance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AllowanceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/erc404.ERC404/Approve" => {
                    #[allow(non_camel_case_types)]
                    struct ApproveSvc<T: Erc404>(pub Arc<T>);
                    impl<T: Erc404> tonic::server::UnaryService<super::ApproveRequest>
                    for ApproveSvc<T> {
                        type Response = super::ApproveResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ApproveRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).approve(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ApproveSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/erc404.ERC404/Transfer" => {
                    #[allow(non_camel_case_types)]
                    struct TransferSvc<T: Erc404>(pub Arc<T>);
                    impl<T: Erc404> tonic::server::UnaryService<super::TransferRequest>
                    for TransferSvc<T> {
                        type Response = super::TransferResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransferRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).transfer(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TransferSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/erc404.ERC404/TransferFrom" => {
                    #[allow(non_camel_case_types)]
                    struct TransferFromSvc<T: Erc404>(pub Arc<T>);
                    impl<
                        T: Erc404,
                    > tonic::server::UnaryService<super::TransferFromRequest>
                    for TransferFromSvc<T> {
                        type Response = super::TransferFromResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransferFromRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).transfer_from(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TransferFromSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/erc404.ERC404/BalanceOfBatch" => {
                    #[allow(non_camel_case_types)]
                    struct BalanceOfBatchSvc<T: Erc404>(pub Arc<T>);
                    impl<
                        T: Erc404,
                    > tonic::server::UnaryService<super::BalanceOfBatchRequest>
                    for BalanceOfBatchSvc<T> {
                        type Response = super::BalanceOfBatchResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BalanceOfBatchRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).balance_of_batch(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BalanceOfBatchSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/erc404.ERC404/SetApprovalForAll" => {
                    #[allow(non_camel_case_types)]
                    struct SetApprovalForAllSvc<T: Erc404>(pub Arc<T>);
                    impl<
                        T: Erc404,
                    > tonic::server::UnaryService<super::SetApprovalForAllRequest>
                    for SetApprovalForAllSvc<T> {
                        type Response = super::SetApprovalForAllResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetApprovalForAllRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_approval_for_all(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetApprovalForAllSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/erc404.ERC404/IsApprovedForAll" => {
                    #[allow(non_camel_case_types)]
                    struct IsApprovedForAllSvc<T: Erc404>(pub Arc<T>);
                    impl<
                        T: Erc404,
                    > tonic::server::UnaryService<super::IsApprovedForAllRequest>
                    for IsApprovedForAllSvc<T> {
                        type Response = super::IsApprovedForAllResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::IsApprovedForAllRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).is_approved_for_all(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = IsApprovedForAllSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/erc404.ERC404/SafeTransferFrom" => {
                    #[allow(non_camel_case_types)]
                    struct SafeTransferFromSvc<T: Erc404>(pub Arc<T>);
                    impl<
                        T: Erc404,
                    > tonic::server::UnaryService<super::SafeTransferFromRequest>
                    for SafeTransferFromSvc<T> {
                        type Response = super::SafeTransferFromResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SafeTransferFromRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).safe_transfer_from(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SafeTransferFromSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/erc404.ERC404/SafeBatchTransferFrom" => {
                    #[allow(non_camel_case_types)]
                    struct SafeBatchTransferFromSvc<T: Erc404>(pub Arc<T>);
                    impl<
                        T: Erc404,
                    > tonic::server::UnaryService<super::SafeBatchTransferFromRequest>
                    for SafeBatchTransferFromSvc<T> {
                        type Response = super::SafeBatchTransferFromResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SafeBatchTransferFromRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).safe_batch_transfer_from(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SafeBatchTransferFromSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/erc404.ERC404/ERC1155BalanceOf" => {
                    #[allow(non_camel_case_types)]
                    struct ERC1155BalanceOfSvc<T: Erc404>(pub Arc<T>);
                    impl<
                        T: Erc404,
                    > tonic::server::UnaryService<super::Erc1155BalanceOfRequest>
                    for ERC1155BalanceOfSvc<T> {
                        type Response = super::Erc1155BalanceOfResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Erc1155BalanceOfRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).erc1155_balance_of(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ERC1155BalanceOfSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/erc404.ERC404/ERC20BalanceOf" => {
                    #[allow(non_camel_case_types)]
                    struct ERC20BalanceOfSvc<T: Erc404>(pub Arc<T>);
                    impl<
                        T: Erc404,
                    > tonic::server::UnaryService<super::Erc20BalanceOfRequest>
                    for ERC20BalanceOfSvc<T> {
                        type Response = super::Erc20BalanceOfResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Erc20BalanceOfRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).erc20_balance_of(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ERC20BalanceOfSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/erc404.ERC404/ERC1155TransferExempt" => {
                    #[allow(non_camel_case_types)]
                    struct ERC1155TransferExemptSvc<T: Erc404>(pub Arc<T>);
                    impl<
                        T: Erc404,
                    > tonic::server::UnaryService<super::Erc1155TransferExemptRequest>
                    for ERC1155TransferExemptSvc<T> {
                        type Response = super::Erc1155TransferExemptResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Erc1155TransferExemptRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).erc1155_transfer_exempt(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ERC1155TransferExemptSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Erc404> Clone for Erc404Server<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Erc404> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Erc404> tonic::server::NamedService for Erc404Server<T> {
        const NAME: &'static str = "erc404.ERC404";
    }
}
