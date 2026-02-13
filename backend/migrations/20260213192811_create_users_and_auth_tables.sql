CREATE TYPE user_role AS ENUM ('student', 'faculty', 'authority', 'admin');
CREATE TYPE user_status AS ENUM ('active', 'inactive', 'suspended', 'pending_setup');
CREATE TYPE auth_provider AS ENUM ('google', 'email');

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    email_verified BOOLEAN DEFAULT FALSE,
    email_verified_at TIMESTAMPTZ,
    
    google_id VARCHAR(255) UNIQUE,
    auth_provider auth_provider NOT NULL,
    password_hash TEXT,
    
    role user_role NOT NULL,
    status user_status DEFAULT 'pending_setup',
    
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    display_name VARCHAR(200),
    profile_picture TEXT,
    bio TEXT,
    
    roll_number VARCHAR(50) UNIQUE,
    batch_year INTEGER,
    program VARCHAR(50),
    department VARCHAR(100),
    current_semester INTEGER,
    
    employee_id VARCHAR(50) UNIQUE,
    designation VARCHAR(100),
    office_location VARCHAR(200),
    
    preferences JSONB DEFAULT '{}',
    metadata JSONB DEFAULT '{}',
    
    terms_accepted_at TIMESTAMPTZ,
    privacy_policy_accepted_at TIMESTAMPTZ,
    
    last_login_at TIMESTAMPTZ,
    last_activity_at TIMESTAMPTZ,
    profile_completed BOOLEAN DEFAULT FALSE,
    
    deleted_at TIMESTAMPTZ,
    deleted_by UUID REFERENCES users(id) ON DELETE SET NULL,
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    CONSTRAINT check_student_has_roll CHECK (
        (role = 'student' AND roll_number IS NOT NULL AND deleted_at IS NULL) OR 
        role != 'student' OR 
        deleted_at IS NOT NULL
    ),
    CONSTRAINT check_staff_has_employee_id CHECK (
        (role IN ('faculty', 'authority', 'admin') AND employee_id IS NOT NULL AND deleted_at IS NULL) OR 
        role = 'student' OR 
        deleted_at IS NOT NULL
    ),
    CONSTRAINT check_institute_email CHECK (
        email ~* '@(students\.)?iitmandi\.ac\.in$'
    )
);

CREATE TABLE email_verifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL,
    otp_code VARCHAR(6) NOT NULL,
    otp_hash VARCHAR(255) NOT NULL,
    attempts INTEGER DEFAULT 0,
    verified BOOLEAN DEFAULT FALSE,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    
    CONSTRAINT check_max_attempts CHECK (attempts <= 3)
);

CREATE TABLE user_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    session_token VARCHAR(255) UNIQUE NOT NULL,
    user_agent TEXT,
    ip_address INET,
    device_info JSONB,
    location_info JSONB,
    expires_at TIMESTAMPTZ NOT NULL,
    last_activity_at TIMESTAMPTZ DEFAULT NOW(),
    revoked BOOLEAN DEFAULT FALSE,
    revoked_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    action VARCHAR(100) NOT NULL,
    ip_address INET,
    user_agent TEXT,
    metadata JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE turnstile_verifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255),
    turnstile_token TEXT NOT NULL,
    verification_response JSONB,
    success BOOLEAN NOT NULL,
    ip_address INET,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
 WHERE deleted_at IS NULL;
CREATE INDEX idx_users_deleted_at ON users(deleted_at);
CREATE UNIQUE INDEX idx_users_roll_number ON users(roll_number) WHERE roll_number IS NOT NULL AND deleted_at IS NULL;
CREATE UNIQUE INDEX idx_users_employee_id ON users(employee_id) WHERE employee_id IS NOT NULL AND deleted_at IS NULL;
CREATE INDEX idx_users_department ON users(department) WHERE deleted_at IS NULL;
CREATE INDEX idx_users_batch_year ON users(batch_year) WHERE deleted_at IS
CREATE INDEX idx_users_role ON users(role);
CREATE INDEX idx_users_status ON users(status);
CREATE UNIQUE INDEX idx_users_roll_number ON users(roll_number) WHERE roll_number IS NOT NULL;
CREATE UNIQUE INDEX idx_users_employee_id ON users(employee_id) WHERE employee_id IS NOT NULL;

CREATE INDEX idx_email_verifications_email ON email_verifications(email);
CREATE INDEX idx_email_verifications_expires ON email_verifications(expires_at);

CREATE INDEX idx_user_sessions_revoked ON user_sessions(revoked) WHERE revoked = FALSE;
CREATE INDEX idx_user_sessions_user_id ON user_sessions(user_id);
CREATE INDEX idx_user_sessions_token ON user_sessions(session_token);
CREATE INDEX idx_user_sessions_expires ON user_sessions(expires_at);

CREATE INDEX idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX idx_audit_logs_action ON audit_logs(action);
CREATE INDEX idx_audit_logs_created_at ON audit_logs(created_at DESC);

CREATE INDEX idx_turnstile_email ON turnstile_verifications(email);
CREATE INDEX idx_turnstile_created_at ON turnstile_verifications(created_at DESC);

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW

CREATE OR REPLACE FUNCTION update_user_last_activity()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE users SET last_activity_at = NOW() WHERE id = NEW.user_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_user_activity_on_session
    AFTER INSERT OR UPDATE OF last_activity_at ON user_sessions
    FOR EACH ROW
    EXECUTE FUNCTION update_user_last_activity();
    EXECUTE FUNCTION update_updated_at_column();
