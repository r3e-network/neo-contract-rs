// build.rs
fn main() {
    // Detect if we're building for wasm32
    let target = std::env::var("TARGET").unwrap_or_else(|_| String::new());
    if target.contains("wasm32") {
        // Don't manually set target_arch="wasm32" as it's already set by --target
        
        // Add additional flags for wasm32 target
        println!("cargo:rustc-cfg=wasm");
        
        // Configure WASM-specific settings
        println!("cargo:rustc-link-arg=--no-entry");
        println!("cargo:rustc-link-arg=--export-all");
        
        // Enable some additional features for wasm32
        println!("cargo:rustc-cfg=feature=\"wasm\"");
    }
} 