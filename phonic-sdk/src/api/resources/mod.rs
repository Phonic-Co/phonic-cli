//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **Agents**
//! - **Tools**
//! - **ExtractionSchemas**
//! - **Voices**
//! - **Workspace**
//! - **ApiKeys**
//! - **ConversationItems**
//! - **Conversations**
//! - **Auth**
//! - **Tts**
//! - **Projects**

use crate::{ApiError, ClientConfig};

pub mod agents;
pub mod api_keys;
pub mod auth;
pub mod conversation_items;
pub mod conversations;
pub mod extraction_schemas;
pub mod projects;
pub mod tools;
pub mod tts;
pub mod voices;
pub mod workspace;
pub struct ApiClient {
    pub config: ClientConfig,
    pub agents: AgentsClient,
    pub tools: ToolsClient,
    pub extraction_schemas: ExtractionSchemasClient,
    pub voices: VoicesClient,
    pub workspace: WorkspaceClient,
    pub api_keys: ApiKeysClient,
    pub conversation_items: ConversationItemsClient,
    pub conversations: ConversationsClient,
    pub auth: AuthClient,
    pub tts: TtsClient,
    pub projects: ProjectsClient,
}

impl ApiClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            agents: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .environment
                    .as_ref()
                    .map_or_else(|| cfg.base_url.clone(), |env| env.base_url().to_string());
                AgentsClient::new(cfg)?
            },
            tools: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .environment
                    .as_ref()
                    .map_or_else(|| cfg.base_url.clone(), |env| env.base_url().to_string());
                ToolsClient::new(cfg)?
            },
            extraction_schemas: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .environment
                    .as_ref()
                    .map_or_else(|| cfg.base_url.clone(), |env| env.base_url().to_string());
                ExtractionSchemasClient::new(cfg)?
            },
            voices: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .environment
                    .as_ref()
                    .map_or_else(|| cfg.base_url.clone(), |env| env.base_url().to_string());
                VoicesClient::new(cfg)?
            },
            workspace: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .environment
                    .as_ref()
                    .map_or_else(|| cfg.base_url.clone(), |env| env.base_url().to_string());
                WorkspaceClient::new(cfg)?
            },
            api_keys: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .environment
                    .as_ref()
                    .map_or_else(|| cfg.base_url.clone(), |env| env.base_url().to_string());
                ApiKeysClient::new(cfg)?
            },
            conversation_items: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .environment
                    .as_ref()
                    .map_or_else(|| cfg.base_url.clone(), |env| env.base_url().to_string());
                ConversationItemsClient::new(cfg)?
            },
            conversations: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .environment
                    .as_ref()
                    .map_or_else(|| cfg.base_url.clone(), |env| env.base_url().to_string());
                ConversationsClient::new(cfg)?
            },
            auth: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .environment
                    .as_ref()
                    .map_or_else(|| cfg.base_url.clone(), |env| env.base_url().to_string());
                AuthClient::new(cfg)?
            },
            tts: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .environment
                    .as_ref()
                    .map_or_else(|| cfg.base_url.clone(), |env| env.base_url().to_string());
                TtsClient::new(cfg)?
            },
            projects: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .environment
                    .as_ref()
                    .map_or_else(|| cfg.base_url.clone(), |env| env.base_url().to_string());
                ProjectsClient::new(cfg)?
            },
        })
    }
}

pub use agents::AgentsClient;
pub use api_keys::ApiKeysClient;
pub use auth::AuthClient;
pub use conversation_items::ConversationItemsClient;
pub use conversations::ConversationsClient;
pub use extraction_schemas::ExtractionSchemasClient;
pub use projects::ProjectsClient;
pub use tools::ToolsClient;
pub use tts::TtsClient;
pub use voices::VoicesClient;
pub use workspace::WorkspaceClient;
