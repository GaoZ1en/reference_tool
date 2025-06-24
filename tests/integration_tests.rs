use reference_tool::models::{Paper, Reference};
use reference_tool::output::{OutputWriter, OutputFormat};
use reference_tool::network::CitationNetwork;
use tempfile::tempdir;
use tokio::fs;

#[tokio::test]
async fn test_json_output_integration() {
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("integration_test.json");
    
    let references = vec![
        Reference {
            title: "Integration Test Paper".to_string(),
            authors: vec!["Test Author".to_string()],
            arxiv_id: Some("test.12345".to_string()),
            inspire_id: Some("123456".to_string()),
            categories: vec!["hep-th".to_string()],
            year: Some(2023),
        }
    ];
    
    let writer = OutputWriter::new(OutputFormat::Json, Some(output_path.clone()));
    writer.write_references(&references).await.unwrap();
    
    let content = fs::read_to_string(&output_path).await.unwrap();
    let parsed: Vec<Reference> = serde_json::from_str(&content).unwrap();
    
    assert_eq!(parsed.len(), 1);
    assert_eq!(parsed[0].title, "Integration Test Paper");
    assert_eq!(parsed[0].arxiv_id, Some("test.12345".to_string()));
}

#[tokio::test]
async fn test_bibtex_output_integration() {
    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("integration_test.bib");
    
    let references = vec![
        Reference {
            title: "BibTeX Integration Test".to_string(),
            authors: vec!["John Doe".to_string(), "Jane Smith".to_string()],
            arxiv_id: Some("test.67890".to_string()),
            inspire_id: Some("789012".to_string()),
            categories: vec!["hep-ph".to_string()],
            year: Some(2023),
        }
    ];
    
    let writer = OutputWriter::new(OutputFormat::Bibtex, Some(output_path.clone()));
    writer.write_references(&references).await.unwrap();
    
    let content = fs::read_to_string(&output_path).await.unwrap();
    
    assert!(content.contains("@article{"));
    assert!(content.contains("BibTeX Integration Test"));
    assert!(content.contains("John Doe and Jane Smith"));
    assert!(content.contains("eprint = {test.67890}"));
    assert!(content.contains("year = {2023}"));
}

#[test]
fn test_citation_network_integration() {
    let mut network = CitationNetwork::new();
    
    let paper1 = Paper {
        id: "1".to_string(),
        title: "Root Paper".to_string(),
        authors: vec!["Author One".to_string()],
        arxiv_id: Some("root.12345".to_string()),
        categories: vec!["hep-th".to_string()],
        year: Some(2023),
    };
    
    let paper2 = Paper {
        id: "2".to_string(),
        title: "Referenced Paper".to_string(),
        authors: vec!["Author Two".to_string()],
        arxiv_id: Some("ref.67890".to_string()),
        categories: vec!["hep-ph".to_string()],
        year: Some(2022),
    };
    
    network.add_paper(paper1);
    network.add_paper(paper2);
    network.add_citation("1", "2");
    
    let stats = network.get_stats();
    assert_eq!(stats.total_papers, 2);
    assert_eq!(stats.total_citations, 1);
    assert_eq!(stats.papers_with_references, 1);
    assert_eq!(stats.papers_being_cited, 1);
    
    let citing_papers = network.get_citing_papers("2");
    assert_eq!(citing_papers.len(), 1);
    assert_eq!(citing_papers[0].title, "Root Paper");
    
    let cited_papers = network.get_cited_papers("1");
    assert_eq!(cited_papers.len(), 1);
    assert_eq!(cited_papers[0].title, "Referenced Paper");
}

#[test]
fn test_reference_bibtex_generation() {
    let reference = Reference {
        title: "A Comprehensive Study of Quantum Field Theory".to_string(),
        authors: vec![
            "Albert Einstein".to_string(),
            "Niels Bohr".to_string(),
            "Werner Heisenberg".to_string(),
        ],
        arxiv_id: Some("quant-ph/9901001".to_string()),
        inspire_id: Some("999999".to_string()),
        categories: vec!["quant-ph".to_string(), "hep-th".to_string()],
        year: Some(1999),
    };
    
    let bibtex = reference.to_bibtex();
    
    // Check structure
    assert!(bibtex.starts_with("@article{"));
    assert!(bibtex.ends_with("}\n"));
    
    // Check required fields
    assert!(bibtex.contains("title = {A Comprehensive Study of Quantum Field Theory}"));
    assert!(bibtex.contains("author = {Albert Einstein and Niels Bohr and Werner Heisenberg}"));
    assert!(bibtex.contains("year = {1999}"));
    assert!(bibtex.contains("eprint = {quant-ph/9901001}"));
    assert!(bibtex.contains("archivePrefix = {arXiv}"));
    assert!(bibtex.contains("primaryClass = {quant-ph}"));
    
    // Verify the key is reasonable
    let lines: Vec<&str> = bibtex.lines().collect();
    let first_line = lines[0];
    assert!(first_line.contains("Einstein"));
    assert!(first_line.contains("1999"));
}
