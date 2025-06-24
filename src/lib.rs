//! Reference Tool - A command-line tool to fetch paper citations via INSPIRE-HEP API
//!
//! This crate provides functionality to:
//! - Fetch paper information and references from INSPIRE-HEP API
//! - Generate BibTeX bibliographies
//! - Build citation networks
//! - Export data in JSON and BibTeX formats
//!
//! # Example
//!
//! ```rust,no_run
//! use reference_tool::{api::InspireClient, output::{OutputWriter, OutputFormat}};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = InspireClient::new();
//!     let paper = client.get_paper_by_arxiv("2301.12345").await?;
//!     let references = client.get_paper_references(&paper.id).await?;
//!     
//!     let writer = OutputWriter::new(OutputFormat::Json, None);
//!     writer.write_references(&references).await?;
//!     
//!     Ok(())
//! }
//! ```

pub mod api;
pub mod models;
pub mod output;
pub mod network;
pub mod config;

// Re-export commonly used types
pub use api::InspireClient;
pub use models::{Paper, Reference};
pub use output::{OutputWriter, OutputFormat};
pub use network::CitationNetwork;
