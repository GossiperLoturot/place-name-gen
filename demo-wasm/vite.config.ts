import { defineConfig } from "vite";
import solidPlugin from "vite-plugin-solid";

export default defineConfig({
  base: process.env.BASE_PATH,
  plugins: [solidPlugin()],
  server: {
    fs: {
      allow: [".."],
    },
  },
});
