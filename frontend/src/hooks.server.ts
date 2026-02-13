import type { Handle } from '@sveltejs/kit';
import { user } from '$lib/auth';
import { get } from 'svelte/store';

export const handle: Handle = async ({ event, resolve }) => {
	console.log(`Handling request for: ${event.url.pathname}`);
	const currentUser = get(user);
	console.log('Current user from store:', currentUser);

	if (!currentUser && event.url.pathname !== '/login') {
		console.log('User not found and not on login page, redirecting to /login');
		return new Response(null, {
			status: 302,
			headers: {
				location: '/login'
			}
		});
	}

	console.log('Resolving request');
	try {
		const response = await resolve(event);
		console.log(`Resolved request for: ${event.url.pathname} with status: ${response.status}`);
		return response;
	} catch (error) {
		console.error(`Error resolving request for ${event.url.pathname}:`, error);
		// Return a generic error response
		return new Response('Internal Server Error', { status: 500 });
	}
};
