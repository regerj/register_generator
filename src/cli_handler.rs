use clap::Args;
use clap::Parser;
use clap::Subcommand;

// CLI parsing structs
#[derive(Subcommand)]
pub enum Commands {
    /// Add a register to an existing JSON configuration file
    AddRegister(AddRegisterArgs),
    /// Generate new header files
    Generate(GenerateArgs),
    /// Creates an initial JSON file for a new register family
    Bootstrap(BootstrapArgs),
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Args)]
pub struct AddRegisterArgs {
    /// Path to the configuration JSON
    #[arg(short, long)]
    pub path: String,
    /// Name of the new register
    #[arg(short, long)]
    pub name: String,
    /// Size of the new register (must be supported by register family)
    #[arg(short, long, value_parser = register_size_supported)]
    pub size: u8,
}

#[derive(Args)]
pub struct GenerateArgs {
    /// Path to the configuration JSON
    #[arg(short, long)]
    pub path: String,
}

#[derive(Args)]
pub struct BootstrapArgs {
    /// Path to the JSON file to generate
    #[arg(short, long)]
    pub path: String,
    
    /// Name of the register family
    #[arg(short, long)]
    pub name: String,
}

fn register_size_supported(size: &str) -> Result<u8, String> {
    let supported_sizes = vec![8, 16, 32, 64];
    let size: usize = size
        .parse()
        .map_err(|_| format!("`{size}` isn't a number"))?;
    if supported_sizes.contains(&size) {
        Ok(size as u8)
    } else {
        Err("Unsupported size. Supported register sizes are: 8, 16, 32, 64".to_string())
    }
}
