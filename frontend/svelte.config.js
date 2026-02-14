import adapter from '@sveltejs/adapter-auto';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		// adapter-auto only supports some environments, see https://svelte.dev/docs/kit/adapter-auto for a list.
		// If your environment is not supported, or you settled on a specific environment, switch out the adapter.
		// See https://svelte.dev/docs/kit/adapters for more information about adapters.
		adapter: adapter()
	},
	onwarn: (warning, handler) => {
		// Suppress a11y label warnings for DaisyUI styled labels
		if (warning.code === 'a11y_label_has_associated_control') {
			return;
		}
		handler(warning);
	}
};

export default config;
