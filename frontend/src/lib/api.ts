import { PUBLIC_BACKEND_URL } from '$env/static/public';
import type { ApiResponse, UserResponse } from '$lib/types';

// Check authentication status
export async function checkAuth(): Promise<ApiResponse<UserResponse>> {
    console.log('[API] checkAuth called');
    console.log('[API] Backend URL:', PUBLIC_BACKEND_URL);
    console.log('[API] Fetching:', `${PUBLIC_BACKEND_URL}auth/me`);
    
    const response = await fetch(`${PUBLIC_BACKEND_URL}auth/me`, {
        method: "GET",
        credentials: "include",
    });

    console.log('[API] checkAuth response status:', response.status);
    console.log('[API] checkAuth response ok:', response.ok);
    console.log('[API] checkAuth response headers:', Object.fromEntries(response.headers.entries()));

    if (!response.ok) {
        console.error('[API] checkAuth failed with status:', response.status);
        throw new Error(`Auth check failed: ${response.statusText}`);
    }
    
    const data = await response.json();
    console.log('[API] checkAuth response data:', data);
    return data;
}

// Logout user
export async function logout(): Promise<ApiResponse<void>> {
    const response = await fetch(`${PUBLIC_BACKEND_URL}auth/logout`, {
        method: "GET",
        credentials: "include"
    });

    if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        throw new Error(errorData.message || `Logout failed: ${response.statusText}`);
    }
    return response.json();
}

// Get current user profile
export async function getMyProfile(): Promise<ApiResponse<UserResponse>> {
    const response = await fetch(`${PUBLIC_BACKEND_URL}auth/me`, {
        method: "GET",
        credentials: "include",
    });

    if (!response.ok) {
        throw new Error(`Failed to get profile: ${response.statusText}`);
    }
    return response.json();
}

// Get Google OAuth login URL (for redirecting)
export function getGoogleLoginUrl(): string {
    return `${PUBLIC_BACKEND_URL}auth/google`;
}
