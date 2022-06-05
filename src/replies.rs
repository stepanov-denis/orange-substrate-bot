pub mod repl_logic {
    use env_logger::{Builder, Target};
    use teloxide::prelude::*;

    #[tokio::main]
    pub async fn dices() {
        // For run with pretty_env_logger in terminal
        // pretty_env_logger::init();
        // log::info!("Starting throw dice bot...");

        // For run with env_logger and record logs to file
        let mut builder = Builder::from_default_env();
        builder.target(Target::Stdout);
        builder.init();

        let bot = Bot::from_env().auto_send();

        teloxide::repl(bot, |message: Message, bot: AutoSend<Bot>| async move {
            bot.send_dice(message.chat.id).await?;
            respond(())
        })
        .await;
    }
}
