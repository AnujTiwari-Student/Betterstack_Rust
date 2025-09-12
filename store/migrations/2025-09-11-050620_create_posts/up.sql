-- Your SQL goes here

-- CreateEnum (using snake_case for values)
CREATE TYPE website_status AS ENUM ('up', 'down', 'unknown');

-- CreateTable for User
CREATE TABLE users (
    id TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    name TEXT,
    created_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    CONSTRAINT users_pkey PRIMARY KEY (id)
);

-- CreateTable
CREATE TABLE websites (
    id TEXT NOT NULL,
    url TEXT NOT NULL,
    user_id TEXT NOT NULL,
    created_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    CONSTRAINT websites_pkey PRIMARY KEY (id)
);

-- CreateTable
CREATE TABLE regions (
    id TEXT NOT NULL,
    name TEXT NOT NULL,
    
    CONSTRAINT regions_pkey PRIMARY KEY (id)
);

-- CreateTable
CREATE TABLE website_ticks (
    id TEXT NOT NULL,
    response_time_ms INTEGER NOT NULL,
    status website_status NOT NULL,
    region_id TEXT NOT NULL,
    website_id TEXT NOT NULL,
    created_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    CONSTRAINT website_ticks_pkey PRIMARY KEY (id)
);

-- AddForeignKey constraints
ALTER TABLE websites ADD CONSTRAINT websites_user_id_fkey 
    FOREIGN KEY (user_id) REFERENCES users(id) 
    ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE website_ticks ADD CONSTRAINT website_ticks_region_id_fkey 
    FOREIGN KEY (region_id) REFERENCES regions(id) 
    ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE website_ticks ADD CONSTRAINT website_ticks_website_id_fkey 
    FOREIGN KEY (website_id) REFERENCES websites(id) 
    ON DELETE RESTRICT ON UPDATE CASCADE;
