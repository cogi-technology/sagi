pub mod contract_wallet;
pub mod jwt;
pub mod key;
pub mod otp;
pub mod user;
pub mod user_operation;
pub mod request;

#[cfg(test)]
mod tests {
    use ethers::types::U256;

    #[test]
    fn test_normal_use_case() {
        let a = U256::from_dec_str(
            "123123123123123123134213412341234123412343425345345134514512342341",
        )
        .unwrap();
        let mut nstr = format!("{:x}", a);
        while nstr.len() < 64 {
            nstr = format!("0{}", nstr);
        }
        format!("\"0x{}\"", nstr);

        println!("{}", a)
    }
}
