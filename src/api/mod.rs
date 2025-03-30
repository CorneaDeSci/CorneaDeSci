pub mod user_routes;
pub mod research_routes;
pub mod funding_routes;
pub mod blockchain_routes;

pub use user_routes::configure as configure_user_routes;
pub use research_routes::configure as configure_research_routes;
pub use funding_routes::configure as configure_funding_routes;
pub use blockchain_routes::configure as configure_blockchain_routes; 