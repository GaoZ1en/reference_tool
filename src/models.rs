use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paper {
    pub id: String,
    pub title: String,
    pub authors: Vec<String>,
    pub arxiv_id: Option<String>,
    pub categories: Vec<String>,
    pub year: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reference {
    pub title: String,
    pub authors: Vec<String>,
    pub arxiv_id: Option<String>,
    pub inspire_id: Option<String>,
    pub categories: Vec<String>,
    pub year: Option<u32>,
}

impl Reference {
    /// Generate BibTeX entry for this reference
    pub fn to_bibtex(&self) -> String {
        let key = self.generate_bibtex_key();
        let authors_str = self.authors.join(" and ");
        
        let mut bibtex = format!("@article{{{},\n", key);
        bibtex.push_str(&format!("  title = {{{}}},\n", self.title));
        
        if !authors_str.is_empty() {
            bibtex.push_str(&format!("  author = {{{}}},\n", authors_str));
        }
        
        if let Some(year) = self.year {
            bibtex.push_str(&format!("  year = {{{}}},\n", year));
        }
        
        if let Some(arxiv_id) = &self.arxiv_id {
            bibtex.push_str(&format!("  eprint = {{{}}},\n", arxiv_id));
            bibtex.push_str("  archivePrefix = {arXiv},\n");
        }
        
        if !self.categories.is_empty() {
            bibtex.push_str(&format!("  primaryClass = {{{}}},\n", self.categories[0]));
        }
        
        bibtex.push_str("}\n");
        bibtex
    }
    
    /// Generate a unique BibTeX key for this reference
    fn generate_bibtex_key(&self) -> String {
        let first_author = self.authors.first()
            .map(|name| name.split_whitespace().last().unwrap_or("Unknown"))
            .unwrap_or("Unknown");
            
        let year = self.year.map(|y| y.to_string()).unwrap_or_else(|| "YYYY".to_string());
        
        // Take first few words of title for uniqueness
        let title_words: Vec<&str> = self.title
            .split_whitespace()
            .take(2)
            .collect();
        let title_part = title_words.join("");
        
        format!("{}{}{}", first_author, year, title_part)
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paper_creation() {
        let paper = Paper {
            id: "123456".to_string(),
            title: "Test Paper".to_string(),
            authors: vec!["John Doe".to_string(), "Jane Smith".to_string()],
            arxiv_id: Some("2301.12345".to_string()),
            categories: vec!["hep-th".to_string(), "hep-ph".to_string()],
            year: Some(2023),
        };

        assert_eq!(paper.id, "123456");
        assert_eq!(paper.title, "Test Paper");
        assert_eq!(paper.authors.len(), 2);
        assert_eq!(paper.arxiv_id, Some("2301.12345".to_string()));
        assert_eq!(paper.categories.len(), 2);
        assert_eq!(paper.year, Some(2023));
    }

    #[test]
    fn test_reference_to_bibtex() {
        let reference = Reference {
            title: "A Study of Quantum Field Theory".to_string(),
            authors: vec!["John Doe".to_string(), "Jane Smith".to_string()],
            arxiv_id: Some("2301.12345".to_string()),
            inspire_id: Some("789012".to_string()),
            categories: vec!["hep-th".to_string()],
            year: Some(2023),
        };

        let bibtex = reference.to_bibtex();
        
        assert!(bibtex.contains("@article{"));
        assert!(bibtex.contains("title = {A Study of Quantum Field Theory}"));
        assert!(bibtex.contains("author = {John Doe and Jane Smith}"));
        assert!(bibtex.contains("year = {2023}"));
        assert!(bibtex.contains("eprint = {2301.12345}"));
        assert!(bibtex.contains("archivePrefix = {arXiv}"));
        assert!(bibtex.contains("primaryClass = {hep-th}"));
    }

    #[test]
    fn test_reference_to_bibtex_minimal() {
        let reference = Reference {
            title: "Minimal Reference".to_string(),
            authors: vec![],
            arxiv_id: None,
            inspire_id: None,
            categories: vec![],
            year: None,
        };

        let bibtex = reference.to_bibtex();
        
        assert!(bibtex.contains("@article{"));
        assert!(bibtex.contains("title = {Minimal Reference}"));
        assert!(!bibtex.contains("author ="));
        assert!(!bibtex.contains("year ="));
        assert!(!bibtex.contains("eprint ="));
    }

    #[test]
    fn test_generate_bibtex_key() {
        let reference = Reference {
            title: "Quantum Field Theory in Curved Spacetime".to_string(),
            authors: vec!["John von Doe".to_string()],
            arxiv_id: None,
            inspire_id: None,
            categories: vec![],
            year: Some(2023),
        };

        let key = reference.generate_bibtex_key();
        assert!(key.contains("Doe"));
        assert!(key.contains("2023"));
        assert!(key.contains("Quantum"));
        assert!(key.chars().all(|c| c.is_alphanumeric()));
    }

    #[test]
    fn test_generate_bibtex_key_no_author() {
        let reference = Reference {
            title: "Anonymous Paper".to_string(),
            authors: vec![],
            arxiv_id: None,
            inspire_id: None,
            categories: vec![],
            year: Some(2023),
        };

        let key = reference.generate_bibtex_key();
        assert!(key.contains("Unknown"));
        assert!(key.contains("2023"));
        assert!(key.contains("Anonymous"));
    }

    #[test]
    fn test_serialize_deserialize() {
        let paper = Paper {
            id: "123456".to_string(),
            title: "Test Paper".to_string(),
            authors: vec!["John Doe".to_string()],
            arxiv_id: Some("2301.12345".to_string()),
            categories: vec!["hep-th".to_string()],
            year: Some(2023),
        };

        let json = serde_json::to_string(&paper).unwrap();
        let deserialized: Paper = serde_json::from_str(&json).unwrap();
        
        assert_eq!(paper.id, deserialized.id);
        assert_eq!(paper.title, deserialized.title);
        assert_eq!(paper.authors, deserialized.authors);
        assert_eq!(paper.arxiv_id, deserialized.arxiv_id);
        assert_eq!(paper.categories, deserialized.categories);
        assert_eq!(paper.year, deserialized.year);
    }
}
