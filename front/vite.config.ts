import { tanstackRouter } from "@tanstack/router-plugin/vite";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

// https://vite.dev/config/
export default defineConfig({
	plugins: [
		tanstackRouter({
			target: "react",
			autoCodeSplitting: true,
		}),
		react({
			babel: {
				plugins: [["babel-plugin-react-compiler"]],
			},
		}),
	],
	server: {
		proxy: {
			"/api": {
				target: "http://localhost:9999",
				changeOrigin: true,
				rewrite: (path) => path.replace(/^\/api/, ""),
			},
			"/game": {
				target: "ws://localhost:9999",
				ws: true,
			},
		},
	},
});
