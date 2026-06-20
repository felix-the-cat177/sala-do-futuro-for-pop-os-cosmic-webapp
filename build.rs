// build.rs - Build script para otimizações

fn main() {
    // Habilitar link-time optimization (LTO)
    println!("cargo:rustc-env=RUSTFLAGS=-C target-cpu=native -C lto -C codegen-units=1");
    
    // Para Vulkan, habilitar features específicas
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-search=native=/usr/lib");
        println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu");
        
        // Vulkan
        println!("cargo:rustc-link-lib=vulkan");
    }

    // Versão da compilação
    let version = env!("CARGO_PKG_VERSION");
    println!("cargo:rustc-env=SALA_VERSION={}", version);

    // Hash do commit (se disponível)
    if let Ok(output) = std::process::Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
    {
        if output.status.success() {
            let git_hash = String::from_utf8(output.stdout)
                .unwrap_or_default()
                .trim()
                .to_string();
            println!("cargo:rustc-env=GIT_HASH={}", git_hash);
        }
    }
}
