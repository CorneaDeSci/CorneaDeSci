pub mod user;
pub mod research;
pub mod funding;

pub use user::{User, UserRole, CreateUserRequest, UpdateUserRequest, UserResponse, LoginRequest, AuthResponse};
pub use research::{Research, ResearchStatus, CreateResearchRequest, UpdateResearchRequest, 
                  ResearchDatapoint, CreateDatapointRequest, ProofOfInvention, CreatePoIRequest,
                  Collaboration, AddCollaboratorRequest};
pub use funding::{Funding, FundingStatus, FundingType, CreateFundingRequest, UpdateFundingStatusRequest,
                 FundingMilestone, CreateMilestoneRequest, UpdateMilestoneRequest, 
                 FundingStatistics, TokenBalance}; 