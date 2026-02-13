import { writable } from 'svelte/store';
import { browser } from '$app/environment';
import { goto } from '$app/navigation';
import { checkAuth, logout as apiLogout } from './api';
import type { UserResponse } from './types';

interface AuthState {
    isAuthenticated: boolean;
    user: UserResponse | null;
    loading: boolean;
}

const initialState: AuthState = {
    isAuthenticated: false,
    user: null,
    loading: true
};

export const authStore = writable<AuthState>(initialState);

// Check authentication status
export async function initAuth() {
    if (!browser) return;
    
    authStore.update(state => ({ ...state, loading: true }));
    
    try {
        const response = await checkAuth();
        if (response.success && response.data) {
            authStore.set({
                isAuthenticated: true,
                user: response.data,
                loading: false
            });
        } else {
            authStore.set({
                isAuthenticated: false,
                user: null,
                loading: false
            });
        }
    } catch (error) {
        console.log('Auth check failed:', error);
        authStore.set({
            isAuthenticated: false,
            user: null,
            loading: false
        });
    }
}

// Login success handler (called after Google OAuth redirect)
export function setAuthenticated(user: UserResponse) {
    authStore.set({
        isAuthenticated: true,
        user,
        loading: false
    });
}

// Logout handler
export async function logout() {
    try {
        await apiLogout();
    } catch (error) {
        console.error('Logout error:', error);
    }
    
    authStore.set({
        isAuthenticated: false,
        user: null,
        loading: false
    });
    
    goto('/login');
}

// Redirect to login page if not authenticated
export function requireAuth() {
    if (!browser) return;
    
    authStore.subscribe(state => {
        if (!state.loading && !state.isAuthenticated) {
            goto('/login');
        }
    })();
}
