-- Add migration script here
ALTER TABLE registered_clients 
    ADD UNIQUE(client_token);