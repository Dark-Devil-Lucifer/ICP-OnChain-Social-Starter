import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  server: {
    port: 5173
  },
  build: {
    outDir: 'dist'
  },
  define: {
    // Allow injecting the canister id via environment variable
    'import.meta.env.VITE_BACKEND_CANISTER_ID': JSON.stringify(process.env.CANISTER_ID_BACKEND || process.env.VITE_BACKEND_CANISTER_ID || '')
  }
})
