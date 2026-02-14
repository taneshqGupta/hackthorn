// vite.config.ts
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	server: {
		host: '0.0.0.0',
		port: 4173,
		hmr: {
			host: 'aegis.taneshq.iitmandi.in.net'
		},
		proxy: {
			'/auth': {
				target: 'https://hackthorn-backend.iitmandi.in.net',
				changeOrigin: true,
				secure: true,
				cookieDomainRewrite: 'localhost'
			}
		}
	},
	preview: {
		host: '0.0.0.0',
		port: 4173,
		allowedHosts: ['aegis.taneshq.iitmandi.in.net'],
		proxy: {
			'/auth': {
				target: 'https://hackthorn-backend.iitmandi.in.net',
				changeOrigin: true,
				secure: true,
				cookieDomainRewrite: 'localhost'
			}
		}
	}
});