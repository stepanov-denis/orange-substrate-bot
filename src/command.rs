pub mod command_logic {
    use env_logger::{Builder, Target};
    use std::error::Error;
    use teloxide::{prelude::*, utils::command::BotCommands};

    #[tokio::main]
    pub async fn my_command() {
        // For run with pretty_env_logger in terminal
        // pretty_env_logger::init();
        // log::info!("Starting command bot...");

        // For run with env_logger and record logs to file
        // let mut builder = Builder::from_default_env();
        // builder.target(Target::Stdout);
        // builder.init();

        let bot = Bot::from_env().auto_send();

        teloxide::commands_repl(bot, answer, Command::ty()).await;
    }

    #[derive(BotCommands, Clone)]
    #[command(rename = "lowercase", description = "These commands are supported:")]
    enum Command {
        #[command(description = "display this text.")]
        Help,
        #[command(description = "handle a username.")]
        Username(String),
        #[command(description = "handle a username and an age.", parse_with = "split")]
        UsernameAndAge { username: String, age: u8 },
    }

    async fn answer(
        bot: AutoSend<Bot>,
        message: Message,
        command: Command,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        match command {
            Command::Help => {
                bot.send_message(message.chat.id, Command::descriptions().to_string())
                    .await?
            }
            Command::Username(username) => {
                bot.send_message(message.chat.id, format!("Your username is @{username}."))
                    .await?
            }
            Command::UsernameAndAge { username, age } => {
                bot.send_message(
                    message.chat.id,
                    format!("Your username is @{username} and age is {age}."),
                )
                .await?
            }
        };

        Ok(())
    }
}
