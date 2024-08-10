-- Your SQL goes here
CREATE TABLE bills (
    id VARCHAR(255) NOT NULL PRIMARY KEY,
    customer VARCHAR(255) NOT NULL,
    paid_amount VARCHAR(255) NOT NULL,
    rewarded_amount VARCHAR(255),
    paid_txhash VARCHAR(255) NOT NULL,
    rewarded_txhash VARCHAR(255),
    paid_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    rewarded_at TIMESTAMP,
    status SMALLINT NOT NULL
);