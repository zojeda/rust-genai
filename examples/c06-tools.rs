use genai::chat::{ChatMessage, ChatRequest};
use genai::Client;

use genai_tools_macros::genai_tool;
use serde_json::{json, Value};

const MODEL: &str = "gpt-4o-mini";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let questions = &[
		// follow-up questions
		"What's Argentina's capital?",
		"What's the weather like there right now?",
		// "How hot will it be today in Seattle? And in Miami?.  Use multi-tool to get both at the same time",
	];

	let weather_tool = Weather::tool();
	let client = Client::builder().with_tool(weather_tool).build();

	let mut chat_req = ChatRequest::default().with_system("Answer in one sentence");
	// Similar to putting a first System Chat Message(s) (will be cumulative with system chat messages)

	for &question in questions {
		chat_req = chat_req.append_message(ChatMessage::user(question));

		println!("\n--- Question:\n{question}");
		let chat_res = client.exec_chat(MODEL, chat_req.clone(), None).await?;

		println!("\n--- Answer:");
		println!("{:?}", chat_res);

		chat_req = chat_req.append_message(ChatMessage::assistant(chat_res.content_text_as_str().unwrap()));
	}

	Ok(())
}

#[genai_tool(Weather)]
/// Get the current weather in a given location
fn get_current_weather(
	#[description("The city and state, e.g. San Francisco, CA")] location: String,
	unit: String,
) -> Value {
	let temp = if location == "Miami" { 35 } else { 20 };
	let weather_info = json!({
		"location": location,
		"temperature": temp,
		"unit": unit,
		"forecast": ["sunny", "windy"]
	});

	weather_info
}
