use {
    actix_prost_build::{ActixGenerator, GeneratorList},
    prost_build::{Config, ServiceGenerator},
    std::path::Path,
};

// custom function to include custom generator
fn compile(
    protos: &[impl AsRef<Path>],
    includes: &[impl AsRef<Path>],
    generator: Box<dyn ServiceGenerator>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::new();
    config
        .service_generator(generator)
        .compile_well_known_types()
        .protoc_arg("--experimental_allow_proto3_optional")
        .protoc_arg("--openapiv2_out=swagger")
        .protoc_arg("--openapiv2_opt")
        .protoc_arg("grpc_api_configuration=proto/api_config_http.yaml,output_format=yaml,allow_merge=true,merge_file_name=sagi-openapi,json_names_for_fields=false")
        .bytes(["."])
        .type_attribute(".", "#[actix_prost_macros::serde(rename_all=\"snake_case\")]")
        .field_attribute(".erc20.DeployRequest.initial_supply", "#[serde(rename=\"initial_supply\")]");

    config.out_dir("./src/generated/");
    config.compile_protos(protos, includes)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // We need to rebuild proto lib only if any of proto definitions
    // (or corresponding http mapping) has been changed.
    println!("cargo:rerun-if-changed=proto/");

    std::fs::create_dir_all("./swagger").unwrap();
    let gens = Box::new(GeneratorList::new(vec![
        tonic_build::configure().service_generator(),
        Box::new(ActixGenerator::new("proto/api_config_http.yaml").unwrap()),
    ]));
    compile(
        &[
            "proto/zion_authorization.proto",
            "proto/auth_telegram.proto",
            "proto/erc20.proto",
            "proto/erc404.proto",
            "proto/erc721.proto",
        ],
        &["."],
        gens,
    )?;
    Ok(())
}
