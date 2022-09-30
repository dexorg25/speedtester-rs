
CREATE TABLE IF NOT EXISTS registered_clients  (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    client_name VARCHAR(255),
    client_token VARCHAR(255)
);

CREATE TABLE IF NOT EXISTS packet_loss_tests (
    ts      timestamptz PRIMARY KEY DEFAULT NOW(),
    client_id UUID REFERENCES registered_clients,
    test    jsonb NOT NULL
);
