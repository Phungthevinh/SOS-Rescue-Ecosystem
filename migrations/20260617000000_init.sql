-- Setup initial database schemas

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    phone_number VARCHAR(20) UNIQUE NOT NULL,
    full_name VARCHAR(100),
    avatar_url VARCHAR(255),
    trust_score INTEGER DEFAULT 30,
    is_verified BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Device fingerprints for anti-abuse
CREATE TABLE IF NOT EXISTS device_fingerprints (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    device_hash VARCHAR(255) NOT NULL,
    platform VARCHAR(20),
    is_emulator BOOLEAN DEFAULT FALSE,
    first_seen TIMESTAMPTZ DEFAULT NOW(),
    last_seen TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, device_hash)
);

-- Emergency Contacts
CREATE TABLE IF NOT EXISTS emergency_contacts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    phone_number VARCHAR(20) NOT NULL,
    name VARCHAR(100) NOT NULL,
    relationship VARCHAR(50),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- SOS Events
CREATE TABLE IF NOT EXISTS sos_events (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    status VARCHAR(20) NOT NULL DEFAULT 'active', -- 'active', 'resolved', 'cancelled'
    start_lat DOUBLE PRECISION NOT NULL,
    start_lng DOUBLE PRECISION NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    resolved_at TIMESTAMPTZ
);

-- SOS Event Logs (tracking locations during event)
CREATE TABLE IF NOT EXISTS sos_event_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    event_id UUID REFERENCES sos_events(id) ON DELETE CASCADE,
    lat DOUBLE PRECISION NOT NULL,
    lng DOUBLE PRECISION NOT NULL,
    recorded_at TIMESTAMPTZ DEFAULT NOW()
);
