-- Create enums for grievance system
CREATE TYPE grievance_category AS ENUM ('infrastructure', 'academics', 'hostel', 'food', 'other');
CREATE TYPE grievance_priority AS ENUM ('low', 'medium', 'high', 'urgent');
CREATE TYPE grievance_status AS ENUM ('submitted', 'under_review', 'in_progress', 'resolved', 'closed');

-- Main grievances table
CREATE TABLE grievances (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Submitter information (nullable for anonymous submissions)
    submitted_by UUID REFERENCES users(id) ON DELETE SET NULL,
    is_anonymous BOOLEAN DEFAULT FALSE,
    anonymous_identifier VARCHAR(100), -- Generated identifier for anonymous users to track their submissions
    
    -- Grievance details
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    category grievance_category NOT NULL,
    priority grievance_priority NOT NULL DEFAULT 'medium',
    status grievance_status NOT NULL DEFAULT 'submitted',
    
    -- Location information
    location_type VARCHAR(100), -- e.g., "Hostel Block", "Academic Building", "Mess"
    location_details TEXT, -- Specific location description
    
    -- Media attachments
    photo_urls TEXT[], -- Array of Cloudinary URLs
    
    -- Assignment and handling
    assigned_to UUID REFERENCES users(id) ON DELETE SET NULL, -- Authority/Admin assigned to handle this
    assigned_department VARCHAR(100),
    
    -- Resolution
    resolution_notes TEXT,
    resolved_at TIMESTAMPTZ,
    resolved_by UUID REFERENCES users(id) ON DELETE SET NULL,
    
    -- Metadata
    view_count INTEGER DEFAULT 0,
    upvote_count INTEGER DEFAULT 0,
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Timeline/status history table
CREATE TABLE grievance_status_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    grievance_id UUID NOT NULL REFERENCES grievances(id) ON DELETE CASCADE,
    
    old_status grievance_status,
    new_status grievance_status NOT NULL,
    
    remarks TEXT,
    updated_by UUID REFERENCES users(id) ON DELETE SET NULL,
    updated_by_role user_role,
    
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Comments/remarks on grievances
CREATE TABLE grievance_comments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    grievance_id UUID NOT NULL REFERENCES grievances(id) ON DELETE CASCADE,
    
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    comment TEXT NOT NULL,
    is_internal BOOLEAN DEFAULT FALSE, -- Internal notes visible only to authorities/admins
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Upvotes/support for grievances
CREATE TABLE grievance_upvotes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    grievance_id UUID NOT NULL REFERENCES grievances(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    
    UNIQUE(grievance_id, user_id)
);

-- Department assignment table for tracking responsibilities
CREATE TABLE departments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) UNIQUE NOT NULL,
    description TEXT,
    head_user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX idx_grievances_submitted_by ON grievances(submitted_by);
CREATE INDEX idx_grievances_assigned_to ON grievances(assigned_to);
CREATE INDEX idx_grievances_status ON grievances(status);
CREATE INDEX idx_grievances_category ON grievances(category);
CREATE INDEX idx_grievances_priority ON grievances(priority);
CREATE INDEX idx_grievances_created_at ON grievances(created_at DESC);
CREATE INDEX idx_grievances_anonymous ON grievances(anonymous_identifier) WHERE anonymous_identifier IS NOT NULL;

CREATE INDEX idx_grievance_status_history_grievance ON grievance_status_history(grievance_id);
CREATE INDEX idx_grievance_comments_grievance ON grievance_comments(grievance_id);
CREATE INDEX idx_grievance_upvotes_grievance ON grievance_upvotes(grievance_id);
CREATE INDEX idx_grievance_upvotes_user ON grievance_upvotes(user_id);

-- Trigger to update updated_at timestamp
CREATE TRIGGER grievances_updated_at BEFORE UPDATE ON grievances
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();

CREATE TRIGGER grievance_comments_updated_at BEFORE UPDATE ON grievance_comments
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();

CREATE TRIGGER departments_updated_at BEFORE UPDATE ON departments
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();

-- Function to automatically log status changes
CREATE OR REPLACE FUNCTION log_grievance_status_change()
RETURNS TRIGGER AS $$
BEGIN
    IF OLD.status IS DISTINCT FROM NEW.status THEN
        INSERT INTO grievance_status_history (
            grievance_id,
            old_status,
            new_status,
            updated_by,
            updated_by_role
        ) VALUES (
            NEW.id,
            OLD.status,
            NEW.status,
            NEW.assigned_to, -- This should ideally be passed from the update context
            NULL -- Role would need to be passed separately
        );
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER track_grievance_status_changes
    AFTER UPDATE ON grievances
    FOR EACH ROW
    EXECUTE FUNCTION log_grievance_status_change();

-- Insert default departments
INSERT INTO departments (name, description) VALUES
    ('Infrastructure', 'Handles infrastructure-related issues including buildings, roads, water supply, electricity'),
    ('Academics', 'Manages academic concerns, course-related issues, examination matters'),
    ('Hostel', 'Oversees hostel facilities, room allocations, hostel maintenance'),
    ('Food Services', 'Handles mess and canteen related complaints and suggestions'),
    ('General Administration', 'General administrative matters not covered by other departments');
