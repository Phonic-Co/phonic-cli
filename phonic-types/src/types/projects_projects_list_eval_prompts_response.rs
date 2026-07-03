pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProjectsListEvalPromptsResponse {
    #[serde(default)]
    pub conversation_eval_prompts: Vec<ConversationEvalPrompt>,
}

impl ProjectsListEvalPromptsResponse {
    pub fn builder() -> ProjectsListEvalPromptsResponseBuilder {
        <ProjectsListEvalPromptsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProjectsListEvalPromptsResponseBuilder {
    conversation_eval_prompts: Option<Vec<ConversationEvalPrompt>>,
}

impl ProjectsListEvalPromptsResponseBuilder {
    pub fn conversation_eval_prompts(mut self, value: Vec<ConversationEvalPrompt>) -> Self {
        self.conversation_eval_prompts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProjectsListEvalPromptsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversation_eval_prompts`](ProjectsListEvalPromptsResponseBuilder::conversation_eval_prompts)
    pub fn build(self) -> Result<ProjectsListEvalPromptsResponse, BuildError> {
        Ok(ProjectsListEvalPromptsResponse {
            conversation_eval_prompts: self.conversation_eval_prompts.ok_or_else(|| BuildError::missing_field("conversation_eval_prompts"))?,
        })
    }
}
