// src/lib/types.ts

// --- Enums ---
export type UserRole = 'student' | 'faculty' | 'authority' | 'admin';
export type UserStatus = 'active' | 'suspended';
export type GrievanceCategory = 'infrastructure' | 'academics' | 'hostel' | 'food' | 'other';
export type GrievancePriority = 'low' | 'medium' | 'high' | 'urgent';
export type GrievanceStatus = 'submitted' | 'under_review' | 'in_progress' | 'resolved' | 'closed';

// --- Responses ---

export interface UserResponse {
    id: string;
    email: string;
    role: UserRole;
    first_name: string;
    last_name: string;
    profile_picture: string | null;
    department?: string | null;
}

// Matches Rust 'GrievanceCommentResponse'
export interface GrievanceComment {
    id: string;
    user: UserResponse; // Backend sends the whole user object
    comment: string;
    is_internal: boolean;
    created_at: string;
}

export interface SystemStats {
    total_users: number;
    active_users: number;
    total_grievances: number;
    pending_grievances: number;
    resolved_grievances: number;
    users_by_role: Record<string, number>;
}

export interface AuditLogResponse {
    id: string;
    user: UserResponse | null;
    action: string;
    metadata: any | null;
    created_at: string;
}

// Matches Rust 'GrievanceStatusHistoryResponse'
export interface GrievanceStatusHistory {
    id: string;
    old_status: GrievanceStatus | null;
    new_status: GrievanceStatus;
    remarks: string | null;
    updated_by: UserResponse | null; // Backend sends object
    created_at: string;
}

// Matches Rust 'GrievanceResponse'
export interface Grievance {
    id: string;

    // Submitter Info
    submitter: UserResponse | null;
    is_anonymous: boolean;

    // Core Details
    title: string;
    description: string;
    category: GrievanceCategory;
    priority: GrievancePriority;
    status: GrievanceStatus;

    // Location
    location_type: string | null;
    location_details: string | null;

    // Media
    photo_urls: string[]; // Rust sends empty array, not null, but good to be safe

    // Assignment
    assigned_to: UserResponse | null;
    assigned_department: string | null;

    // Resolution
    resolution_notes: string | null;
    resolved_at: string | null;

    // Stats
    view_count: number;
    upvote_count: number;
    user_has_upvoted: boolean; // Critical for the UI toggle

    // Timestamps
    created_at: string;
    updated_at: string;
}

export interface Department {
    id: string;
    name: string;
    description: string | null;
    head_user_id: string | null;
    created_at: string;
}

export interface ApiResponse<T> {
    success: boolean;
    data: T; // Rust usually sends data or throws error
    message: string | null;
}

export type User = UserResponse;

// --- Academic Mastery (Pillar III) ---

export type CourseType = 'core' | 'elective' | 'major' | 'minor';
export type ResourceType = 'pyq' | 'notes' | 'lecture' | 'assignment';
export type AttendanceStatus = 'present' | 'absent' | 'cancelled';

export interface Course {
    id: string;
    code: string;
    title: string;
    description: string | null;
    credits: number;
    department: string;
    course_type: CourseType;
    instructor: UserResponse | null;
    semester: string;
}

export interface CourseResponse {
    id: string;
    code: string;
    title: string;
    description?: string;
    credits: number;
    department: string;
    course_type: 'core' | 'elective' | 'major' | 'minor';
    instructor?: {
        first_name: string;
        last_name: string;
    };
    semester: string;
}

export interface AttendanceLog {
    id: string;
    enrollment_id: string;
    date: string; // YYYY-MM-DD
    status: 'present' | 'absent' | 'cancelled';
    remarks?: string;
    created_at: string;
}

export interface AttendanceSummary {
    course_id: string;
    total_classes: number;
    present_count: number;
    percentage: number;
    logs: AttendanceLog[];
}

export interface AcademicResource {
    id: string;
    title: string;
    description: string | null;
    resource_type: ResourceType;
    file_url: string;
    uploaded_by: UserResponse | null;
    year: number | null;
    tags: string[];
    created_at: string;
}