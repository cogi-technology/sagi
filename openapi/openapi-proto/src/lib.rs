#[allow(clippy::clone_on_ref_ptr)]
mod generated;

pub mod erc20_service {
    include!("./generated/erc20.rs");
    // tonic::include_proto!("erc20");
}

pub mod erc721_service {
    include!("./generated/erc721.rs");
    // tonic::include_proto!("erc721");
}

pub mod erc404_service {
    include!("./generated/erc404.rs");
    // tonic::include_proto!("erc404");
}

pub mod authtelegram_service {
    include!("./generated/authtelegram.rs");
    // tonic::include_proto!("authtelegram");
}

pub mod zionauthorization_service {
    include!("./generated/zionauthorization.rs");
    // tonic::include_proto!("authtelegram");
}
