import { defineConfig, loadEnv } from "vite";
import react from "@vitejs/plugin-react-swc";
import path from "path";

// https://vitejs.dev/config/
export default defineConfig(({ mode }) => {
    const env = loadEnv(mode, process.cwd());
    const apiUrl = env.VITE_API_URL || "/api";

    return {
        server: {
            proxy: {
                "/api": {
                    target: apiUrl,
                    changeOrigin: true,
                },
            },
        },
        plugins: [react()],
        resolve: {
            alias: {
                "@": path.resolve(__dirname, "src"),
            },
        },
    };
});
