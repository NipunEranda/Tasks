import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import tailwindcss from '@tailwindcss/vite'

// https://vite.dev/config/
export default defineConfig({
  base: '/',
  server: {
    port: 3000,
    proxy: {
      '/api/': {
        target: 'http://localhost:8000/api/',
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api/, ''),
      },
    }
  },
  plugins: [
    vue(),
    tailwindcss()
  ],
  resolve: {
    alias: {
      '@': '/src', // Ensure this alias is correctly set
    },
  },
})
