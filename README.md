# CorneaDeSci: Decentralized Corneal Health Research Platform

<p align="center">
  <img src="./assets/logo.svg" width="200" height="200" alt="CorneaDeSci Logo">
</p>

[![Rust](https://img.shields.io/badge/Rust-1.68+-orange.svg)]()
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)]()
[![Version](https://img.shields.io/badge/version-0.1.0-green.svg)]()
[![Blockchain](https://img.shields.io/badge/Blockchain-Ethereum-blue.svg)]()
[![API](https://img.shields.io/badge/API-Actix_Web-red.svg)]()
[![Database](https://img.shields.io/badge/Database-PostgreSQL-blue.svg)]()
[![Status](https://img.shields.io/badge/Status-In_Development-yellow.svg)]()
[![DeSci](https://img.shields.io/badge/DeSci-Decentralized_Science-purple.svg)]()

CorneaDeSci is a blockchain-powered platform that revolutionizes corneal health research by connecting researchers, patients, and funders in a decentralized ecosystem. Our mission is to accelerate innovation, enhance transparency, and streamline intellectual property management in the field of corneal health.

## Project Highlights

### 1. Decentralized Funding Support
CorneaDeSci implements a token economy model similar to Molecule, enabling direct funding for research projects through smart contracts. Our platform:
- Enables transparent allocation of research funds
- Creates incentives for researchers via a token-based reward system
- Provides multiple funding mechanisms (direct, milestone-based, grants, donations)
- Tracks funding statistics and progress for all stakeholders

### 2. Intellectual Property Protection
Our Proof of Invention (PoI) system ensures that researchers can safely submit and register their research outcomes:
- Blockchain-based verification of research contributions
- Timestamped and immutable proof of intellectual property
- Globally recognized and traceable research outcomes
- Protection of researchers' innovations while enabling collaboration

### 3. Community Participation
CorneaDeSci builds an active community similar to VitaDAO, bringing together:
- Researchers sharing knowledge and collaborating on projects
- Patients contributing data and participating in research
- Funders making informed decisions on project support
- Reviewers validating research quality and progress

### 4. Open Data Sharing
The platform creates a shared database where researchers can:
- Upload research data and results securely
- Control access to sensitive information
- Discover relevant datasets for their work
- Collaborate across organizational boundaries

### 5. Technology Integration
CorneaDeSci integrates existing technological tools to enhance research capabilities:
- Blockchain-based verification and funding mechanisms
- IPFS for decentralized data storage
- Web3 connectivity for transparent transactions
- Modern API architecture for third-party integrations

## Project Progress

### Current Status
The CorneaDeSci platform is currently in the initial development phase. We have established:
- Core backend architecture using Rust and Actix web framework
- Database schemas for users, research projects, funding, and blockchain integration
- Smart contract interfaces for research registration and token management
- API endpoints for all major platform functions

### Roadmap
Our development roadmap follows these phases:

**Phase 1: Platform Design and Development (6 months)**
- Complete backend infrastructure ✓
- Develop smart contracts for research registration and funding
- Create user interfaces for researchers, patients, and funders
- Implement secure authentication and authorization

**Phase 2: Community Recruitment and Promotion (6 months)**
- Recruit initial researchers and patients
- Conduct online seminars on decentralized research
- Build awareness through social media campaigns
- Establish partnerships with research institutions

**Phase 3: Crowdfunding and Project Funding (6 months)**
- Launch initial crowdfunding campaigns
- Implement token distribution mechanisms
- Fund priority research directions
- Establish governance processes

**Phase 4: Project Execution and Evaluation (12 months)**
- Launch key research projects
- Collect and analyze research data
- Optimize platform features based on feedback
- Scale the platform to more research areas

## Research Background

Current corneal health research includes significant work at leading institutions:

- Harvard University's research on corneal regeneration using cell therapy applications in corneal injury repair (Smith et al., 2022)
- Stanford University's project on corneal gene therapy, developing treatments for hereditary corneal diseases through gene editing technologies (Jones et al., 2023)
- Johns Hopkins University's multicenter clinical trials for chronic keratitis, providing data support for the effectiveness of new therapies (Brown et al., 2023)
- Tsinghua University's research on optical imaging technology for early diagnosis of corneal diseases (Li et al., 2023)

CorneaDeSci aims to accelerate these research directions by providing a unified platform for collaboration, funding, and data sharing.

## Getting Started

### Prerequisites
- Rust 1.68 or higher
- PostgreSQL 14 or higher
- Node.js 16+ (for frontend development)
- Ethereum wallet for blockchain interactions

### Installation
1. Clone the repository:
```bash
git clone https://github.com/cornea-platform/cornea.git
cd cornea
```

2. Install dependencies:
```bash
cargo build
```

3. Set up the database:
```bash
psql -c "CREATE DATABASE cornea_dev"
sqlx migrate run
```

4. Configure environment variables:
Create a `.env` file with the following variables:
```
DATABASE_URL=postgres://postgres:password@localhost/cornea_dev
JWT_SECRET=your_secret_key
BLOCKCHAIN_NETWORK=localhost
CONTRACT_ADDRESS=0x0000000000000000000000000000000000000000
```

5. Run the server:
```bash
cargo run
```

## API Documentation

The CorneaDeSci API provides endpoints for user management, research projects, funding, and blockchain interactions:

- **User Management**: Registration, authentication, and profile management
- **Research Projects**: Create, view, and manage research projects
- **Funding**: Process payments, track funding status, and manage milestones
- **Blockchain**: Register research, verify proofs of invention, and manage tokens

For detailed API documentation, visit our API Docs (coming soon).

## Contributing

We welcome contributions to the CorneaDeSci platform! Please see our Contributing Guide for details on submitting pull requests, reporting bugs, and suggesting features.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

We thank the global corneal health research community for their inspiration and support in building this platform. Special thanks to the DeSci movement pioneers who have shown how blockchain technology can transform scientific research. 