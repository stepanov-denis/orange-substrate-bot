pub mod dialogues_logic {
    use env_logger::{Builder, Target};
    use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

    type MyDialogue = Dialogue<State, InMemStorage<State>>;
    type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

    #[derive(Clone)]
    pub enum State {
        Start,
        ReceiveFullName,
        ReceiveAge { full_name: String },
        ReceiveLocation { full_name: String, age: u8 },
    }

    impl Default for State {
        fn default() -> Self {
            Self::Start
        }
    }

    #[tokio::main]
    pub async fn dialog() {
        // For run with pretty_env_logger in terminal
        // pretty_env_logger::init();
        // log::info!("Starting dialogue bot...");

        // For run with env_logger and record logs to file
        let mut builder = Builder::from_default_env();
        builder.target(Target::Stdout);
        builder.init();

        let bot = Bot::from_env().auto_send();

        Dispatcher::builder(
            bot,
            Update::filter_message()
                .enter_dialogue::<Message, InMemStorage<State>, State>()
                .branch(dptree::case![State::Start].endpoint(start))
                .branch(dptree::case![State::ReceiveFullName].endpoint(receive_full_name))
                .branch(dptree::case![State::ReceiveAge { full_name }].endpoint(receive_age))
                .branch(
                    dptree::case![State::ReceiveLocation { full_name, age }]
                        .endpoint(receive_location),
                ),
        )
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .build()
        .setup_ctrlc_handler()
        .dispatch()
        .await;
    }

    async fn start(bot: AutoSend<Bot>, msg: Message, dialogue: MyDialogue) -> HandlerResult {
        bot.send_message(msg.chat.id, "Let's start! What's your full name?")
            .await?;
        dialogue.update(State::ReceiveFullName).await?;
        Ok(())
    }

    async fn receive_full_name(
        bot: AutoSend<Bot>,
        msg: Message,
        dialogue: MyDialogue,
    ) -> HandlerResult {
        match msg.text() {
            Some(text) => {
                bot.send_message(msg.chat.id, "How old are you?").await?;
                dialogue
                    .update(State::ReceiveAge {
                        full_name: text.into(),
                    })
                    .await?;
            }
            None => {
                bot.send_message(msg.chat.id, "Send me plain text.").await?;
            }
        }

        Ok(())
    }

    async fn receive_age(
        bot: AutoSend<Bot>,
        msg: Message,
        dialogue: MyDialogue,
        full_name: String, // Available from `State::ReceiveAge`.
    ) -> HandlerResult {
        match msg.text().map(|text| text.parse::<u8>()) {
            Some(Ok(age)) => {
                bot.send_message(msg.chat.id, "What's your location?")
                    .await?;
                dialogue
                    .update(State::ReceiveLocation { full_name, age })
                    .await?;
            }
            _ => {
                bot.send_message(msg.chat.id, "Send me a number.").await?;
            }
        }

        Ok(())
    }

    async fn receive_location(
        bot: AutoSend<Bot>,
        msg: Message,
        dialogue: MyDialogue,
        (full_name, age): (String, u8), // Available from `State::ReceiveLocation`.
    ) -> HandlerResult {
        match msg.text() {
            Some(location) => {
                let message = format!("Full name: {full_name}\nAge: {age}\nLocation: {location}");
                bot.send_message(msg.chat.id, message).await?;
                dialogue.exit().await?;
            }
            None => {
                bot.send_message(msg.chat.id, "Send me plain text.").await?;
            }
        }

        Ok(())
    }
}
