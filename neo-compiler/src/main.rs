use anyhow::Result;
use clap::{Parser, Subcommand};
use env_logger::Env;
use log::info;
use neo_compiler::NeoCompiler;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "neo-compiler")]
#[command(about = "Neo N3 Smart Contract Compiler", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Enable debug output
    #[arg(short, long)]
    debug: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile a WASM file to NEF format
    Compile {
        /// Input WASM file
        input: PathBuf,
        
        /// Output directory
        #[arg(short, long, default_value = "build")]
        output: PathBuf,
        
        /// Manifest file (optional, auto-detected for Solana-style)
        #[arg(short, long)]
        manifest: Option<PathBuf>,
        
        /// Source identifier
        #[arg(short, long, default_value = "neo-compiler")]
        source: String,
    },
    
    /// Compile all examples in the project
    CompileAll {
        /// Output directory
        #[arg(short, long, default_value = "build")]
        output: PathBuf,
    },
    
    /// Verify a NEF file
    Verify {
        /// NEF file to verify
        nef: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logger
    let log_level = if cli.debug { "debug" } else { "info" };
    env_logger::Builder::from_env(Env::default().default_filter_or(log_level)).init();
    
    match cli.command {
        Commands::Compile { input, output, manifest: _, source } => {
            info!("Compiling {} to {}", input.display(), output.display());
            
            let compiler = NeoCompiler::new()
                .with_debug(cli.debug)
                .with_source(source)
                .with_output_dir(output);
            
            let result = compiler.compile(&input)?;
            
            info!("✅ Compilation successful!");
            info!("  NEF: {}", result.nef_path.display());
            info!("  Manifest: {}", result.manifest_path.display());
            
            if result.is_solana_style {
                info!("  Style: Solana-style contract detected");
            }
            
            info!("  Script size: {} bytes", result.nef.script.len());
            info!("  Methods: {}", result.manifest.abi.methods.len());
            
            if !result.manifest.supported_standards.is_empty() {
                info!("  Standards: {}", result.manifest.supported_standards.join(", "));
            }
        }
        
        Commands::CompileAll { output } => {
            info!("Compiling all examples to {}", output.display());
            
            let compiler = NeoCompiler::new()
                .with_debug(cli.debug)
                .with_output_dir(output);
            
            let results = compiler.compile_all_examples()?;
            
            info!("✅ Compiled {} contracts", results.len());
            
            for result in &results {
                info!("  - {}", result.nef_path.file_name().unwrap().to_string_lossy());
                if result.is_solana_style {
                    info!("    (Solana-style)");
                }
            }
        }
        
        Commands::Verify { nef } => {
            info!("Verifying {}", nef.display());
            
            let nef_bytes = std::fs::read(&nef)?;
            let nef = neo_compiler::nef::Nef3::from_bytes(&nef_bytes)?;
            
            info!("NEF File Information:");
            info!("  Magic: 0x{:08X}", nef.magic);
            info!("  Compiler: {}", nef.compiler);
            info!("  Source: {}", nef.source);
            info!("  Script size: {} bytes", nef.script.len());
            info!("  Checksum: 0x{:08X}", nef.checksum);
            
            if nef.verify_checksum() {
                info!("✅ Checksum valid");
            } else {
                info!("❌ Checksum invalid!");
            }
            
            if !nef.tokens.is_empty() {
                info!("  Method tokens: {}", nef.tokens.len());
                for token in &nef.tokens {
                    info!("    - {}", token.method);
                }
            }
        }
    }
    
    Ok(())
}