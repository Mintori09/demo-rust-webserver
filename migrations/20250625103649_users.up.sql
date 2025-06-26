-- Up migration
-- Tạo enum type cho vai trò người dùng
CREATE TYPE user_role AS ENUM ('admin', 'user');

-- Tạo extension để sinh UUID (chỉ khi chưa có)
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Tạo bảng users
CREATE TABLE "users" (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(100) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    verified BOOLEAN NOT NULL DEFAULT FALSE,
    password VARCHAR(100) NOT NULL,
    verification_token VARCHAR(255),
    token_expires_at TIMESTAMPTZ, 
    role user_role NOT NULL DEFAULT 'user',
    created_at TIMESTAMPTZ DEFAULT NOW(), 
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Tạo index cho cột email
CREATE INDEX user_email_idx ON users (email);
