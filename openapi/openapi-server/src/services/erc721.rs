use {
    super::Result,
    crate::proto::erc721_service::{erc721_server::Erc721, *},
    tonic::{Request, Response},
};

pub struct Erc721Service {
    // erc721_etherman: Arc<Erc721_Etherman>
}

#[tonic::async_trait]
impl Erc721 for Erc721Service {
    async fn deploy(&self, req: Request<DeployRequest>) -> Result<Response<DeployResponse>> {
        todo!()
    }

    async fn balance_of(
        &self,
        req: Request<BalanceOfRequest>,
    ) -> Result<Response<BalanceOfResponse>> {
        todo!()
    }

    async fn owner_of(&self, req: Request<OwnerOfRequest>) -> Result<Response<OwnerOfResponse>> {
        todo!()
    }

    async fn safe_transfer_from(
        &self,
        req: Request<SafeTransferFromRequest>,
    ) -> Result<Response<SafeTransferFromResponse>> {
        todo!()
    }

    async fn transfer_from(
        &self,
        req: Request<TransferFromRequest>,
    ) -> Result<Response<TransferFromResponse>> {
        todo!()
    }

    async fn approve(&self, req: Request<ApproveRequest>) -> Result<Response<ApproveResponse>> {
        todo!()
    }

    async fn get_approved(
        &self,
        req: Request<GetApprovedRequest>,
    ) -> Result<Response<GetApprovedResponse>> {
        todo!()
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
}
