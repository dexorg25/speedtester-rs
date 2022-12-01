
CREATE TABLE IF NOT EXISTS registered_clients  (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    client_name VARCHAR(255) NOT NULL,
    client_token VARCHAR(255) UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS packet_loss_tests (
    ts      timestamptz PRIMARY KEY DEFAULT NOW(),
    client_id UUID NOT NULL REFERENCES registered_clients,
    test    jsonb NOT NULL
);
