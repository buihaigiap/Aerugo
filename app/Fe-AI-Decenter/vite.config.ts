import path from 'path';
import { defineConfig, loadEnv } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig(({ mode }) => {
  // Load environment variables from root directory
  const env = loadEnv(mode, '../../', '');

  // Parse allowed hosts from environment variable and add domain
  const allowedHosts = [];

  // Add hosts from VITE_ALLOWED_HOSTS
  if (env.VITE_ALLOWED_HOSTS) {
    allowedHosts.push(...env.VITE_ALLOWED_HOSTS.split(',').map(host => host.trim()));
  }

  // Add domain from VITE_DOMAIN
  if (env.VITE_DOMAIN) {
    allowedHosts.push(env.VITE_DOMAIN);
  }

  // Default to localhost if no hosts specified
  if (allowedHosts.length === 0) {
    allowedHosts.push('localhost');
  }

  return {
    plugins: [react()],
    server: {
      host: '0.0.0.0',
      port: 5173,
      allowedHosts: allowedHosts
    },
    define: {
      'process.env.API_BASE_URL': JSON.stringify(env.VITE_API_BASE_URL || 'http://localhost:8080'),
      'process.env.DOMAIN': JSON.stringify(env.VITE_DOMAIN || 'localhost'),
      'process.env.GEMINI_API_KEY': JSON.stringify(env.VITE_GEMINI_API_KEY),
      'process.env.API_KEY': JSON.stringify(env.VITE_GEMINI_API_KEY)
    },
    resolve: {
      alias: {
        '@': path.resolve(__dirname, '.'),
      }
    }
  };
});
