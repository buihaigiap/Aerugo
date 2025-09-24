use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=app/Fe-AI-Decenter/src");
    println!("cargo:rerun-if-changed=app/Fe-AI-Decenter/package.json");
    println!("cargo:rerun-if-changed=app/Fe-AI-Decenter/vite.config.ts");

    // Only build frontend in release mode to avoid slow dev builds
    let profile = env::var("PROFILE").unwrap_or_default();
    
    // Always build frontend for proper integration
    build_frontend();
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
