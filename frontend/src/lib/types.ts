// Types matching backend Rust structs

export type UserRole = 'student' | 'faculty' | 'authority' | 'admin';

export interface UserResponse {
    id: string; // UUID as string
    email: string;
    role: UserRole;
    first_name: string;
    last_name: string;
    profile_picture: string | null;
    department: string | null;
}

export interface ApiResponse<T> {
    success: boolean;
    data: T | null;
    message: string | null;
}
