use std::collections::HashMap;

use serde_json::Value;

use crate::chat::FunctionCall;

use super::ToolSpecification;

#[derive(Debug, Clone)]
pub struct FunctionCallResult {
	pub result: Value,
	pub call_id: String,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ToolsManager {
	available_functions: HashMap<String, fn(Value) -> Value>,
	specifications: Vec<ToolSpecification>,
}

impl ToolsManager {
	pub(crate) fn add(&mut self, specification: ToolSpecification, handler: fn(Value) -> Value) {
		self.available_functions.insert(specification.name.clone(), handler);
		self.specifications.push(specification);
	}

	pub fn specifications(&self) -> &Vec<ToolSpecification> {
		&self.specifications
	}

	pub fn handle_call(&self, function_call: &FunctionCall, arguments: Value) -> FunctionCallResult {
		let function = self.available_functions.get(&function_call.name).unwrap();
		let result = function(arguments);
		FunctionCallResult {
			result,
			call_id: function_call.id.clone(),
		}
	}
}