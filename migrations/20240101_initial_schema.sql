-- Create extension for UUID support
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create user roles enum
CREATE TYPE user_role AS ENUM ('USER', 'RESEARCHER', 'ADMIN');

-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) NOT NULL UNIQUE,
    username VARCHAR(50) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    full_name VARCHAR(100),
    bio TEXT,
    role user_role NOT NULL DEFAULT 'USER',
    wallet_address VARCHAR(42),
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);

-- Create research status enum
CREATE TYPE research_status AS ENUM ('DRAFT', 'PENDING', 'ACTIVE', 'COMPLETED', 'CANCELLED');

-- Create research type enum
CREATE TYPE research_type AS ENUM ('CLINICAL', 'OBSERVATIONAL', 'SURVEY', 'EXPERIMENTAL');

-- Create researches table
CREATE TABLE IF NOT EXISTS researches (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    methodology TEXT NOT NULL,
    goal TEXT NOT NULL,
    status research_status NOT NULL DEFAULT 'DRAFT',
    research_type research_type NOT NULL,
    required_participants INTEGER NOT NULL DEFAULT 0,
    current_participants INTEGER NOT NULL DEFAULT 0,
    funding_goal BIGINT NOT NULL DEFAULT 0,
    current_funding BIGINT NOT NULL DEFAULT 0,
    start_date TIMESTAMPTZ,
    end_date TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_researches_user_id ON researches(user_id);
CREATE INDEX idx_researches_status ON researches(status);
CREATE INDEX idx_researches_type ON researches(research_type);

-- Create research participations table
CREATE TABLE IF NOT EXISTS research_participations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    research_id UUID NOT NULL REFERENCES researches(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    joined_at TIMESTAMPTZ NOT NULL,
    UNIQUE(research_id, user_id)
);

CREATE INDEX idx_participations_research_id ON research_participations(research_id);
CREATE INDEX idx_participations_user_id ON research_participations(user_id);

-- Create funding status enum
CREATE TYPE funding_status AS ENUM ('PENDING', 'COMPLETED', 'FAILED', 'REFUNDED');

-- Create fundings table
CREATE TABLE IF NOT EXISTS fundings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    research_id UUID NOT NULL REFERENCES researches(id) ON DELETE CASCADE,
    amount BIGINT NOT NULL,
    transaction_hash VARCHAR(66) NOT NULL,
    status funding_status NOT NULL DEFAULT 'PENDING',
    message TEXT,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_fundings_user_id ON fundings(user_id);
CREATE INDEX idx_fundings_research_id ON fundings(research_id);
CREATE INDEX idx_fundings_status ON fundings(status);

-- Create funding transactions table
CREATE TABLE IF NOT EXISTS funding_transactions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    funding_id UUID NOT NULL REFERENCES fundings(id) ON DELETE CASCADE,
    transaction_hash VARCHAR(66) NOT NULL UNIQUE,
    status VARCHAR(20) NOT NULL,
    verified_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_transactions_funding_id ON funding_transactions(funding_id);
CREATE INDEX idx_transactions_hash ON funding_transactions(transaction_hash);
CREATE INDEX idx_transactions_status ON funding_transactions(status);

-- Create corneal data records table
CREATE TABLE IF NOT EXISTS corneal_data (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    research_id UUID REFERENCES researches(id) ON DELETE SET NULL,
    image_url VARCHAR(255) NOT NULL,
    metadata JSONB NOT NULL,
    diagnosis TEXT,
    ipfs_hash VARCHAR(64),
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_corneal_data_user_id ON corneal_data(user_id);
CREATE INDEX idx_corneal_data_research_id ON corneal_data(research_id);

-- Create statistics views
CREATE VIEW research_stats AS
SELECT 
    r.id as research_id,
    r.title,
    r.user_id as researcher_id,
    u.username as researcher_name,
    r.status,
    r.research_type,
    r.required_participants,
    r.current_participants,
    r.funding_goal,
    r.current_funding,
    (SELECT COUNT(*) FROM corneal_data WHERE research_id = r.id) as data_count,
    r.created_at
FROM researches r
JOIN users u ON r.user_id = u.id;

CREATE VIEW funding_stats AS
SELECT
    r.id as research_id,
    r.title,
    COUNT(f.id) as funding_count,
    SUM(f.amount) as total_funding,
    COUNT(DISTINCT f.user_id) as unique_backers
FROM researches r
LEFT JOIN fundings f ON r.id = f.research_id AND f.status = 'COMPLETED'
GROUP BY r.id, r.title;

-- Create initial admin user (password: admin_password)
INSERT INTO users (
    id, email, username, password_hash, full_name, role, created_at, updated_at
) VALUES (
    uuid_generate_v4(), 
    'admin@corneadesci.org', 
    'admin', 
    '$2b$10$X7V1R8JMMnZ4AlPsJP/9R.m/BV5V8P4XnwI0tRB1kwwR1hyc/XJdm', 
    'System Administrator', 
    'ADMIN', 
    NOW(), 
    NOW()
); 