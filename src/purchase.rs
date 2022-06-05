pub mod purchase_logic {
    use teloxide::{
        dispatching::{
            dialogue::{self, InMemStorage},
            UpdateHandler,
        },
        prelude::*,
        types::{InlineKeyboardButton, InlineKeyboardMarkup},
        utils::command::BotCommands,
    };
    use env_logger::{Builder, Target};

    type MyDialogue = Dialogue<State, InMemStorage<State>>;
    type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

    #[derive(Clone)]
    pub enum State {
        Start,
        ReceiveFullName,
        ReceiveProductChoice { full_name: String },
    }

    impl Default for State {
        fn default() -> Self {
            Self::Start
        }
    }

    #[derive(BotCommands, Clone)]
    #[command(rename = "lowercase", description = "These commands are supported:")]
    enum Command {
        #[command(description = "display this text.")]
        Help,
        #[command(description = "start the purchase procedure.")]
        Start,
        #[command(description = "cancel the purchase procedure.")]
        Cancel,
    }

    #[tokio::main]
    pub async fn buy() {
        // For run with pretty_env_logger in terminal
        // pretty_env_logger::init();
        // log::info!("Starting throw dice bot...");
        
        // For run with env_logger and record logs to file
        let mut builder = Builder::from_default_env();
        builder.target(Target::Stdout);
        builder.init();

        let bot = Bot::from_env().auto_send();

        Dispatcher::builder(bot, schema())
            .dependencies(dptree::deps![InMemStorage::<State>::new()])
            .build()
            .setup_ctrlc_handler()
            .dispatch()
            .await;
    }

    fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
        let command_handler = teloxide::filter_command::<Command, _>()
            .branch(
                dptree::case![State::Start]
                    .branch(dptree::case![Command::Help].endpoint(help))
                    .branch(dptree::case![Command::Start].endpoint(start)),
            )
            .branch(dptree::case![Command::Cancel].endpoint(cancel));

        let message_handler = Update::filter_message()
            .branch(command_handler)
            .branch(dptree::case![State::ReceiveFullName].endpoint(receive_full_name))
            .branch(dptree::endpoint(invalid_state));

        let callback_query_handler = Update::filter_callback_query().chain(
            dptree::case![State::ReceiveProductChoice { full_name }]
                .endpoint(receive_product_selection),
        );

        dialogue::enter::<Update, InMemStorage<State>, State, _>()
            .branch(message_handler)
            .branch(callback_query_handler)
    }

    async fn start(bot: AutoSend<Bot>, msg: Message, dialogue: MyDialogue) -> HandlerResult {
        bot.send_message(msg.chat.id, "Let's start! What's your full name?").await?;
        dialogue.update(State::ReceiveFullName).await?;
        Ok(())
    }

    async fn help(bot: AutoSend<Bot>, msg: Message) -> HandlerResult {
        bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
        Ok(())
    }

    async fn cancel(bot: AutoSend<Bot>, msg: Message, dialogue: MyDialogue) -> HandlerResult {
        bot.send_message(msg.chat.id, "Cancelling the dialogue.").await?;
        dialogue.exit().await?;
        Ok(())
    }

    async fn invalid_state(bot: AutoSend<Bot>, msg: Message) -> HandlerResult {
        bot.send_message(msg.chat.id, "Unable to handle the message. Type /help to see the usage.")
            .await?;
        Ok(())
    }

    async fn receive_full_name(
        bot: AutoSend<Bot>,
        msg: Message,
        dialogue: MyDialogue,
    ) -> HandlerResult {
        match msg.text().map(ToOwned::to_owned) {
            Some(full_name) => {
                let products = ["Apple", "Banana", "Orange", "Potato"]
                    .map(|product| InlineKeyboardButton::callback(product, product));

                bot.send_message(msg.chat.id, "Select a product:")
                    .reply_markup(InlineKeyboardMarkup::new([products]))
                    .await?;
                dialogue.update(State::ReceiveProductChoice { full_name }).await?;
            }
            None => {
                bot.send_message(msg.chat.id, "Please, send me your full name.").await?;
            }
        }

        Ok(())
    }

    async fn receive_product_selection(
        bot: AutoSend<Bot>,
        q: CallbackQuery,
        dialogue: MyDialogue,
        full_name: String,
    ) -> HandlerResult {
        if let Some(product) = &q.data {
            bot.send_message(
                dialogue.chat_id(),
                format!("{full_name}, product '{product}' has been purchased successfully!"),
            )
            .await?;
            dialogue.exit().await?;
        }

        Ok(())
    }
}