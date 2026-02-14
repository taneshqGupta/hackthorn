import { writable } from 'svelte/store';
import { browser } from '$app/environment';
import { goto } from '$app/navigation';
import { checkAuth, logout as apiLogout } from './api';
import type { User } from './types';

export const user = writable<User | null>(null);

if (browser) {
	checkAuth()
		.then((data) => {
			if (data.success && data.data) {
				// Map UserResponse to User with convenience fields
				const userData: User = {
					...data.data,
					name: `${data.data.first_name} ${data.data.last_name}`,
					avatar: data.data.profile_picture || undefined
				};
				user.set(userData);
			} else {
				user.set(null);
			}
		})
		.catch(() => {
			user.set(null);
		});
}

