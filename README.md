# Reference Tool

A Rust command-line tool for fetching paper citations via the INSPIRE-HEP API.

## Features

- 🔍 **Fetch References**: Get all references for any paper by ArXiv ID
- 📄 **Multiple Output Formats**: Support for JSON and BibTeX output
- 🏷️ **Category Filtering**: Filter references by subject categories (hep-th, hep-ph, etc.)
- 🌐 **Citation Networks**: Build and analyze citation networks with configurable depth
- ⚙️ **Configuration Support**: Customizable settings via TOML configuration file
- 📊 **Comprehensive Logging**: Detailed logging with configurable verbosity levels

## Installation

### Prerequisites

- Rust 1.70 or later
- Internet connection (for API access)

### Build from Source

```bash
git clone https://github.com/yourusername/reference_tool.git
cd reference_tool
cargo build --release
```

The compiled binary will be available at `target/release/reference_tool`.

## Usage

### Basic Usage

Get references for a paper:

```bash
# Fetch references in JSON format
reference_tool --arxiv-id hep-th/9905104 --format json --output references.json

# Fetch references in BibTeX format
reference_tool --arxiv-id hep-th/9905104 --format bibtex --output references.bib

# Filter by categories
reference_tool --arxiv-id hep-th/9905104 --categories "hep-th,hep-ph"
```

### Citation Networks

Build citation networks to analyze paper relationships:

```bash
# Build a citation network with depth 2
reference_tool network --arxiv-id hep-th/9905104 --build-network --depth 2

# Output network data
reference_tool network --arxiv-id hep-th/9905104 --build-network --depth 1 --format json --output network.json
```

### Configuration Management

```bash
# Initialize configuration file
reference_tool init-config

# Show current configuration
reference_tool config
```

### Command-Line Options

```
Usage: reference_tool [OPTIONS] [COMMAND]

Commands:
  network      Build citation network
  config       Show current configuration
  init-config  Initialize configuration file
  help         Print this message or the help of the given subcommand(s)

Options:
      --arxiv-id <ARXIV_ID>      ArXiv ID of the paper
      --format <FORMAT>          Output format [default: json] [possible values: json, bibtex]
      --output <OUTPUT>          Output file path
      --categories <CATEGORIES>  Categories to filter (comma-separated)
  -v, --verbose                  Enable verbose logging
  -h, --help                     Print help
  -V, --version                  Print version
```

## Configuration

The tool supports configuration via a TOML file located at:
- Linux/macOS: `~/.config/reference_tool/config.toml`
- Windows: `%APPDATA%\reference_tool\config.toml`

### Configuration Options

```toml
# Default output format
default_format = "json"

# Default output directory
default_output_dir = "./output"

# Default categories to filter
default_categories = ["hep-th", "hep-ph"]

# Enable verbose logging by default
verbose = false

# Default network depth
default_network_depth = 1

[api]
# INSPIRE API base URL
base_url = "https://inspirehep.net/api"

# Request timeout in seconds
timeout_seconds = 30

# Maximum retries for failed requests
max_retries = 3

# Delay between requests (in milliseconds)
request_delay_ms = 100

[ui]
# Show progress bars
show_progress = true

# Use colored output
use_colors = true
```

## Examples

### Example 1: Basic Reference Fetching

```bash
# Get references for a famous AdS/CFT paper
reference_tool --arxiv-id hep-th/9711200 --format json --output ads_cft_refs.json
```

### Example 2: Category Filtering

```bash
# Get only high-energy theory references
reference_tool --arxiv-id hep-th/9905104 --categories "hep-th" --format bibtex
```

### Example 3: Building Citation Networks

```bash
# Build a 2-level citation network
reference_tool network --arxiv-id hep-th/9905104 --build-network --depth 2 --output network.json
```

## Output Formats

### JSON Format

```json
[
  {
    "title": "The Large N limit of superconformal field theories and supergravity",
    "authors": ["Juan Maldacena"],
    "arxiv_id": "hep-th/9711200",
    "inspire_id": "451647",
    "categories": ["hep-th"],
    "year": 1997
  }
]
```

### BibTeX Format

```bibtex
@article{Maldacena1997,
    title = {The Large N limit of superconformal field theories and supergravity},
    author = {Juan Maldacena},
    eprint = {hep-th/9711200},
    year = {1997},
    journal = {Adv. Theor. Math. Phys.}
}
```

## API Integration

This tool uses the [INSPIRE-HEP REST API](https://inspirehep.net/api) to:

- Search for papers by ArXiv ID
- Retrieve paper metadata and references
- Access bibliographic information

The API is free and doesn't require authentication, but please be respectful with request rates.

## Development

### Project Structure

```
src/
├── main.rs          # Main application entry point
├── api.rs           # INSPIRE-HEP API client
├── models.rs        # Data structures for papers and references
├── output.rs        # Output formatting (JSON, BibTeX)
├── network.rs       # Citation network building and analysis
└── config.rs        # Configuration management
```

### Running Tests

```bash
# Run unit tests
cargo test

# Run integration tests
cargo test --test integration_tests

# Run with verbose output
cargo test -- --nocapture
```

### Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Dependencies

- **clap**: Command-line argument parsing
- **tokio**: Async runtime
- **reqwest**: HTTP client for API requests
- **serde**: Serialization/deserialization
- **anyhow**: Error handling
- **log** & **env_logger**: Logging

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Thanks to the [INSPIRE-HEP](https://inspirehep.net/) team for providing the excellent API
- Built with ❤️ using Rust

## Troubleshooting

### Common Issues

1. **Network Connection**: Ensure you have internet access to reach the INSPIRE API
2. **Invalid ArXiv IDs**: Make sure to use the correct ArXiv ID format (e.g., `hep-th/9905104` or `2301.12345`)
3. **Rate Limiting**: If you encounter rate limits, increase the `request_delay_ms` in the configuration

### Getting Help

If you encounter issues:

1. Check the [Issues](https://github.com/yourusername/reference_tool/issues) page
2. Run with `--verbose` flag for detailed logging
3. Create a new issue with the error details and steps to reproduce

---

**Note**: Replace `yourusername` in the GitHub URLs with your actual GitHub username when creating the repository.
