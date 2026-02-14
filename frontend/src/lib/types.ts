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
