use {
    openapi_proto::zionauthorization_service::{ProofPoints, StringArray},
    zion_aa::types::jwt::ProofPoints as SdkProofPoints,
};

#[allow(dead_code)]
pub fn sdk_proofpoint_from(value: ProofPoints) -> SdkProofPoints {
    let pi_b = value
        .pi_b
        .into_iter()
        .map(|item| item.values.into_iter().collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

    SdkProofPoints {
        pi_a: value.pi_a,
        pi_b,
        pi_c: value.pi_c,
        protocol: Some(value.protocol),
    }
}

pub fn proto_proofpoint_from(value: SdkProofPoints) -> ProofPoints {
    let pi_b = value
        .pi_b
        .into_iter()
        .map(|item| StringArray { values: item })
        .collect::<Vec<StringArray>>();

    ProofPoints {
        protocol: value.protocol.unwrap_or_default(),
        pi_a: value.pi_a,
        pi_b,
        pi_c: value.pi_c,
    }
}
