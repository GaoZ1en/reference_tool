# Copilot Instructions

<!-- Use this file to provide workspace-specific custom instructions to Copilot. For more details, visit https://code.visualstudio.com/docs/copilot/copilot-customization#_use-a-githubcopilotinstructionsmd-file -->

This is a Rust command-line tool for fetching paper citations via the INSPIRE-HEP API.

## Project Structure

- `src/main.rs` - Main application entry point with CLI parsing
- `src/api.rs` - INSPIRE-HEP API client implementation
- `src/models.rs` - Data structures for papers and references
- `src/output.rs` - Output formatting (JSON, BibTeX)
- `src/network.rs` - Citation network building and analysis

## Key Features

- Fetch paper references by ArXiv ID
- Support for JSON and BibTeX output formats
- Filter references by subject categories (hep-th, hep-ph, etc.)
- Build citation networks with configurable depth
- Comprehensive error handling with `anyhow` and `thiserror`

## Dependencies

- `clap` - Command-line argument parsing
- `tokio` - Async runtime
- `reqwest` - HTTP client for API requests
- `serde` - Serialization/deserialization
- `anyhow` - Error handling
- `log` - Logging

## Usage Examples

```bash
# Get references for a paper
cargo run -- --arxiv-id 2301.12345 --format json --output refs.json

# Filter by categories
cargo run -- --arxiv-id 2301.12345 --categories "hep-th,hep-ph" --format bibtex

# Build citation network
cargo run -- network --arxiv-id 2301.12345 --build-network --depth 2
```

## API Integration

The tool uses the INSPIRE-HEP REST API (https://inspirehep.net/api) to:
- Search for papers by ArXiv ID
- Retrieve paper metadata and references
- Access bibliographic information

## Code Style

- Use async/await for all I/O operations
- Implement proper error handling with descriptive error messages
- Follow Rust naming conventions and idiomatic patterns
- Use structured logging with different verbosity levels
