pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationAnalysis {
    /// The ID of the conversation analysis.
    #[serde(default)]
    pub id: String,
    /// Latencies between turns in milliseconds.
    #[serde(default)]
    pub latencies_ms: Vec<f64>,
    /// Number of interruptions in the conversation.
    #[serde(default)]
    pub interruptions_count: i64,
}

impl ConversationAnalysis {
    pub fn builder() -> ConversationAnalysisBuilder {
        <ConversationAnalysisBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationAnalysisBuilder {
    id: Option<String>,
    latencies_ms: Option<Vec<f64>>,
    interruptions_count: Option<i64>,
}

impl ConversationAnalysisBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn latencies_ms(mut self, value: Vec<f64>) -> Self {
        self.latencies_ms = Some(value);
        self
    }

    pub fn interruptions_count(mut self, value: i64) -> Self {
        self.interruptions_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationAnalysis`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ConversationAnalysisBuilder::id)
    /// - [`latencies_ms`](ConversationAnalysisBuilder::latencies_ms)
    /// - [`interruptions_count`](ConversationAnalysisBuilder::interruptions_count)
    pub fn build(self) -> Result<ConversationAnalysis, BuildError> {
        Ok(ConversationAnalysis {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            latencies_ms: self.latencies_ms.ok_or_else(|| BuildError::missing_field("latencies_ms"))?,
            interruptions_count: self.interruptions_count.ok_or_else(|| BuildError::missing_field("interruptions_count"))?,
        })
    }
}
