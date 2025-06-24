use std::path::PathBuf;
use clap::ValueEnum;
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, BufWriter};
use anyhow::Result;
use serde_json;

use crate::models::Reference;
use crate::network::CitationNetwork;

#[derive(Debug, Clone, ValueEnum, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum OutputFormat {
    Json,
    Bibtex,
}

pub struct OutputWriter {
    format: OutputFormat,
    output_path: Option<PathBuf>,
}

impl OutputWriter {
    pub fn new(format: OutputFormat, output_path: Option<PathBuf>) -> Self {
        Self {
            format,
            output_path,
        }
    }
    
    /// Write references to output
    pub async fn write_references(&self, references: &[Reference]) -> Result<()> {
        let content = match self.format {
            OutputFormat::Json => self.format_json(references)?,
            OutputFormat::Bibtex => self.format_bibtex(references),
        };
        
        self.write_content(&content).await
    }
    
    /// Write citation network to output
    pub async fn write_network(&self, network: &CitationNetwork) -> Result<()> {
        let content = match self.format {
            OutputFormat::Json => network.to_json()?,
            OutputFormat::Bibtex => {
                // For BibTeX, write all papers in the network
                let all_papers = network.get_all_papers();
                all_papers.iter()
                    .map(|paper| format!("% Paper: {}\n% Authors: {}\n",
                        paper.title,
                        paper.authors.join(", ")))
                    .collect::<Vec<_>>()
                    .join("\n")
            }
        };
        
        self.write_content(&content).await
    }
    
    /// Format references as JSON
    fn format_json(&self, references: &[Reference]) -> Result<String> {
        Ok(serde_json::to_string_pretty(references)?)
    }
    
    /// Format references as BibTeX
    fn format_bibtex(&self, references: &[Reference]) -> String {
        references.iter()
            .map(|r| r.to_bibtex())
            .collect::<Vec<_>>()
            .join("\n")
    }
    
    /// Write content to file or stdout
    async fn write_content(&self, content: &str) -> Result<()> {
        match &self.output_path {
            Some(path) => {
                let file = File::create(path).await?;
                let mut writer = BufWriter::new(file);
                writer.write_all(content.as_bytes()).await?;
                writer.flush().await?;
                println!("Output written to: {}", path.display());
            }
            None => {
                print!("{}", content);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use tokio::fs;

    fn create_test_references() -> Vec<Reference> {
        vec![
            Reference {
                title: "First Test Paper".to_string(),
                authors: vec!["Alice Smith".to_string(), "Bob Jones".to_string()],
                arxiv_id: Some("2301.12345".to_string()),
                inspire_id: Some("123456".to_string()),
                categories: vec!["hep-th".to_string()],
                year: Some(2023),
            },
            Reference {
                title: "Second Test Paper".to_string(),
                authors: vec!["Charlie Brown".to_string()],
                arxiv_id: Some("2302.67890".to_string()),
                inspire_id: Some("789012".to_string()),
                categories: vec!["hep-ph".to_string()],
                year: Some(2023),
            },
        ]
    }

    #[test]
    fn test_format_json() {
        let writer = OutputWriter::new(OutputFormat::Json, None);
        let references = create_test_references();
        
        let json = writer.format_json(&references).unwrap();
        
        assert!(json.contains("First Test Paper"));
        assert!(json.contains("Second Test Paper"));
        assert!(json.contains("Alice Smith"));
        assert!(json.contains("2301.12345"));
        
        // Verify it's valid JSON
        let parsed: Vec<Reference> = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.len(), 2);
    }

    #[test]
    fn test_format_bibtex() {
        let writer = OutputWriter::new(OutputFormat::Bibtex, None);
        let references = create_test_references();
        
        let bibtex = writer.format_bibtex(&references);
        
        assert!(bibtex.contains("@article{"));
        assert!(bibtex.contains("First Test Paper"));
        assert!(bibtex.contains("Second Test Paper"));
        assert!(bibtex.contains("Alice Smith and Bob Jones"));
        assert!(bibtex.contains("Charlie Brown"));
        assert!(bibtex.contains("eprint = {2301.12345}"));
        assert!(bibtex.contains("eprint = {2302.67890}"));
    }

    #[tokio::test]
    async fn test_write_references_to_file() {
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("test_output.json");
        
        let writer = OutputWriter::new(OutputFormat::Json, Some(output_path.clone()));
        let references = create_test_references();
        
        writer.write_references(&references).await.unwrap();
        
        let content = fs::read_to_string(&output_path).await.unwrap();
        assert!(content.contains("First Test Paper"));
        assert!(content.contains("Second Test Paper"));
        
        // Verify it's valid JSON
        let parsed: Vec<Reference> = serde_json::from_str(&content).unwrap();
        assert_eq!(parsed.len(), 2);
    }

    #[tokio::test]
    async fn test_write_references_bibtex_to_file() {
        let temp_dir = tempdir().unwrap();
        let output_path = temp_dir.path().join("test_output.bib");
        
        let writer = OutputWriter::new(OutputFormat::Bibtex, Some(output_path.clone()));
        let references = create_test_references();
        
        writer.write_references(&references).await.unwrap();
        
        let content = fs::read_to_string(&output_path).await.unwrap();
        assert!(content.contains("@article{"));
        assert!(content.contains("First Test Paper"));
        assert!(content.contains("Second Test Paper"));
    }

    #[test]
    fn test_output_writer_creation() {
        let writer1 = OutputWriter::new(OutputFormat::Json, None);
        let writer2 = OutputWriter::new(OutputFormat::Bibtex, Some(PathBuf::from("test.bib")));
        
        // Just test that creation works without panicking
        assert!(matches!(writer1.format, OutputFormat::Json));
        assert!(matches!(writer2.format, OutputFormat::Bibtex));
        assert!(writer1.output_path.is_none());
        assert!(writer2.output_path.is_some());
    }
}
