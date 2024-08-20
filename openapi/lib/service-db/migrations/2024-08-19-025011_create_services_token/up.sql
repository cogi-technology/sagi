-- Your SQL goes here
CREATE TABLE service_token(
    id VARCHAR(255) PRIMARY KEY,
    client_id VARCHAR(255) NOT NULL,
    address VARCHAR(255) NOT NULL,
    to_transfer VARCHAR(255) NOT NULL,
    namespace VARCHAR(255) NOT NULL,
    start_block_number INT NOT NULL,
    created_by VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);