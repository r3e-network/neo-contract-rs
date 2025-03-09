use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use neo_compiler::compile;

/// Neo N3 smart contract compiler for Rust
#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile a WebAssembly file to a Neo N3 smart contract
    Compile {
        /// Path to the WebAssembly file to compile
        #[clap(value_parser)]
        wasm_file: PathBuf,
        
        /// Output directory for the compiled files
        #[clap(short, long, value_parser)]
        output: Option<PathBuf>,
        
        /// Name of the contract
        #[clap(short, long)]
        name: Option<String>,
        
        /// Overwrite existing files
        #[clap(short, long)]
        force: bool,
    },
    
    /// Show information about a NEF file
    Info {
        /// Path to the NEF file
        #[clap(value_parser)]
        nef_file: PathBuf,
    },
    
    /// Disassemble a NEF file to a human-readable format
    Disassemble {
        /// Path to the NEF file
        #[clap(value_parser)]
        nef_file: PathBuf,
        
        /// Output file for the disassembled code
        #[clap(short, long, value_parser)]
        output: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Compile {
            wasm_file,
            output,
            name,
            force,
        } => {
            // Check if the input file exists
            if !wasm_file.exists() {
                return Err(anyhow!("Input file not found: {:?}", wasm_file));
            }
            
            // Get the contract name
            let name = match name {
                Some(name) => name.clone(),
                None => {
                    // Use the input file name without extension
                    wasm_file
                        .file_stem()
                        .ok_or_else(|| anyhow!("Failed to get file name from {:?}", wasm_file))?
                        .to_string_lossy()
                        .to_string()
                }
            };
            
            // Get the output directory
            let output_dir = match output {
                Some(output) => output.clone(),
                None => {
                    // Use the current directory
                    std::env::current_dir()?
                }
            };
            
            // Check if the output directory exists
            if !output_dir.exists() {
                std::fs::create_dir_all(&output_dir)?;
            }
            
            // Check if the output files already exist
            let nef_path = output_dir.join(format!("{}.nef", &name));
            let manifest_path = output_dir.join(format!("{}.manifest.json", &name));
            
            if !*force && (nef_path.exists() || manifest_path.exists()) {
                return Err(anyhow!(
                    "Output files already exist. Use --force to overwrite."
                ));
            }
            
            // Compile the WebAssembly file
            compile(wasm_file, &output_dir, Some(&name))?;
            
            println!("Successfully compiled {} to Neo N3 smart contract", wasm_file.display());
            println!("Output files:");
            println!("  NEF file: {}", nef_path.display());
            println!("  Manifest file: {}", manifest_path.display());
            
            Ok(())
        }
        
        Commands::Info { nef_file } => {
            // Check if the input file exists
            if !nef_file.exists() {
                return Err(anyhow!("NEF file not found: {:?}", nef_file));
            }
            
            // Load the NEF file
            let nef = neo_compiler::NefFile::load_from(nef_file)?;
            
            // Print information about the NEF file
            println!("NEF File: {}", nef_file.display());
            println!("Magic: 0x{:08X}", nef.magic);
            println!("Version: {:?}", nef.version);
            println!("Compiler: {}", nef.compiler);
            println!("Script size: {} bytes", nef.script.len());
            println!("Checksum: {}", hex::encode(nef.checksum));
            println!("Checksum verification: {}", if nef.verify_checksum() { "OK" } else { "FAILED" });
            
            Ok(())
        }
        
        Commands::Disassemble { nef_file, output } => {
            // Check if the input file exists
            if !nef_file.exists() {
                return Err(anyhow!("NEF file not found: {:?}", nef_file));
            }
            
            // Load the NEF file
            let nef = neo_compiler::NefFile::load_from(nef_file)?;
            
            // Create a script from the NEF file
            let script = neo_compiler::Script::from_bytes(&nef.script);
            
            // Disassemble the script
            let disassembly = neo_compiler::script::disassemble_script(&script);
            
            // Output the disassembly
            match output {
                Some(output_path) => {
                    // Write to the output file
                    std::fs::write(output_path, disassembly)?;
                    println!("Disassembly written to {}", output_path.display());
                }
                None => {
                    // Print to stdout
                    println!("Disassembly of {}:", nef_file.display());
                    println!("{}", disassembly);
                }
            }
            
            Ok(())
        }
    }
}