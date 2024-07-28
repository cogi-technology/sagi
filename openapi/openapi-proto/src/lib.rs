use prost::Message;

pub mod erc20_service {
    tonic::include_proto!("erc20");
}

pub mod erc721_service {
    tonic::include_proto!("erc721");
}

pub mod erc404_service {
    tonic::include_proto!("erc404");
}

pub mod authtelegram_service {
    tonic::include_proto!("authtelegram");
}

