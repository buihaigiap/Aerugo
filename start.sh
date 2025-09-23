#!/bin/bash

echo "Starting Aerugo Container Registry..."

# Check if frontend dist exists
if [ -d "/app/Fe-AI-Decenter/dist" ]; then
    echo "✅ Frontend static files found at /app/Fe-AI-Decenter/dist"
else
    echo "⚠️  Frontend static files not found - frontend will show fallback page"
fi

# Start backend API server (which will serve both API and frontend static files)
echo "🚀 Starting backend API server..."
echo "Frontend and Backend will be served on the same port: 8080"
cd /app
aerugo