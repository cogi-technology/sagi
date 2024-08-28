#![allow(clippy::clone_on_ref_ptr)]

mod generated;

pub mod erc20_service {
    include!("./generated/erc20.rs");
}

pub mod erc721_service {
    include!("./generated/erc721.rs");
}

pub mod erc404_service {
    include!("./generated/erc404.rs");
}

pub mod authtelegram_service {
    include!("./generated/authtelegram.rs");
}

pub mod zionauthorization_service {
    include!("./generated/zionauthorization.rs");
}

pub mod serviceszion_service {
    include!("./generated/serviceszion.rs");
}
