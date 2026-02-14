-- Enums for Academic Structure
CREATE TYPE course_type AS ENUM ('core', 'elective', 'major', 'minor');
CREATE TYPE resource_type AS ENUM ('pyq', 'notes', 'lecture', 'assignment');
CREATE TYPE event_type AS ENUM ('exam', 'deadline', 'holiday', 'class');
CREATE TYPE attendance_status AS ENUM ('present', 'absent', 'cancelled');

-- 1. COURSES TABLE
CREATE TABLE courses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(20) NOT NULL UNIQUE,       -- e.g., "CS301"
    title VARCHAR(255) NOT NULL,            -- e.g., "Operating Systems"
    description TEXT,
    credits INTEGER NOT NULL DEFAULT 3,
    department VARCHAR(100) NOT NULL,       -- e.g., "CSE" (Matching your users table pattern)
    course_type course_type NOT NULL,
    instructor_id UUID REFERENCES users(id) ON DELETE SET NULL, -- Link to a Faculty user
    semester VARCHAR(50) NOT NULL,          -- e.g., "Spring 2026"
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- 2. COURSE ENROLLMENTS (Linking Students to Courses)
CREATE TABLE course_enrollments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    student_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    course_id UUID NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    enrolled_at TIMESTAMPTZ DEFAULT NOW(),
    
    UNIQUE(student_id, course_id)
);

-- 3. ATTENDANCE LOGS
CREATE TABLE attendance_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    enrollment_id UUID NOT NULL REFERENCES course_enrollments(id) ON DELETE CASCADE,
    date DATE NOT NULL DEFAULT CURRENT_DATE,
    status attendance_status NOT NULL,
    remarks VARCHAR(255),
    
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- 4. ACADEMIC RESOURCES (The Vault)
CREATE TABLE academic_resources (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    course_id UUID NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    uploaded_by UUID REFERENCES users(id) ON DELETE SET NULL,
    
    title VARCHAR(255) NOT NULL,
    description TEXT,
    resource_type resource_type NOT NULL,
    file_url TEXT NOT NULL,
    
    year INTEGER,
    tags TEXT[],
    is_verified BOOLEAN DEFAULT FALSE,
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- 5. ACADEMIC EVENTS (The Calendar)
CREATE TABLE academic_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    course_id UUID REFERENCES courses(id) ON DELETE CASCADE, -- NULL = Global Event
    created_by UUID REFERENCES users(id) ON DELETE SET NULL,
    
    title VARCHAR(255) NOT NULL,
    description TEXT,
    event_type event_type NOT NULL,
    
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ,
    
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX idx_courses_code ON courses(code);
CREATE INDEX idx_enrollments_student ON course_enrollments(student_id);
CREATE INDEX idx_attendance_enrollment ON attendance_logs(enrollment_id);
CREATE INDEX idx_resources_course ON academic_resources(course_id);
CREATE INDEX idx_events_start_time ON academic_events(start_time);

-- Apply your existing timestamp trigger
CREATE TRIGGER courses_updated_at BEFORE UPDATE ON courses
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();

CREATE TRIGGER resources_updated_at BEFORE UPDATE ON academic_resources
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();

CREATE TRIGGER events_updated_at BEFORE UPDATE ON academic_events
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();