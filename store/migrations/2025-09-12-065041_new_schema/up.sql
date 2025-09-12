-- Your SQL goes here

-- First, drop the status column that depends on the enum
ALTER TABLE website_ticks DROP COLUMN status;

-- Now we can drop the enum type
DROP TYPE website_status;

-- Create the new enum with uppercase values
CREATE TYPE website_status AS ENUM ('UP', 'DOWN', 'UNKNOWN');

-- Add back the status column with the new enum type
ALTER TABLE website_ticks ADD COLUMN status website_status NOT NULL DEFAULT 'UNKNOWN';

-- Update user table - remove email and name columns
ALTER TABLE users DROP COLUMN IF EXISTS email;
ALTER TABLE users DROP COLUMN IF EXISTS name;

-- Add username and password columns
ALTER TABLE users ADD COLUMN username TEXT NOT NULL DEFAULT '';
ALTER TABLE users ADD COLUMN password TEXT NOT NULL DEFAULT '';
