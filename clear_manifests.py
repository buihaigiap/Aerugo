#!/usr/bin/env python3

import psycopg2
import os
from urllib.parse import urlparse

# Get database URL
db_url = os.environ.get('DATABASE_URL')
if not db_url:
    print("❌ DATABASE_URL not set")
    exit(1)

# Parse URL
url = urlparse(db_url)

# Connect to database
try:
    conn = psycopg2.connect(
        database=url.path[1:],  # Remove leading slash
        user=url.username,
        password=url.password,
        host=url.hostname,
        port=url.port
    )
    cur = conn.cursor()
    
    print("✅ Connected to database")
    
    # Clear existing data for fresh test
    cur.execute("DELETE FROM tags")
    cur.execute("DELETE FROM manifests")
    
    conn.commit()
    print("✅ Cleared existing manifests and tags")
    
    cur.close()
    conn.close()
    print("✅ Database cleanup completed")
    
except Exception as e:
    print(f"❌ Error: {e}")
    exit(1)
