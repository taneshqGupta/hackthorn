import type { Handle } from '@sveltejs/kit';

// Server-side request handling
// Since we're using session cookies from the backend, 
// the browser automatically sends them with credentials: 'include'
export const handle: Handle = async ({ event, resolve }) => {
    // You can add server-side auth checks here if needed
    // For now, we rely on client-side checks and backend session validation
    
    const response = await resolve(event);
    return response;
};
