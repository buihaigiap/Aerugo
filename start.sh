#!/bin/bash

echo "Starting Aerugo with Frontend..."

# Try different possible frontend paths
FRONTEND_PATHS=(
    "/app/Fe-AI-Decenter"
    "/app/app/Fe-AI-Decenter" 
    "./app/Fe-AI-Decenter"
    "./Fe-AI-Decenter"
)

FRONTEND_FOUND=false

for path in "${FRONTEND_PATHS[@]}"; do
    if [ -d "$path" ]; then
        echo "Found frontend directory at: $path"
        cd "$path"
        echo "Starting frontend dev server..."
        # Start frontend dev server in background
        npm run dev -- --host 0.0.0.0 --port 5173 &
        echo "Frontend dev server started on port 5173"
        FRONTEND_FOUND=true
        break
    fi
done

if [ "$FRONTEND_FOUND" = false ]; then
    echo "Frontend directory not found in any of the expected locations"
    echo "Available directories:"
    find /app -name "Fe-AI-Decenter" -type d 2>/dev/null || echo "No Fe-AI-Decenter directory found"
fi

# Wait a bit for frontend to start
sleep 2

# Start backend
echo "Starting backend API server..."
cd /app
aerugo