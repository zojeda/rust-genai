//! This module contains all the types related to a Chat Response (except ChatStream which has it file).

use serde_json::Value;

use crate::chat::{ChatStream, MessageContent};

// region:    --- ChatResponse

#[derive(Debug, Clone, Default)]
pub struct ChatResponse {
	pub response: Response,
	pub usage: MetaUsage,
}

#[derive(Debug, Clone)]
pub enum Response {
	Content(Option<MessageContent>),
	FunctionCalls(Vec<FunctionCall>),
}

impl Default for Response {
	fn default() -> Self {
		Response::Content(None)
	}
}

#[derive(Debug, Clone, Default)]
pub struct FunctionCall {
	pub id: String,
	pub name: String,
	pub arguments: Value,
}
// Getters
impl ChatResponse {
	/// Returns the eventual content as `&str` if it is of type `MessageContent::Text`
	/// Otherwise, return None
	pub fn content_text_as_str(&self) -> Option<&str> {
		match &self.response {
			Response::Content(content) => content.as_ref().and_then(MessageContent::text_as_str),
			Response::FunctionCalls(_) => None,
		}
	}

	/// Consume the ChatResponse and returns the eventual String content of the `MessageContent::Text`
	/// Otherwise, return None
	pub fn content_text_into_string(self) -> Option<String> {
		match self.response {
			Response::Content(content) => content.and_then(MessageContent::text_into_string),
			Response::FunctionCalls(_) => None,
		}
	}
}

// endregion: --- ChatResponse

// region:    --- ChatStreamResponse

pub struct ChatStreamResponse {
	pub stream: ChatStream,
}

// endregion: --- ChatStreamResponse

// region:    --- MetaUsage

/// IMPORTANT: This is **NOT SUPPORTED** for now. To show the API direction.
#[derive(Default, Debug, Clone)]
pub struct MetaUsage {
	pub input_tokens: Option<i32>,
	pub output_tokens: Option<i32>,
	pub total_tokens: Option<i32>,
}

// endregion: --- MetaUsage
