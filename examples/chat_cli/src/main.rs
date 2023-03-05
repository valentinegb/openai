use dotenvy::dotenv;
use openai::{
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
    models::ModelID,
};
use std::io::{stdin, stdout, Write};

#[tokio::main]
async fn main() {
    // Make sure you have a file named `.env` with the `OPENAI_KEY` environment variable defined!
    dotenv().unwrap();

    let mut messages = vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::System,
        content: "You are a large language built into a command line interface as an example of what the `openai` Rust library made by Valentine Briese can do.".to_string(),
        user_name: None,
    }];

    loop {
        print!("User: ");
        stdout().flush().unwrap();

        let mut user_message_content = String::new();

        stdin().read_line(&mut user_message_content).unwrap();
        messages.push(ChatCompletionMessage {
            role: ChatCompletionMessageRole::User,
            content: user_message_content,
            user_name: None,
        });

        let chat_completion = ChatCompletion::builder(ModelID::Gpt3_5Turbo, messages.clone())
            .create()
            .await
            .unwrap()
            .unwrap();
        let returned_message = chat_completion.choices.first().unwrap().message.clone();

        println!(
            "{:#?}: {}",
            &returned_message.role,
            &returned_message.content.trim()
        );

        messages.push(returned_message);
    }
}
