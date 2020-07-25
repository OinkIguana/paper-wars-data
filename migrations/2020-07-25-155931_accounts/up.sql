CREATE EXTENSION IF NOT EXISTS "pgcrypto";
CREATE EXTENSION IF NOT EXISTS "citext";

CREATE TABLE accounts (
    id         UUID                     PRIMARY KEY DEFAULT gen_random_uuid(),
    name       CITEXT                   UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);
COMMENT ON TABLE accounts             IS 'An account encapsulates the data required for user authentication.';
COMMENT ON COLUMN accounts.id         IS 'Unique ID for an account.';
COMMENT ON COLUMN accounts.name       IS 'The name associated with this account (username).';
COMMENT ON COLUMN accounts.created_at IS 'The date and time at which the account was created.';

CREATE TABLE emails (
    address         CITEXT                   PRIMARY KEY,
    account_id      UUID                     NOT NULL REFERENCES accounts ON DELETE CASCADE,
    verified_at     TIMESTAMP WITH TIME ZONE,
    protected_until TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now() + INTERVAL '3 days',
    created_at      TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    UNIQUE (address, account_id)
);
COMMENT ON TABLE emails              IS 'Emails which are associated with accounts. Accounts may have multiple associated emails, any of which may have been verified.';
COMMENT ON COLUMN emails.address     IS 'The actual email address.';
COMMENT ON COLUMN emails.account_id  IS 'The account which this email is associated with.';
COMMENT ON COLUMN emails.verified_at IS 'The date and time at which the email was verified by the user. A NULL value indicates that the email has not yet been verified.';
COMMENT ON COLUMN emails.created_at  IS 'The date and time at which this email was listed.';

CREATE FUNCTION emails_remove_expired_procedure() RETURNS TRIGGER
    LANGUAGE plpgsql
    AS $$
BEGIN
    DELETE FROM emails
          WHERE verified_at IS NULL
            AND protected_until < NOW()
            AND address = NEW.address;
    RETURN NEW;
END;
$$;

CREATE TRIGGER emails_remove_expired 
    BEFORE INSERT 
    ON emails
    FOR EACH ROW
    EXECUTE PROCEDURE emails_remove_expired_procedure();
COMMENT ON TRIGGER emails_remove_expired ON emails IS 'Removes expired emails (not-verified or protected) when a new insertion would be replacing that email.';

-- Account security

CREATE TABLE logins (
    account_id     UUID                     PRIMARY KEY REFERENCES accounts ON DELETE CASCADE,
    email_address  CITEXT                   UNIQUE NOT NULL REFERENCES emails,
    password       CHAR(60)                 NOT NULL,
    disabled_until TIMESTAMP WITH TIME ZONE,
    FOREIGN KEY (email_address, account_id) REFERENCES emails (address, account_id)
);
COMMENT ON TABLE logins                 IS 'Sets the credentials which allow the user to log in.';
COMMENT ON COLUMN logins.account_id     IS 'The account that can be logged in to.';
COMMENT ON COLUMN logins.email_address  IS 'The email address that may be used to log in to the account. This is the account''s primary email, which should be the one contacted for security purposes.';
COMMENT ON COLUMN logins.password       IS 'The encrypted (bcrypt) password to the account.';
COMMENT ON COLUMN logins.disabled_until IS 'The time at which this login stops being disabled. A NULL value indicates that this login has not been disabled.';

CREATE TABLE password_resets (
    id          UUID                     PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id  UUID                     NOT NULL REFERENCES accounts ON DELETE CASCADE,
    created_at  TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    valid_until TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now() + INTERVAL '1 day',
    consumed    BOOLEAN                  NOT NULL DEFAULT FALSE
);
COMMENT ON TABLE password_resets              IS 'A record of all password reset requests.';
COMMENT ON COLUMN password_resets.id          IS 'A random ID used as part of the password reset process.';
COMMENT ON COLUMN password_resets.account_id  IS 'The ID of the accout for which the password reset is being issued.';
COMMENT ON COLUMN password_resets.created_at  IS 'The date and time at which the reset was requested.';
COMMENT ON COLUMN password_resets.valid_until IS 'The date and time at which the reset request will expire if not consumed.';
COMMENT ON COLUMN password_resets.consumed    IS 'A flag indicating that this reset request has already been used.';
