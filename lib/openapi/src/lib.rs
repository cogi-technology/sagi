pub mod erc20;
pub mod erc404;
pub mod erc721;

mod erc20_service {
    tonic::include_proto!("erc20");
}

mod erc721_service {
    tonic::include_proto!("erc721");
}

mod erc404_service {
    tonic::include_proto!("erc404");
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
