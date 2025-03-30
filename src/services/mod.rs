pub mod auth;
pub mod user;
pub mod research;
pub mod funding;
pub mod blockchain;

// Export common functionality
pub use auth::AuthService;
pub use user::UserService;
pub use research::ResearchService;
pub use funding::FundingService;
pub use blockchain::BlockchainService; 