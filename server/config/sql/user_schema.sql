CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),         -- Auto-incrementing primary key
    email VARCHAR(255) UNIQUE NOT NULL, -- Username, must be unique and not null
    first_name VARCHAR(255),
    last_name VARCHAR(255),
    full_name VARCHAR(255),
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL, -- Record creation time
    update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL  -- Record last update time
);

CREATE OR REPLACE FUNCTION update_user_time() 
RETURNS TRIGGER AS $$
BEGIN
    NEW.update_time = CURRENT_TIMESTAMP; -- Set update_time to the current timestamp
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_time
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE FUNCTION update_user_time();
