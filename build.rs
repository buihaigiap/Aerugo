use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=app/Fe-AI-Decenter/src");
    println!("cargo:rerun-if-changed=app/Fe-AI-Decenter/package.json");
    println!("cargo:rerun-if-changed=app/Fe-AI-Decenter/vite.config.ts");

    // Only build frontend in release mode to avoid slow dev builds
    let profile = env::var("PROFILE").unwrap_or_default();
    
    if profile == "release" {
        build_frontend();
    } else {
        // In debug mode, just ensure the dist directory exists with a simple fallback
        std::fs::create_dir_all("dist/static").unwrap_or_else(|_| {
            eprintln!("Warning: Could not create dist/static directory");
        });
        
        // Create a simple index.html for development - only if it doesn't exist
        let index_path = "dist/static/index.html";
        if !std::path::Path::new(index_path).exists() {
            let dev_html = r#"<!DOCTYPE html>
<html>
<head>
    <title>Aerugo Registry - Development</title>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <style>
        body { font-family: system-ui; padding: 2rem; text-align: center; background: #f5f5f5; }
        .container { max-width: 600px; margin: 0 auto; background: white; padding: 2rem; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        code { background: #e5e5e5; padding: 0.2rem 0.4rem; border-radius: 3px; }
        .link { display: inline-block; margin: 0.5rem; padding: 0.5rem 1rem; background: #007acc; color: white; text-decoration: none; border-radius: 4px; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🚀 Aerugo Container Registry</h1>
        <p><strong>Development Mode</strong></p>
        <p>Frontend dev server should be starting automatically...</p>
        <br>
        <a href="/docs" class="link">📖 API Documentation</a>
        <a href="http://localhost:5173" class="link">⚛️ Frontend Dev Server</a>
    </div>
</body>
</html>"#;
            
            std::fs::write(index_path, dev_html).unwrap_or_else(|_| {
                eprintln!("Warning: Could not create development index.html");
            });
        }
    }
}

fn build_frontend() {
    println!("Building frontend for production...");
    
    let fe_dir = "app/Fe-AI-Decenter";
    
    if !Path::new(fe_dir).exists() {
        eprintln!("Frontend directory not found: {}", fe_dir);
        return;
    }

    // Install dependencies
    println!("Installing frontend dependencies...");
    let npm_install = Command::new("npm")
        .current_dir(fe_dir)
        .args(&["install"])
        .status();

    match npm_install {
        Ok(status) if status.success() => {
            println!("Frontend dependencies installed successfully");
        }
        _ => {
            eprintln!("Warning: Failed to install frontend dependencies");
            return;
        }
    }

    // Build frontend
    println!("Building frontend...");
    let npm_build = Command::new("npm")
        .current_dir(fe_dir)
        .args(&["run", "build"])
        .status();

    match npm_build {
        Ok(status) if status.success() => {
            println!("Frontend built successfully");
        }
        _ => {
            eprintln!("Warning: Failed to build frontend");
            return;
        }
    }

    // Copy built files
    println!("Copying frontend files...");
    std::fs::create_dir_all("dist/static").unwrap_or_else(|e| {
        eprintln!("Failed to create dist directory: {}", e);
    });

    let copy_result = if cfg!(target_os = "windows") {
        Command::new("xcopy")
            .args(&[&format!("{}\\dist", fe_dir), "dist\\static", "/E", "/I", "/Y"])
            .status()
    } else {
        Command::new("cp")
            .args(&["-r", &format!("{}/dist/.", fe_dir), "dist/static/"])
            .status()
    };

    match copy_result {
        Ok(status) if status.success() => {
            println!("Frontend files copied successfully");
        }
        _ => {
            eprintln!("Warning: Failed to copy frontend files");
        }
    }
}
