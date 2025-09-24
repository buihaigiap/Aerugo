import { defineConfig, loadEnv } from 'vite'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'
import path from 'path'

// https://vite.dev/config/
export default defineConfig(({ mode }) => {
  // Load env file from root directory (../../.env)
  const env = loadEnv(mode, path.resolve(__dirname, '../../'), '')

  const allowedHosts = [
    'localhost',
    '127.0.0.1',
  ]

  // Thêm host từ environment variable nếu có
  if (env.VITE_ALLOWED_HOST) {
    allowedHosts.push(env.VITE_ALLOWED_HOST)
  }

  // Thêm domain pattern từ environment variable nếu có
  if (env.VITE_ALLOWED_DOMAIN_PATTERN) {
    allowedHosts.push(env.VITE_ALLOWED_DOMAIN_PATTERN)
  }

  return {
    plugins: [react(), tailwindcss()],
    define: {
      'import.meta.env.VITE_API_BASE_URL': JSON.stringify(env.VITE_API_BASE_URL)
    },
    server: {
      host: env.VITE_HOST === 'true' || true, // Default true, có thể override bằng VITE_HOST
      port: parseInt(env.VITE_PORT || '5173'), // Default 5173, có thể override bằng VITE_PORT
      allowedHosts: allowedHosts
    }
  }
})
