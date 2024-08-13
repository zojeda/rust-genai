
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct GenAITool {
	pub specification: ToolSpecification,
	pub handler: fn(Value) -> Value,
}

#[derive(Debug, Clone)]
pub struct ToolSpecification {
	pub name: String,
	pub description: String,
	pub parameters: Value,
}

