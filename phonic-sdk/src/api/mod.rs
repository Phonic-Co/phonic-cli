//! API client and types for the Phonic
//!
//! This module contains all the API definitions including request/response types
//! and client implementations for interacting with the API.
//!
//! ## Modules
//!
//! - [`resources`] - Service clients and endpoints

pub mod resources;

pub use resources::{
    AgentsClient, ApiClient, ApiKeysClient, AuthClient, ConversationItemsClient,
    ConversationsClient, ExtractionSchemasClient, ProjectsClient, ToolsClient, TtsClient,
    VoicesClient, WorkspaceClient,
};

pub use phonic_types::*;
