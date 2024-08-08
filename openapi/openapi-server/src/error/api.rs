use {openapi_logger::warn, tonic::Status};

pub type Result<T> = std::result::Result<T, tonic::Status>;

pub fn into_anyhow(err: anyhow::Error) -> Status {
    let error = format!("{}", err);
    warn!("{error}");
    Status::new(tonic::Code::Aborted, error)
}
