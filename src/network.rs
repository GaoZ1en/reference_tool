use std::collections::{HashMap, HashSet};
use anyhow::Result;
use serde::{Serialize, Deserialize};
use log::{info, debug};

use crate::api::InspireClient;
use crate::models::Paper;

#[derive(Debug, Serialize, Deserialize)]
pub struct CitationNetwork {
    pub papers: HashMap<String, Paper>,
    pub citations: HashMap<String, Vec<String>>, // paper_id -> [referenced_paper_ids]
    pub reverse_citations: HashMap<String, Vec<String>>, // paper_id -> [citing_paper_ids]
}

impl CitationNetwork {
    pub fn new() -> Self {
        Self {
            papers: HashMap::new(),
            citations: HashMap::new(),
            reverse_citations: HashMap::new(),
        }
    }
    
    /// Build citation network starting from a paper with given depth
    pub async fn build(&mut self, client: &InspireClient, arxiv_id: &str, depth: u32) -> Result<()> {
        let mut to_process = Vec::new();
        let mut processed = HashSet::new();
        
        // Get the root paper
        let root_paper = client.get_paper_by_arxiv(arxiv_id).await?;
        let root_id = root_paper.id.clone();
        
        info!("Starting network build from paper: {}", root_paper.title);
        println!("📄 Root paper: {}", root_paper.title);
        
        self.add_paper(root_paper);
        to_process.push((root_id.clone(), 0));
        
        let mut processed_count = 0;
        
        while let Some((paper_id, current_depth)) = to_process.pop() {
            if processed.contains(&paper_id) || current_depth >= depth {
                continue;
            }
            
            processed.insert(paper_id.clone());
            processed_count += 1;
            
            debug!("Processing paper at depth {}: {}", current_depth, paper_id);
            println!("🔍 Processing depth {} (paper {})", current_depth, processed_count);
            
            // Get references for this paper
            match client.get_paper_references(&paper_id).await {
                Ok(references) => {
                    let mut ref_ids = Vec::new();
                    
                    for reference in references {
                        // Try to find the paper in INSPIRE if we have an ID
                        if let Some(inspire_id) = &reference.inspire_id {
                            // Convert reference to paper (simplified)
                            let ref_paper = Paper {
                                id: inspire_id.clone(),
                                title: reference.title.clone(),
                                authors: reference.authors.clone(),
                                arxiv_id: reference.arxiv_id.clone(),
                                categories: reference.categories.clone(),
                                year: reference.year,
                            };
                            
                            self.add_paper(ref_paper);
                            ref_ids.push(inspire_id.clone());
                            
                            // Add to processing queue for next depth level
                            if current_depth + 1 < depth {
                                to_process.push((inspire_id.clone(), current_depth + 1));
                            }
                        }
                    }
                    
                    self.add_citations(&paper_id, ref_ids);
                }
                Err(e) => {
                    debug!("Failed to get references for {}: {}", paper_id, e);
                }
            }
        }
        
        println!("✅ Network build complete! {} papers processed", self.papers.len());
        info!("Network build complete. {} papers processed.", self.papers.len());
        Ok(())
    }
    
    /// Add a paper to the network
    pub fn add_paper(&mut self, paper: Paper) {
        self.papers.insert(paper.id.clone(), paper);
    }
    
    /// Add citation relationships
    pub fn add_citations(&mut self, citing_paper_id: &str, referenced_paper_ids: Vec<String>) {
        self.citations.insert(citing_paper_id.to_string(), referenced_paper_ids.clone());
        
        // Update reverse citations
        for ref_id in referenced_paper_ids {
            self.reverse_citations
                .entry(ref_id)
                .or_insert_with(Vec::new)
                .push(citing_paper_id.to_string());
        }
    }
    
    /// Get the number of papers in the network
    pub fn paper_count(&self) -> usize {
        self.papers.len()
    }
    
    /// Get all papers in the network
    pub fn get_all_papers(&self) -> Vec<&Paper> {
        self.papers.values().collect()
    }
    
    /// Convert network to JSON string
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Paper;

    fn create_test_paper(id: &str, title: &str, arxiv_id: Option<&str>) -> Paper {
        Paper {
            id: id.to_string(),
            title: title.to_string(),
            authors: vec!["Test Author".to_string()],
            arxiv_id: arxiv_id.map(|s| s.to_string()),
            categories: vec!["hep-th".to_string()],
            year: Some(2023),
        }
    }

    #[test]
    fn test_new_network() {
        let network = CitationNetwork::new();
        assert!(network.papers.is_empty());
        assert!(network.citations.is_empty());
        assert!(network.reverse_citations.is_empty());
    }

    #[test]
    fn test_add_paper() {
        let mut network = CitationNetwork::new();
        let paper = create_test_paper("123", "Test Paper", Some("2301.12345"));
        
        network.add_paper(paper.clone());
        
        assert_eq!(network.papers.len(), 1);
        assert!(network.papers.contains_key("123"));
        assert_eq!(network.papers.get("123").unwrap().title, "Test Paper");
    }

    #[test]
    fn test_get_all_papers() {
        let mut network = CitationNetwork::new();
        let paper1 = create_test_paper("123", "Paper 1", Some("2301.12345"));
        let paper2 = create_test_paper("456", "Paper 2", Some("2301.67890"));
        
        network.add_paper(paper1);
        network.add_paper(paper2);
        
        let all_papers = network.get_all_papers();
        assert_eq!(all_papers.len(), 2);
        
        let titles: Vec<&String> = all_papers.iter().map(|p| &p.title).collect();
        assert!(titles.contains(&&"Paper 1".to_string()));
        assert!(titles.contains(&&"Paper 2".to_string()));
    }

    #[test]
    fn test_to_json() {
        let mut network = CitationNetwork::new();
        let paper = create_test_paper("123", "Test Paper", Some("2301.12345"));
        
        network.add_paper(paper);
        
        let json = network.to_json().unwrap();
        assert!(json.contains("Test Paper"));
        assert!(json.contains("2301.12345"));
        assert!(json.contains("papers"));
        assert!(json.contains("citations"));
        assert!(json.contains("reverse_citations"));
        
        // Verify it's valid JSON
        let parsed: CitationNetwork = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.papers.len(), 1);
    }
}
