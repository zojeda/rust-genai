use crate::adapter::{AdapterConfig, AdapterKind};
use crate::chat::ChatOptions;
use crate::resolver::AdapterKindResolver;
use crate::tools::{GenAITool, ToolsManager};
use crate::webc::WebClient;
use crate::{Client, ClientConfig};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct ClientBuilder {
	adapter_config_by_kind: Option<HashMap<AdapterKind, AdapterConfig>>,

	web_client: Option<WebClient>,

	config: Option<ClientConfig>,

	tools: Option<Vec<GenAITool>>,
}

/// Builder methods
impl ClientBuilder {
	pub fn with_reqwest(mut self, reqwest_client: reqwest::Client) -> Self {
		self.web_client = Some(WebClient::from_reqwest_client(reqwest_client));
		self
	}

	/// With a client config.
	pub fn with_config(mut self, config: ClientConfig) -> Self {
		self.config = Some(config);
		self
	}

	pub fn insert_adapter_config(mut self, kind: AdapterKind, adapter_config: AdapterConfig) -> Self {
		self.adapter_config_by_kind
			.get_or_insert_with(HashMap::new)
			.insert(kind, adapter_config);
		self
	}

	pub fn with_tool(mut self, tool: GenAITool) -> Self {
		self.tools.get_or_insert_with(Vec::new).push(tool);
		self
	}
}

/// Builder ClientConfig passthrough convenient setters
/// The goal of those functions is to set nested value such as ClientConfig.
impl ClientBuilder {
	/// Set the ChatOptions for the ClientConfig of this ClientBuilder.
	/// Will create the ClientConfig if not present.
	/// Otherwise, will just set the `client_config.chat_options`
	pub fn with_chat_options(mut self, options: ChatOptions) -> Self {
		let client_config = self.config.get_or_insert_with(ClientConfig::default);
		client_config.chat_options = Some(options);
		self
	}

	/// Set the AdapterKindResolver for the ClientConfig of this ClientBuilder.
	/// Will create the ClientConfig if not present.
	/// Otherwise, will just set the `client_config.adapter_kind_resolver`
	pub fn with_adapter_kind_resolver(mut self, resolver: AdapterKindResolver) -> Self {
		let client_config = self.config.get_or_insert_with(ClientConfig::default);
		client_config.adapter_kind_resolver = Some(resolver);
		self
	}
}

/// Build() method
impl ClientBuilder {
	pub fn build(self) -> Client {
		let inner = super::ClientInner {
			web_client: self.web_client.unwrap_or_default(),
			config: self.config.unwrap_or_default(),
			adapter_config_by_kind: self.adapter_config_by_kind,
			tools_manager: self.tools.map(|tools| {
				let mut p = ToolsManager::default();
				for tool in tools {
					p.add(tool.specification, tool.handler);
				}
				p
			}),
		};
		Client { inner: Arc::new(inner) }
	}
}
