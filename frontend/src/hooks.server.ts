import type { Handle } from '@sveltejs/kit';
import { user } from '$lib/auth';
import { get } from 'svelte/store';

export const handle: Handle = async ({ event, resolve }) => {
	console.log(`[HOOKS] Handling request for: ${event.url.pathname}`);
	const currentUser = get(user);
	console.log('[HOOKS] Current user from store:', currentUser);

	// Allow access to login, auth-error, and other public pages without authentication
	const publicPaths = ['/login', '/auth-error', '/'];
	const isPublicPath = publicPaths.includes(event.url.pathname);
	
	console.log('[HOOKS] Is public path:', isPublicPath);

	if (!currentUser && !isPublicPath) {
		console.log('[HOOKS] User not authenticated and trying to access protected route, redirecting to /login');
		return new Response(null, {
			status: 302,
			headers: {
				location: '/login'
			}
		});
	}

	console.log('[HOOKS] Resolving request');
	try {
		const response = await resolve(event);
		console.log(`[HOOKS] Resolved request for: ${event.url.pathname} with status: ${response.status}`);
		return response;
	} catch (error) {
		console.error(`[HOOKS] Error resolving request for ${event.url.pathname}:`, error);
		return new Response('Internal Server Error', { status: 500 });
	}
};
