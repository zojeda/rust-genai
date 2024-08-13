use crate::adapter::{AdapterConfig, AdapterKind};
use crate::chat::{ChatOptionsSet, ChatRequest, ChatResponse, ChatStreamResponse};
use crate::tools::ToolsManager;
use crate::webc::WebResponse;
use crate::Result;
use crate::{ConfigSet, ModelInfo};
use reqwest::RequestBuilder;
use serde_json::Value;

pub trait Adapter {
	// NOTE: Adapter is a crate Trait, so, ok to use async fn here.
	async fn all_model_names(kind: AdapterKind) -> Result<Vec<String>>;

	/// The static default AdapterConfig for this AdapterKind
	/// Note: Implementation typically using OnceLock
	fn default_adapter_config(kind: AdapterKind) -> &'static AdapterConfig;

	/// The base service url for this AdapterKind for this given service type.
	/// NOTE: For some services, the url will be further updated in the to_web_request_data
	fn get_service_url(model_info: ModelInfo, service_type: ServiceType) -> String;

	/// To be implemented by Adapters
	fn to_web_request_data(
		model_info: ModelInfo,
		config_set: &ConfigSet<'_>,
		service_type: ServiceType,
		chat_req: ChatRequest,
		options_set: ChatOptionsSet<'_, '_>,
	) -> Result<WebRequestData>;

	/// To be implemented by Adapters
	fn to_chat_response(model_info: ModelInfo, web_response: WebResponse) -> Result<ChatResponse>;

	/// To be implemented by Adapters
	fn to_chat_stream(
		model_info: ModelInfo,
		reqwest_builder: RequestBuilder,
		options_set: ChatOptionsSet<'_, '_>,
	) -> Result<ChatStreamResponse>;
}

// region:    --- ServiceType

#[derive(Debug, Clone, Copy)]
pub enum ServiceType {
	Chat,
	ChatStream,
}

// endregion: --- ServiceType

// region:    --- WebRequestData

// NOTE: This cannot really move to `webc` because it has to be public with the adapter and `webc` is private for now.

pub struct WebRequestData {
	pub url: String,
	pub headers: Vec<(String, String)>,
	pub payload: Value,
}

// endregion: --- WebRequestData
