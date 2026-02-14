// vite.config.ts
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import { VitePWA } from 'vite-plugin-pwa';

export default defineConfig({
	plugins: [
		sveltekit(),
		VitePWA({
			registerType: 'autoUpdate',
			manifest: {
				name: 'AEGIS - Campus Grievance System',
				short_name: 'AEGIS',
				description: 'Voice your campus grievances and track their resolution',
				theme_color: '#d06065',
				background_color: '#ffffff',
				display: 'standalone',
				start_url: '/',
				icons: [
					{
						src: '/pwa-icons/192.png',
						sizes: '192x192',
						type: 'image/png'
					},
					{
						src: '/pwa-icons/512.png',
						sizes: '512x512',
						type: 'image/png'
					},
					{
						src: '/pwa-icons/512.png',
						sizes: '512x512',
						type: 'image/png',
						purpose: 'any maskable'
					}
				]
			},
			workbox: {
				globPatterns: ['**/*.{js,css,html,ico,png,svg,woff,woff2}']
			}
		})
	],
	server: {
		host: '0.0.0.0',
		port: 4173,
		hmr: {
			host: 'aegis.taneshq.iitmandi.in.net'
		}
	},
	preview: {
		host: '0.0.0.0',
		port: 4173,
		allowedHosts: ['aegis.taneshq.iitmandi.in.net']
	}
});