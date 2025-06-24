use reqwest::Client;
use serde_json::Value;
use anyhow::{Result, anyhow};
use log::{debug, info};

use crate::models::{Paper, Reference};

pub struct InspireClient {
    client: Client,
    base_url: String,
}

impl InspireClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://inspirehep.net/api".to_string(),
        }
    }
    
    /// Get paper information by ArXiv ID
    pub async fn get_paper_by_arxiv(&self, arxiv_id: &str) -> Result<Paper> {
        let url = format!("{}/literature", self.base_url);
        let query = format!("arxiv:{}", arxiv_id);
        
        debug!("Searching for paper with query: {}", query);
        
        let response = self.client
            .get(&url)
            .query(&[("q", query.as_str()), ("size", "1")])
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch paper: {}", response.status()));
        }
        
        let json: Value = response.json().await?;
        let hits = json["hits"]["hits"].as_array()
            .ok_or_else(|| anyhow!("Invalid response format"))?;
            
        if hits.is_empty() {
            return Err(anyhow!("Paper not found with ArXiv ID: {}", arxiv_id));
        }
        
        let paper_data = &hits[0]["metadata"];
        self.parse_paper(paper_data)
    }
    
    /// Get references for a paper by its INSPIRE ID
    pub async fn get_paper_references(&self, paper_id: &str) -> Result<Vec<Reference>> {
        let url = format!("{}/literature/{}", self.base_url, paper_id);
        
        debug!("Fetching paper details for ID: {}", paper_id);
        
        let response = self.client
            .get(&url)
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch paper details: {}", response.status()));
        }
        
        let json: Value = response.json().await?;
        let empty_vec = vec![];
        let references = json["metadata"]["references"].as_array()
            .unwrap_or(&empty_vec);
            
        info!("Found {} references", references.len());
        
        let mut refs = Vec::new();
        for reference in references.iter() {
            if let Ok(parsed_ref) = self.parse_reference(reference) {
                refs.push(parsed_ref);
            }
        }
        
        Ok(refs)
    }
    
    /// Parse paper data from INSPIRE API response
    fn parse_paper(&self, data: &Value) -> Result<Paper> {
        let id = data["control_number"].as_u64()
            .ok_or_else(|| anyhow!("Missing control number"))?
            .to_string();
            
        let title = data["titles"][0]["title"].as_str()
            .unwrap_or("Unknown Title")
            .to_string();
            
        let authors = data["authors"].as_array()
            .map(|authors| {
                authors.iter()
                    .filter_map(|author| author["full_name"].as_str())
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();
            
        let arxiv_id = data["arxiv_eprints"].as_array()
            .and_then(|eprints| eprints.first())
            .and_then(|eprint| eprint["value"].as_str())
            .map(|s| s.to_string());
            
        let categories = data["inspire_categories"].as_array()
            .map(|cats| {
                cats.iter()
                    .filter_map(|cat| cat["term"].as_str())
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();
            
        let year = data["preprint_date"].as_str()
            .or_else(|| data["imprints"][0]["date"].as_str())
            .and_then(|date| date.split('-').next())
            .and_then(|year_str| year_str.parse().ok());
            
        Ok(Paper {
            id,
            title,
            authors,
            arxiv_id,
            categories,
            year,
        })
    }
    
    /// Parse reference data from INSPIRE API response
    fn parse_reference(&self, data: &Value) -> Result<Reference> {
        let title = data["reference"]["title"]["title"].as_str()
            .unwrap_or("Unknown Title")
            .to_string();
            
        let authors = data["reference"]["authors"].as_array()
            .map(|authors| {
                authors.iter()
                    .filter_map(|author| author["full_name"].as_str())
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();
            
        let arxiv_id = data["reference"]["arxiv_eprint"].as_str()
            .map(|s| s.to_string());
            
        let inspire_id = data["record"]["$ref"].as_str()
            .and_then(|url| url.split('/').last())
            .map(|s| s.to_string());
            
        let categories = data["reference"]["inspire_categories"].as_array()
            .map(|cats| {
                cats.iter()
                    .filter_map(|cat| cat["term"].as_str())
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();
            
        let year = data["reference"]["imprint"]["date"].as_str()
            .and_then(|date| date.split('-').next())
            .and_then(|year_str| year_str.parse().ok());
            
        Ok(Reference {
            title,
            authors,
            arxiv_id,
            inspire_id,
            categories,
            year,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_paper() {
        let client = InspireClient::new();
        let paper_data = json!({
            "control_number": 123456,
            "titles": [{"title": "Test Paper Title"}],
            "authors": [
                {"full_name": "John Doe"},
                {"full_name": "Jane Smith"}
            ],
            "arxiv_eprints": [{"value": "2301.12345"}],
            "inspire_categories": [
                {"term": "hep-th"},
                {"term": "hep-ph"}
            ],
            "preprint_date": "2023-01-15"
        });

        let paper = client.parse_paper(&paper_data).unwrap();
        
        assert_eq!(paper.id, "123456");
        assert_eq!(paper.title, "Test Paper Title");
        assert_eq!(paper.authors, vec!["John Doe", "Jane Smith"]);
        assert_eq!(paper.arxiv_id, Some("2301.12345".to_string()));
        assert_eq!(paper.categories, vec!["hep-th", "hep-ph"]);
        assert_eq!(paper.year, Some(2023));
    }

    #[test]
    fn test_parse_reference() {
        let client = InspireClient::new();
        let ref_data = json!({
            "reference": {
                "title": {"title": "Reference Paper"},
                "authors": [{"full_name": "Alice Cooper"}],
                "arxiv_eprint": "1234.5678",
                "inspire_categories": [{"term": "hep-ex"}],
                "imprint": {"date": "2022-05-10"}
            },
            "record": {"$ref": "https://inspirehep.net/api/literature/789012"}
        });

        let reference = client.parse_reference(&ref_data).unwrap();
        
        assert_eq!(reference.title, "Reference Paper");
        assert_eq!(reference.authors, vec!["Alice Cooper"]);
        assert_eq!(reference.arxiv_id, Some("1234.5678".to_string()));
        assert_eq!(reference.inspire_id, Some("789012".to_string()));
        assert_eq!(reference.categories, vec!["hep-ex"]);
        assert_eq!(reference.year, Some(2022));
    }

    #[test]
    fn test_parse_paper_minimal_data() {
        let client = InspireClient::new();
        let paper_data = json!({
            "control_number": 654321,
            "titles": [{"title": "Minimal Paper"}]
        });

        let paper = client.parse_paper(&paper_data).unwrap();
        
        assert_eq!(paper.id, "654321");
        assert_eq!(paper.title, "Minimal Paper");
        assert!(paper.authors.is_empty());
        assert_eq!(paper.arxiv_id, None);
        assert!(paper.categories.is_empty());
        assert_eq!(paper.year, None);
    }

    #[test]
    fn test_parse_paper_missing_control_number() {
        let client = InspireClient::new();
        let paper_data = json!({
            "titles": [{"title": "Paper without ID"}]
        });

        let result = client.parse_paper(&paper_data);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Missing control number"));
    }
}
