# CorneaDeSci

<img src="assets/logo.svg" width="200" height="200" alt="CorneaDeSci Logo" style="display: block; margin: 0 auto;">

## Decentralized Corneal Health Research Platform

CorneaDeSci is an open-source platform that combines blockchain technology with ophthalmology research to create a decentralized ecosystem for corneal health data management and research collaboration.

> Note: This codebase is maintained in English to ensure global accessibility and collaboration.

### Key Features

- **Secure Data Management**: Store and manage corneal health data with blockchain-based security
- **Research Collaboration**: Connect researchers, clinicians, and patients in a decentralized network
- **Token Economy**: Incentivize participation and data sharing through a transparent token system
- **Smart Contracts**: Automate research agreements and data usage permissions
- **Patient Control**: Give patients sovereignty over their health data

### Technology Stack

- Rust backend with Actix-web framework
- Ethereum blockchain integration
- PostgreSQL database for structured data
- IPFS for decentralized storage
- Web3 connectivity

### Getting Started

```bash
# Clone the repository
git clone https://github.com/CorneaDeSci/CorneaDeSci.git
cd CorneaDeSci

# Install dependencies
cargo build

# Run the development server
cargo run
```

### Project Structure

```
CorneaDeSci/
├── src/              # Source code
│   ├── api/          # API endpoints
│   ├── blockchain/   # Blockchain integration
│   ├── database/     # Database models and connections
│   ├── services/     # Business logic services
│   ├── config.rs     # Configuration management
│   ├── error.rs      # Error handling
│   ├── lib.rs        # Library exports
│   └── main.rs       # Application entry point
├── migrations/       # Database migrations
├── assets/           # Static assets
└── Cargo.toml        # Project dependencies
```

### Contributing

We welcome contributions from developers, researchers, and healthcare professionals.

### License

This project is licensed under the MIT License - see the LICENSE file for details.

### Contact

For more information, reach out to us at info@corneadesci.io
