use crate::types::otp::{ZkOTPInput, ZkProof};
// use fastcrypto_zkp::bn254::poseidon::

// pub async fn generate_zk_otp_input(
//     otp: String,
//     time: u64,
//     hashes: Vec<u128>,
//     layer: usize,
// ) -> Result<ZkOTPInput, Box<dyn std::error::Error>> {
//     let poseidon = fastcrypto_zkp::bn254::poseidon::poseidon_bytes()

//     let mut current_node = poseidon.hash(&[time as u128, otp.parse::<u128>()?]);

//     if !hashes.contains(&current_node) {
//         return Err("Invalid OTP".into());
//     }

//     let mut path_elements = Vec::new();
//     let mut path_index = Vec::new();

//     for _ in 0..layer {
//         let index = hashes.iter().position(|&r| r == current_node).unwrap();
//         if index % 2 == 0 {
//             path_index.push(0);
//             let current_index = index + 1;
//             path_elements.push(hashes[current_index]);
//             current_node = poseidon.hash(&[hashes[index], hashes[current_index]]);
//         } else {
//             path_index.push(1);
//             let current_index = index - 1;
//             path_elements.push(hashes[current_index]);
//             current_node = poseidon.hash(&[hashes[current_index], hashes[current_index + 1]]);
//         }
//     }

//     Ok(ZkOTPInput {
//         time,
//         otp,
//         path_elements,
//         path_index,
//     })
// }

pub async fn generate_zk_proof(
    input: ZkOTPInput,
    path_wasm: &str,
    path_zkey: &str,
) -> Result<ZkProof, Box<dyn std::error::Error>> {
    let (proof, public_signals) = ark_groth16::Groth16 create_proof(&input, path_wasm, path_zkey).await?;
    let calldata = export_solidity_call_data(&proof, &public_signals)?;

    let argv: Vec<String> = calldata
        .replace(&['[', ']', '"', ' '][..], "")
        .split(',')
        .map(|x| x.to_string())
        .collect();

    let a = vec![argv[0].clone(), argv[1].clone()];
    let b = vec![
        vec![argv[2].clone(), argv[3].clone()],
        vec![argv[4].clone(), argv[5].clone()],
    ];
    let c = vec![argv[6].clone(), argv[7].clone()];
    let pub_signals = argv[8..].to_vec();

    Ok(ZkProof {
        p_a: a,
        p_b: b,
        p_c: c,
        pub_signals,
    })
}