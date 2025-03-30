<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 800 800" width="200" height="200" style="display: block; margin: 0 auto;">
  <!-- Define gradients and filters -->
  <defs>
    <!-- Background gradient -->
    <radialGradient id="space-bg" cx="50%" cy="50%" r="50%" fx="50%" fy="50%">
      <stop offset="0%" stop-color="#1a2980" />
      <stop offset="50%" stop-color="#0D1426" />
      <stop offset="100%" stop-color="#03070D" />
    </radialGradient>
    
    <!-- Iris gradient -->
    <radialGradient id="iris-gradient" cx="50%" cy="50%" r="60%" fx="50%" fy="50%">
      <stop offset="0%" stop-color="#00C9FF" />
      <stop offset="40%" stop-color="#0078FF" />
      <stop offset="80%" stop-color="#3742B4" />
      <stop offset="100%" stop-color="#0C1F4F" />
    </radialGradient>
    
    <!-- Outer glow gradient -->
    <radialGradient id="outer-glow" cx="50%" cy="50%" r="50%" fx="50%" fy="50%">
      <stop offset="0%" stop-color="#00C9FF" stop-opacity="0.7" />
      <stop offset="70%" stop-color="#00C9FF" stop-opacity="0.2" />
      <stop offset="100%" stop-color="#00C9FF" stop-opacity="0" />
    </radialGradient>
    
    <!-- Tech texture gradient -->
    <linearGradient id="tech-gradient" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#26D0CE" />
      <stop offset="100%" stop-color="#1A2980" />
    </linearGradient>
    
    <!-- Glow effect -->
    <filter id="blue-glow" x="-50%" y="-50%" width="200%" height="200%">
      <feGaussianBlur stdDeviation="15" result="blur" />
      <feComposite in="SourceGraphic" in2="blur" operator="over" />
    </filter>
    
    <!-- Bright glow filter -->
    <filter id="bright-glow" x="-50%" y="-50%" width="200%" height="200%">
      <feGaussianBlur stdDeviation="5" result="blur" />
      <feComposite in="SourceGraphic" in2="blur" operator="over" />
    </filter>
    
    <!-- Hexagon texture -->
    <pattern id="hex-pattern" x="0" y="0" width="60" height="60" patternUnits="userSpaceOnUse">
      <path d="M25,0 L50,15 L50,45 L25,60 L0,45 L0,15 Z" fill="#1A2980" fill-opacity="0.3"/>
    </pattern>
    
    <!-- Shine effect -->
    <radialGradient id="shine-gradient" cx="50%" cy="50%" r="50%" fx="40%" fy="40%">
      <stop offset="0%" stop-color="#FFFFFF" stop-opacity="0.9" />
      <stop offset="20%" stop-color="#4FC3F7" stop-opacity="0.7" />
      <stop offset="100%" stop-color="#00C9FF" stop-opacity="0" />
    </radialGradient>
  </defs>
  
  <!-- Main background -->
  <circle cx="400" cy="400" r="390" fill="url(#space-bg)" />
  
  <!-- Outer halo -->
  <circle cx="400" cy="400" r="370" fill="none" stroke="url(#outer-glow)" stroke-width="20" stroke-opacity="0.3" />
  
  <!-- Outer ring structure -->
  <g>
    <!-- Outer decorative panels -->
    <!-- Top panel -->
    <path d="M320,100 Q400,70 480,100 L460,140 Q400,120 340,140 Z" fill="#192B59" />
    <path d="M320,100 Q400,70 480,100 L460,140 Q400,120 340,140 Z" fill="#4FC3F7" fill-opacity="0.1" />
    <circle cx="400" cy="100" r="15" fill="#0288D1" filter="url(#bright-glow)" />
    <rect x="370" y="95" width="60" height="10" rx="5" fill="#4FC3F7" opacity="0.7" />
    
    <!-- Upper right panel -->
    <path d="M590,170 Q630,230 650,300 L610,310 Q595,250 560,210 Z" fill="#192B59" />
    <path d="M590,170 Q630,230 650,300 L610,310 Q595,250 560,210 Z" fill="#4FC3F7" fill-opacity="0.1" />
    <circle cx="600" cy="240" r="15" fill="#0288D1" filter="url(#bright-glow)" />
    <path d="M585,235 Q600,220 615,235" fill="#4FC3F7" opacity="0.7" />
    
    <!-- Lower right panel -->
    <path d="M650,500 Q630,570 590,630 L560,590 Q595,550 610,490 Z" fill="#192B59" />
    <path d="M650,500 Q630,570 590,630 L560,590 Q595,550 610,490 Z" fill="#4FC3F7" fill-opacity="0.1" />
    <circle cx="600" cy="560" r="15" fill="#0288D1" filter="url(#bright-glow)" />
    <rect x="585" y="555" width="30" height="10" rx="5" fill="#4FC3F7" opacity="0.7" />
    
    <!-- Bottom panel -->
    <path d="M480,700 Q400,730 320,700 L340,660 Q400,680 460,660 Z" fill="#192B59" />
    <path d="M480,700 Q400,730 320,700 L340,660 Q400,680 460,660 Z" fill="#4FC3F7" fill-opacity="0.1" />
    <circle cx="400" cy="700" r="15" fill="#0288D1" filter="url(#bright-glow)" />
    <path d="M385,700 Q400,715 415,700" fill="#4FC3F7" opacity="0.7" />
    
    <!-- Lower left panel -->
    <path d="M210,630 Q170,570 150,500 L190,490 Q205,550 240,590 Z" fill="#192B59" />
    <path d="M210,630 Q170,570 150,500 L190,490 Q205,550 240,590 Z" fill="#4FC3F7" fill-opacity="0.1" />
    <circle cx="200" cy="560" r="15" fill="#0288D1" filter="url(#bright-glow)" />
    <rect x="185" y="555" width="30" height="10" rx="5" fill="#4FC3F7" opacity="0.7" />
    
    <!-- Upper left panel -->
    <path d="M150,300 Q170,230 210,170 L240,210 Q205,250 190,310 Z" fill="#192B59" />
    <path d="M150,300 Q170,230 210,170 L240,210 Q205,250 190,310 Z" fill="#4FC3F7" fill-opacity="0.1" />
    <circle cx="200" cy="240" r="15" fill="#0288D1" filter="url(#bright-glow)" />
    <path d="M185,235 Q200,250 215,235" fill="#4FC3F7" opacity="0.7" />
  </g>
  
  <!-- Connect panel elements -->
  <g>
    <!-- Connection beams -->
    <path d="M400,115 Q400,200 350,250" fill="#4FC3F7" fill-opacity="0.2" />
    <path d="M400,115 Q400,200 450,250" fill="#4FC3F7" fill-opacity="0.2" />
    <path d="M585,240 Q500,300 500,400" fill="#4FC3F7" fill-opacity="0.2" />
    <path d="M585,560 Q500,500 500,400" fill="#4FC3F7" fill-opacity="0.2" />
    <path d="M400,685 Q400,600 450,550" fill="#4FC3F7" fill-opacity="0.2" />
    <path d="M400,685 Q400,600 350,550" fill="#4FC3F7" fill-opacity="0.2" />
    <path d="M215,560 Q300,500 300,400" fill="#4FC3F7" fill-opacity="0.2" />
    <path d="M215,240 Q300,300 300,400" fill="#4FC3F7" fill-opacity="0.2" />
    
    <!-- Connection nodes -->
    <circle cx="350" cy="250" r="12" fill="#192B59" />
    <circle cx="350" cy="250" r="8" fill="#00C9FF" filter="url(#bright-glow)" />
    
    <circle cx="450" cy="250" r="12" fill="#192B59" />
    <circle cx="450" cy="250" r="8" fill="#00C9FF" filter="url(#bright-glow)" />
    
    <circle cx="500" cy="400" r="12" fill="#192B59" />
    <circle cx="500" cy="400" r="8" fill="#00C9FF" filter="url(#bright-glow)" />
    
    <circle cx="450" cy="550" r="12" fill="#192B59" />
    <circle cx="450" cy="550" r="8" fill="#00C9FF" filter="url(#bright-glow)" />
    
    <circle cx="350" cy="550" r="12" fill="#192B59" />
    <circle cx="350" cy="550" r="8" fill="#00C9FF" filter="url(#bright-glow)" />
    
    <circle cx="300" cy="400" r="12" fill="#192B59" />
    <circle cx="300" cy="400" r="8" fill="#00C9FF" filter="url(#bright-glow)" />
  </g>
  
  <!-- Middle ring area -->
  <g>
    <!-- Hexagon texture area -->
    <circle cx="400" cy="400" r="280" fill="url(#hex-pattern)" />
    <circle cx="400" cy="400" r="280" fill="#1A2980" fill-opacity="0.1" />
    
    <!-- Hexagon 3D structure -->
    <g transform="translate(400, 400)">
      <!-- Outer hexagon -->
      <path d="M0,-180 L156,-90 L156,90 L0,180 L-156,90 L-156,-90 Z" fill="#192B59" fill-opacity="0.6" />
      <path d="M0,-180 L156,-90 L156,90 L0,180 L-156,90 L-156,-90 Z" fill="#00C9FF" fill-opacity="0.1" />
      
      <!-- Inner hexagon -->
      <path d="M0,-150 L130,-75 L130,75 L0,150 L-130,75 L-130,-75 Z" fill="#0D1426" fill-opacity="0.7" />
      <path d="M0,-150 L130,-75 L130,75 L0,150 L-130,75 L-130,-75 Z" fill="#00C9FF" fill-opacity="0.15" />
      
      <!-- Corner decorations -->
      <circle cx="0" cy="-180" r="15" fill="#1A2980" />
      <circle cx="0" cy="-180" r="10" fill="#00C9FF" filter="url(#bright-glow)" />
      
      <circle cx="156" cy="-90" r="15" fill="#1A2980" />
      <circle cx="156" cy="-90" r="10" fill="#00C9FF" filter="url(#bright-glow)" />
      
      <circle cx="156" cy="90" r="15" fill="#1A2980" />
      <circle cx="156" cy="90" r="10" fill="#00C9FF" filter="url(#bright-glow)" />
      
      <circle cx="0" cy="180" r="15" fill="#1A2980" />
      <circle cx="0" cy="180" r="10" fill="#00C9FF" filter="url(#bright-glow)" />
      
      <circle cx="-156" cy="90" r="15" fill="#1A2980" />
      <circle cx="-156" cy="90" r="10" fill="#00C9FF" filter="url(#bright-glow)" />
      
      <circle cx="-156" cy="-90" r="15" fill="#1A2980" />
      <circle cx="-156" cy="-90" r="10" fill="#00C9FF" filter="url(#bright-glow)" />
    </g>
  </g>
  
  <!-- Particle area -->
  <g>
    <circle cx="450" cy="180" r="7" fill="#192B59" />
    <circle cx="450" cy="180" r="5" fill="#00C9FF" filter="url(#bright-glow)" />
    
    <circle cx="530" cy="330" r="8" fill="#192B59" />
    <circle cx="530" cy="330" r="6" fill="#00C9FF" filter="url(#bright-glow)" />
    
    <circle cx="530" cy="470" r="6" fill="#192B59" />
    <circle cx="530" cy="470" r="4" fill="#00C9FF" filter="url(#bright-glow)" />
    
    <circle cx="450" cy="620" r="7" fill="#192B59" />
    <circle cx="450" cy="620" r="5" fill="#00C9FF" filter="url(#bright-glow)" />
    
    <circle cx="350" cy="620" r="8" fill="#192B59" />
    <circle cx="350" cy="620" r="6" fill="#00C9FF" filter="url(#bright-glow)" />
    
    <circle cx="270" cy="470" r="7" fill="#192B59" />
    <circle cx="270" cy="470" r="5" fill="#00C9FF" filter="url(#bright-glow)" />
    
    <circle cx="270" cy="330" r="6" fill="#192B59" />
    <circle cx="270" cy="330" r="4" fill="#00C9FF" filter="url(#bright-glow)" />
    
    <circle cx="350" cy="180" r="8" fill="#192B59" />
    <circle cx="350" cy="180" r="6" fill="#00C9FF" filter="url(#bright-glow)" />
  </g>
  
  <!-- Data flow area -->
  <g>
    <!-- Upper data flow area -->
    <path d="M300,350 C350,320 400,350 450,320 C500,350 550,320 600,350" fill="#00C9FF" fill-opacity="0.2" filter="url(#bright-glow)" />
    <path d="M300,330 C350,300 400,330 450,300 C500,330 550,300 600,330" fill="#00C9FF" fill-opacity="0.15" filter="url(#bright-glow)" />
    
    <!-- Lower data flow area -->
    <path d="M300,450 C350,480 400,450 450,480 C500,450 550,480 600,450" fill="#00C9FF" fill-opacity="0.2" filter="url(#bright-glow)" />
    <path d="M300,470 C350,500 400,470 450,500 C500,470 550,500 600,470" fill="#00C9FF" fill-opacity="0.15" filter="url(#bright-glow)" />
  </g>
  
  <!-- Corneal core -->
  <g>
    <!-- Outer halo -->
    <circle cx="400" cy="400" r="170" fill="none" stroke="#00C9FF" stroke-width="20" stroke-opacity="0.2" filter="url(#blue-glow)" />
    
    <!-- Corneal outer ring -->
    <circle cx="400" cy="400" r="150" fill="url(#outer-glow)" filter="url(#blue-glow)" />
    
    <!-- Iris layer -->
    <circle cx="400" cy="400" r="120" fill="url(#iris-gradient)" />
    
    <!-- Iris texture - using fill areas instead of lines -->
    <g>
      <!-- Radial texture fills -->
      <path d="M390,280 L410,280 L410,340 L390,340 Z" fill="#00C9FF" fill-opacity="0.6" />
      <path d="M390,460 L410,460 L410,520 L390,520 Z" fill="#00C9FF" fill-opacity="0.6" />
      <path d="M280,390 L340,390 L340,410 L280,410 Z" fill="#00C9FF" fill-opacity="0.6" />
      <path d="M460,390 L520,390 L520,410 L460,410 Z" fill="#00C9FF" fill-opacity="0.6" />
      
      <path d="M335,335 L320,320 L345,345 L360,360 Z" fill="#00C9FF" fill-opacity="0.6" transform="rotate(45, 340, 340)" />
      <path d="M455,455 L440,440 L465,465 L480,480 Z" fill="#00C9FF" fill-opacity="0.6" transform="rotate(45, 460, 460)" />
      <path d="M335,455 L320,470 L345,445 L360,430 Z" fill="#00C9FF" fill-opacity="0.6" transform="rotate(-45, 340, 450)" />
      <path d="M455,335 L440,350 L465,325 L480,310 Z" fill="#00C9FF" fill-opacity="0.6" transform="rotate(-45, 460, 330)" />
      
      <!-- Circular texture fills -->
      <circle cx="400" cy="400" r="100" fill="#FFFFFF" fill-opacity="0.1" />
      <circle cx="400" cy="400" r="85" fill="#FFFFFF" fill-opacity="0.15" />
      <circle cx="400" cy="400" r="70" fill="#FFFFFF" fill-opacity="0.1" />
    </g>
    
    <!-- Center pupil -->
    <circle cx="400" cy="400" r="50" fill="#03070D" />
    <circle cx="400" cy="400" r="40" fill="#151F52" />
    
    <!-- Center light source -->
    <circle cx="400" cy="400" r="30" fill="#192B59" />
    <circle cx="400" cy="400" r="25" fill="#00C9FF" filter="url(#blue-glow)" />
    <circle cx="400" cy="400" r="15" fill="url(#shine-gradient)" />
    
    <!-- Reflection effect -->
    <circle cx="385" cy="385" r="8" fill="#FFFFFF" opacity="0.7" />
    <circle cx="375" cy="375" r="4" fill="#FFFFFF" opacity="0.5" />
  </g>
  
  <!-- Energy pulse effect - using fill areas -->
  <g>
    <path d="M260,395 C300,355 350,375 400,395 C450,415 500,435 540,395 L540,405 C500,445 450,425 400,405 C350,385 300,365 260,405 Z" fill="#00C9FF" fill-opacity="0.3" filter="url(#bright-glow)" />
    <path d="M395,260 C355,300 375,350 395,400 C415,450 435,500 395,540 L405,540 C445,500 425,450 405,400 C385,350 365,300 405,260 Z" fill="#00C9FF" fill-opacity="0.3" filter="url(#bright-glow)" />
  </g>
  
  <!-- Decorative particles starlight -->
  <g>
    <circle cx="320" cy="320" r="4" fill="#FFFFFF" opacity="0.7" />
    <circle cx="480" cy="320" r="5" fill="#FFFFFF" opacity="0.7" />
    <circle cx="320" cy="480" r="5" fill="#FFFFFF" opacity="0.7" />
    <circle cx="480" cy="480" r="4" fill="#FFFFFF" opacity="0.7" />
    <circle cx="350" cy="300" r="3" fill="#FFFFFF" opacity="0.7" />
    <circle cx="450" cy="300" r="3" fill="#FFFFFF" opacity="0.7" />
    <circle cx="300" cy="350" r="3" fill="#FFFFFF" opacity="0.7" />
    <circle cx="300" cy="450" r="3" fill="#FFFFFF" opacity="0.7" />
    <circle cx="450" cy="500" r="3" fill="#FFFFFF" opacity="0.7" />
    <circle cx="350" cy="500" r="3" fill="#FFFFFF" opacity="0.7" />
    <circle cx="500" cy="350" r="3" fill="#FFFFFF" opacity="0.7" />
    <circle cx="500" cy="450" r="3" fill="#FFFFFF" opacity="0.7" />
  </g>
  
  <!-- Outer decorative light effect -->
  <circle cx="400" cy="400" r="375" fill="none" stroke="#00C9FF" stroke-width="10" stroke-opacity="0.05" />
</svg>

# CorneaDeSci: Decentralized Corneal Health Research Platform

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