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

// Generic API helper
const api = {
    async get<T = any>(path: string): Promise<T> {
        const url = `${PUBLIC_BACKEND_URL}${path.startsWith('/') ? path.slice(1) : path}`;
        console.log(`[API] GET ${url}`);
        
        const response = await fetch(url, {
            method: 'GET',
            credentials: 'include',
        });
        
        console.log(`[API] GET ${path} - Status: ${response.status}`);
        
        if (!response.ok) {
            const error = await response.json().catch(() => ({ message: response.statusText }));
            console.error(`[API] GET ${path} - Failed:`, error);
            throw new Error(error.message || `GET ${path} failed`);
        }
        const data = await response.json();
        console.log(`[API] GET ${path} - Success:`, data);
        return data;
    },
    async post<T = any>(path: string, data?: any): Promise<T> {
        const isFormData = data instanceof FormData;
        const url = `${PUBLIC_BACKEND_URL}${path.startsWith('/') ? path.slice(1) : path}`;
        
        console.log(`[API] POST ${url}`);
        console.log(`[API] POST ${path} - Request body:`, isFormData ? '[FormData]' : data);
        console.log(`[API] POST ${path} - Content-Type:`, isFormData ? 'multipart/form-data' : 'application/json');
        
        const response = await fetch(url, {
            method: 'POST',
            credentials: 'include',
            headers: isFormData ? {} : { 'Content-Type': 'application/json' },
            body: isFormData ? data : JSON.stringify(data),
        });
        
        console.log(`[API] POST ${path} - Status: ${response.status}`);
        console.log(`[API] POST ${path} - Status text: ${response.statusText}`);
        
        if (!response.ok) {
            console.error(`[API] POST ${path} - Response not OK, attempting to parse error`);
            const error = await response.json().catch((e) => {
                console.error(`[API] POST ${path} - Failed to parse error response:`, e);
                return { message: response.statusText };
            });
            console.error(`[API] POST ${path} - Error response:`, error);
            throw new Error(error.message || `POST ${path} failed`);
        }
        
        const responseData = await response.json();
        console.log(`[API] POST ${path} - Success response:`, responseData);
        return responseData;
    },
    async put<T = any>(path: string, data?: any): Promise<T> {
        const response = await fetch(`${PUBLIC_BACKEND_URL}${path.startsWith('/') ? path.slice(1) : path}`, {
            method: 'PUT',
            credentials: 'include',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(data),
        });
        if (!response.ok) {
            const error = await response.json().catch(() => ({ message: response.statusText }));
            throw new Error(error.message || `PUT ${path} failed`);
        }
        return response.json();
    },
    async delete<T = any>(path: string): Promise<T> {
        const response = await fetch(`${PUBLIC_BACKEND_URL}${path.startsWith('/') ? path.slice(1) : path}`, {
            method: 'DELETE',
            credentials: 'include',
        });
        if (!response.ok) {
            const error = await response.json().catch(() => ({ message: response.statusText }));
            throw new Error(error.message || `DELETE ${path} failed`);
        }
        return response.json();
    },
};

export default api;
export { api };
