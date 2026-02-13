import { writable } from 'svelte/store';
import { browser } from '$app/environment';
import { goto } from '$app/navigation';
import { checkAuth, logout as apiLogout } from './api';
import type { User } from './types';

export const user = writable<User | null>(null);

if (browser) {
	checkAuth()
		.then((data) => {
			if (data.success) {
				user.set(data.data);
			} else {
				user.set(null);
			}
		})
		.catch(() => {
			user.set(null);
		});
}

