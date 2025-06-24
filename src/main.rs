use clap::{Args, Parser, Subcommand};
use log::info;
use std::path::PathBuf;

mod api;
mod models;
mod output;
mod network;
mod config;

use crate::api::InspireClient;
use crate::output::{OutputFormat, OutputWriter};
use crate::network::CitationNetwork;
use crate::config::Config;

#[derive(Parser)]
#[command(name = "reference_tool")]
#[command(about = "A tool to fetch paper citations via INSPIRE-HEP API")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    /// ArXiv ID of the paper
    #[arg(long, global = true)]
    arxiv_id: Option<String>,
    
    /// Output format
    #[arg(long, value_enum, default_value_t = OutputFormat::Json, global = true)]
    format: OutputFormat,
    
    /// Output file path
    #[arg(long, global = true)]
    output: Option<PathBuf>,
    
    /// Categories to filter (comma-separated)
    #[arg(long, global = true)]
    categories: Option<String>,
    
    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Build citation network
    Network(NetworkArgs),
    /// Show current configuration
    Config,
    /// Initialize configuration file
    InitConfig,
}

#[derive(Args)]
struct NetworkArgs {
    /// ArXiv ID of the paper (can also be specified globally)
    arxiv_id: Option<String>,
    /// Depth of the citation network
    #[arg(long, default_value_t = 1)]
    depth: u32,
    /// Build the network
    #[arg(long)]
    build_network: bool,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();
    
    // Load configuration
    let config = Config::load().unwrap_or_else(|_| {
        eprintln!("Warning: Could not load configuration, using defaults");
        Config::default()
    });
    
    // Initialize logger with effective verbosity
    let verbose = config.effective_verbose(cli.verbose);
    if verbose {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Debug)
            .init();
    } else {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .init();
    }
    
    let client = InspireClient::new();
    
    // Use config defaults for CLI options
    let format = config.effective_format(Some(cli.format));
    let output_path = cli.output.or_else(|| config.effective_output_dir(None));
    let output_writer = OutputWriter::new(format, output_path);
    
    match cli.command {
        Some(Commands::Config) => {
            config.show()?;
        }
        Some(Commands::InitConfig) => {
            let default_config = Config::default();
            default_config.save()?;
        }
        Some(Commands::Network(args)) => {
            let arxiv_id = args.arxiv_id.or(cli.arxiv_id)
                .ok_or_else(|| anyhow::anyhow!("ArXiv ID is required"))?;
            
            if !args.build_network {
                return Err(anyhow::anyhow!("--build-network flag is required for network command"));
            }
            
            info!("Building citation network for paper: {} with depth: {}", arxiv_id, args.depth);
            
            let mut network = CitationNetwork::new();
            network.build(&client, &arxiv_id, args.depth).await?;
            
            output_writer.write_network(&network).await?;
            info!("Built network with {} papers", network.paper_count());
        }
        None => {
            // Default behavior: fetch references
            let arxiv_id = cli.arxiv_id
                .ok_or_else(|| anyhow::anyhow!("ArXiv ID is required"))?;
            
            info!("Fetching references for paper: {}", arxiv_id);
            
            let paper = client.get_paper_by_arxiv(&arxiv_id).await?;
            println!("📄 Found paper: {}", paper.title);
            
            let references = client.get_paper_references(&paper.id).await?;
            
            let filtered_refs = if let Some(categories) = config.effective_categories(cli.categories) {
                references.into_iter()
                    .filter(|r| r.categories.iter().any(|c| categories.contains(c)))
                    .collect()
            } else {
                references
            };
            
            output_writer.write_references(&filtered_refs).await?;
            println!("✅ Successfully processed {} references", filtered_refs.len());
            info!("Found {} references", filtered_refs.len());
        }
    }
    
    Ok(())
}
