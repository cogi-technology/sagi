use crate::types::jwt::ProofPoints;

pub struct ProtoProofPoints {
    pub protocol: String,
    pub pi_a: Vec<String>,
    pub pi_b: Vec<StringArray>,
    pub pi_c: Vec<String>,
}

pub struct StringArray {
    pub values: Vec<String>,
}

pub fn sdk_proofpoint_from(value: ProtoProofPoints) -> ProofPoints {
    let pi_b = value
        .pi_b
        .into_iter()
        .map(|item| item.values.into_iter().map(|v| v).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

    ProofPoints {
        pi_a: value.pi_a,
        pi_b,
        pi_c: value.pi_c,
        protocol: Some(value.protocol),
    }
}
