    CREATE TABLE IF NOT EXISTS packet_loss_tests (
        ts      timestamptz PRIMARY KEY DEFAULT NOW(),
        client_id INET NOT NULL DEFAULT inet_client_addr(),
        test    jsonb NOT NULL)
