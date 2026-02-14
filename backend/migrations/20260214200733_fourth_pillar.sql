-- Enums
CREATE TYPE opportunity_type AS ENUM ('internship', 'research', 'project', 'job');
CREATE TYPE application_status AS ENUM ('submitted', 'under_review', 'shortlisted', 'accepted', 'rejected');
CREATE TYPE task_priority AS ENUM ('low', 'medium', 'high', 'urgent');
CREATE TYPE task_status AS ENUM ('pending', 'in_progress', 'completed', 'archived');

-- 1. OPPORTUNITIES (The Professor's Call)
CREATE TABLE opportunities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    posted_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    
    opportunity_type opportunity_type NOT NULL DEFAULT 'research',
    department VARCHAR(100) NOT NULL,  -- ADDED: Crucial for "Smart Filtering"
    required_skills TEXT[],
    duration VARCHAR(100),
    stipend VARCHAR(100),
    location VARCHAR(100),
    
    application_deadline TIMESTAMPTZ,
    is_active BOOLEAN DEFAULT TRUE,
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- 2. APPLICATIONS
CREATE TABLE applications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    opportunity_id UUID NOT NULL REFERENCES opportunities(id) ON DELETE CASCADE,
    student_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    resume_url TEXT,
    cover_letter TEXT,
    portfolio_url TEXT,            -- ADDED: Requirement mentioned "Resume/Portfolio upload"
    status application_status NOT NULL DEFAULT 'submitted',
    
    faculty_remarks TEXT,          -- Internal notes for the professor
    
    applied_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    UNIQUE(opportunity_id, student_id)
);

-- 3. APPLICATION MESSAGES (Direct Messaging System)
-- ADDED: Handles "Direct messaging system with applicants"
CREATE TABLE application_messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    application_id UUID NOT NULL REFERENCES applications(id) ON DELETE CASCADE,
    sender_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    message TEXT NOT NULL,
    is_read BOOLEAN DEFAULT FALSE,
    
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- 4. SAVED OPPORTUNITIES
CREATE TABLE saved_opportunities (
    student_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    opportunity_id UUID NOT NULL REFERENCES opportunities(id) ON DELETE CASCADE,
    saved_at TIMESTAMPTZ DEFAULT NOW(),
    
    PRIMARY KEY (student_id, opportunity_id)
);

-- 5. PERSONAL TASKS (The Scholar's Ledger)
CREATE TABLE personal_tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    title VARCHAR(255) NOT NULL,
    description TEXT,
    
    status task_status NOT NULL DEFAULT 'pending',
    priority task_priority NOT NULL DEFAULT 'medium',
    progress_percentage INTEGER DEFAULT 0 CHECK (progress_percentage BETWEEN 0 AND 100), -- ADDED: For "Progress Visualization"
    
    due_date TIMESTAMPTZ,
    tags TEXT[], 
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_opportunities_type ON opportunities(opportunity_type);
CREATE INDEX idx_opportunities_dept ON opportunities(department);
CREATE INDEX idx_opportunities_active ON opportunities(is_active);
CREATE INDEX idx_applications_student ON applications(student_id);
CREATE INDEX idx_applications_opportunity ON applications(opportunity_id);
CREATE INDEX idx_messages_application ON application_messages(application_id);
CREATE INDEX idx_tasks_user ON personal_tasks(user_id);
CREATE INDEX idx_tasks_due_date ON personal_tasks(due_date);

-- Triggers
CREATE TRIGGER opportunities_updated_at BEFORE UPDATE ON opportunities
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();

CREATE TRIGGER applications_updated_at BEFORE UPDATE ON applications
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();

CREATE TRIGGER tasks_updated_at BEFORE UPDATE ON personal_tasks
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();
