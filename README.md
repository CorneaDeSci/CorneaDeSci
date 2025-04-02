# CorneaDeSci

<div align="center">
  <img src="https://raw.githubusercontent.com/CorneaDeSci/CorneaDeSci/main/assets/logo.jpg" width="300" height="300" alt="CorneaDeSci Logo">
</div>

<div align="center">

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![GitHub issues](https://img.shields.io/github/issues/CorneaDeSci/CorneaDeSci)](https://github.com/CorneaDeSci/CorneaDeSci/issues)
[![GitHub stars](https://img.shields.io/github/stars/CorneaDeSci/CorneaDeSci)](https://github.com/CorneaDeSci/CorneaDeSci/stargazers)
[![Build Status](https://img.shields.io/github/workflow/status/CorneaDeSci/CorneaDeSci/CI)](https://github.com/CorneaDeSci/CorneaDeSci/actions)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://corneadesci.github.io/docs)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/CorneaDeSci/CorneaDeSci/pulls)

</div>

A decentralized corneal health research platform built with Rust and blockchain technology.

## Overview

CorneaDeSci is a decentralized platform that enables researchers and healthcare providers to collaborate on corneal health research. The platform provides secure data sharing, research collaboration tools, and blockchain-based verification of research findings.

## Features

- Decentralized data storage and sharing
- Blockchain-based research verification
- Secure authentication and authorization
- Research collaboration tools
- Data analysis and visualization
- API for third-party integrations

## Technology Stack

- Backend: Rust
- Database: PostgreSQL
- Blockchain: Substrate
- Authentication: JWT
- API: RESTful

## Getting Started

### Prerequisites

- Rust 1.70 or later
- PostgreSQL 14 or later
- Node.js 18 or later (for frontend)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/CorneaDeSci/CorneaDeSci.git
cd CorneaDeSci
```

2. Install dependencies:
```bash
cargo build
```

3. Set up the database:
```bash
cargo run --bin migrate
```

4. Start the server:
```bash
cargo run
```

## Project Structure

```
cornea_new/
├── src/
│   ├── api/           # API endpoints and handlers
│   ├── blockchain/    # Blockchain integration
│   ├── database/      # Database models and migrations
│   ├── services/      # Business logic
│   └── utils/         # Utility functions
├── assets/           # Static assets
├── migrations/       # Database migrations
└── tests/           # Test files
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contact

For any questions or concerns, please open an issue in the GitHub repository.
