-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "users"
(
    id                UUID PRIMARY KEY      DEFAULT uuid_generate_v4(),
    first_name        VARCHAR(80)  NOT NULL,
    last_name         VARCHAR(80)  NOT NULL,
    full_name         CHAR(160)    NOT NULL GENERATED ALWAYS AS (first_name || ' ' || last_name) STORED,
    email             VARCHAR(255) NOT NULL,
    username          VARCHAR(40)  NOT NULL,
    password          VARCHAR(118) NOT NULL,
    first_login_at    TIMESTAMP    NULL     DEFAULT NULL,
    last_login_at     TIMESTAMP    NULL     DEFAULT NULL,
    is_online         BOOLEAN      NOT NULL DEFAULT FALSE,
    is_email_verified BOOLEAN      NOT NULL DEFAULT FALSE,
    is_active         BOOLEAN      NOT NULL DEFAULT TRUE,
    is_deleted        BOOLEAN      NOT NULL DEFAULT FALSE,
    created_at        TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at        TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at        TIMESTAMP    NULL     DEFAULT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS "users_email_unique" ON "users" (email);
CREATE UNIQUE INDEX IF NOT EXISTS "users_username_unique" ON "users" (username);