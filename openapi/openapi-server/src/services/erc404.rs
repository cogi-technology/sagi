use {
    super::Result,
    crate::proto::erc404_service::{erc404_server::Erc404, *},
    std::sync::Arc,
    tonic::{Request, Response},
};

pub struct Erc404Service {
    // erc404_etherman: Arc<Erc404_Etherman>
}

#[tonic::async_trait]
impl Erc404 for Erc404Service {
    async fn deploy(&self, req: Request<DeployRequest>) -> Result<Response<DeployResponse>> {
        todo!();
    }

    async fn total_supply(
        &self,
        req: Request<TotalSupplyRequest>,
    ) -> Result<Response<TotalSupplyResponse>> {
        todo!();
    }

    async fn approve(&self, req: Request<ApproveRequest>) -> Result<Response<ApproveResponse>> {
        todo!();
    }

    async fn balance_of(
        &self,
        req: Request<BalanceOfRequest>,
    ) -> Result<Response<BalanceOfResponse>> {
        todo!();
    }

    async fn balance_of_batch(
        &self,
        req: Request<BalanceOfBatchRequest>,
    ) -> Result<Response<BalanceOfBatchResponse>> {
        todo!();
    }

    async fn allowance(
        &self,
        req: Request<AllowanceRequest>,
    ) -> Result<Response<AllowanceResponse>> {
        todo!();
    }

    async fn transfer(&self, req: Request<TransferRequest>) -> Result<Response<TransferResponse>> {
        todo!();
    }

    async fn transfer_from(
        &self,
        req: Request<TransferFromRequest>,
    ) -> Result<Response<TransferFromResponse>> {
        todo!();
    }

    async fn set_approval_for_all(
        &self,
        req: Request<SetApprovalForAllRequest>,
    ) -> Result<Response<SetApprovalForAllResponse>> {
        todo!()
    }

    async fn is_approved_for_all(
        &self,
        req: Request<IsApprovedForAllRequest>,
    ) -> Result<Response<IsApprovedForAllResponse>> {
        todo!()
    }

    async fn safe_transfer_from(
        &self,
        req: Request<SafeTransferFromRequest>,
    ) -> Result<Response<SafeTransferFromResponse>> {
        todo!()
    }

    async fn safe_batch_transfer_from(
        &self,
        req: Request<SafeBatchTransferFromRequest>,
    ) -> Result<Response<SafeBatchTransferFromResponse>> {
        todo!()
    }

    async fn erc1155_balance_of(
        &self,
        req: Request<Erc1155BalanceOfRequest>,
    ) -> Result<Response<Erc1155BalanceOfResponse>> {
        todo!()
    }

    async fn erc20_balance_of(
        &self,
        req: Request<Erc20BalanceOfRequest>,
    ) -> Result<Response<Erc20BalanceOfResponse>> {
        todo!()
    }

    async fn erc1155_transfer_exempt(
        &self,
        req: Request<Erc1155TransferExemptRequest>,
    ) -> Result<Response<Erc1155TransferExemptResponse>> {
        todo!()
    }
}
