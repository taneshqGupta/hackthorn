// Types matching backend Rust structs

export type UserRole = 'student' | 'faculty' | 'authority' | 'admin';
export type UserStatus = 'active' | 'suspended';

export interface UserResponse {
    id: string; // UUID as string
    email: string;
    role: UserRole;
    first_name: string;
    last_name: string;
    profile_picture: string | null;
    department: string | null;
    status?: UserStatus;
}

// Alias for convenience
export type User = UserResponse & {
    name?: string;
    avatar?: string;
};

export interface ApiResponse<T> {
    success: boolean;
    data: T | null;
    message: string | null;
}

// Grievance Types
export type GrievanceCategory = 'infrastructure' | 'academics' | 'hostel' | 'food' | 'other';
export type GrievancePriority = 'low' | 'medium' | 'high' | 'urgent';
export type GrievanceStatus = 'submitted' | 'under_review' | 'in_progress' | 'resolved' | 'rejected';

export interface Grievance {
    id: string;
    title: string;
    description: string;
    category: GrievanceCategory;
    priority: GrievancePriority;
    status: GrievanceStatus;
    location: string | null;
    is_anonymous: boolean;
    submitter_id: string;
    submitter_name: string | null;
    assigned_to: string | null;
    assigned_to_name: string | null;
    department_id: string | null;
    department_name: string | null;
    upvotes_count: number;
    photo_urls: string[] | null;
    resolution_summary: string | null;
    created_at: string;
    updated_at: string;
}

export interface GrievanceStatusHistory {
    id: string;
    grievance_id: string;
    status: GrievanceStatus;
    changed_by: string;
    changed_by_name: string | null;
    remarks: string | null;
    changed_at: string;
}

export interface GrievanceComment {
    id: string;
    grievance_id: string;
    commenter_id: string;
    commenter_name: string | null;
    comment: string;
    is_internal_note: boolean;
    created_at: string;
}

export interface Department {
    id: string;
    name: string;
    description: string | null;
    head_user_id: string | null;
    created_at: string;
}

