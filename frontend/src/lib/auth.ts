import { writable } from 'svelte/store';
import { browser } from '$app/environment';
import { goto } from '$app/navigation';
import { checkAuth, logout as apiLogout } from './api';
import type { User } from './types';

console.log('[AUTH] Initializing auth store');

export const user = writable<User | null>(null);

if (browser) {
	console.log('[AUTH] Browser detected, checking authentication');
	checkAuth()
		.then((data) => {
			console.log('[AUTH] checkAuth response:', data);
			if (data.success && data.data) {
				// Map UserResponse to User with convenience fields
				const userData: User = {
					...data.data,
					name: `${data.data.first_name} ${data.data.last_name}`,
					avatar: data.data.profile_picture || undefined
				};
				console.log('[AUTH] User authenticated, setting user:', userData);
				user.set(userData);
			} else {
				console.log('[AUTH] Not authenticated, setting user to null');
				user.set(null);
			}
		})
		.catch((error) => {
			console.error('[AUTH] checkAuth failed:', error);
			user.set(null);
		});
}

