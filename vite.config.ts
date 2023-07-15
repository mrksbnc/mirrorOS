import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import { fileURLToPath, URL } from 'node:url';

// https://vitejs.dev/config/
export default defineConfig(async () => ({
	plugins: [vue()],
	clearScreen: false,
	envPrefix: ['VITE_', 'TAURI_'],
	resolve: {
		alias: {
			'@': fileURLToPath(new URL('./src', import.meta.url)),
		},
	},
	server: {
		port: 1420,
		strictPort: true,
	},
	build: {
		target: process.env.TAURI_PLATFORM == 'windows' ? 'chrome105' : 'safari13',
		minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
		sourcemap: !!process.env.TAURI_DEBUG,
	},
}));
