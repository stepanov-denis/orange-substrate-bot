pub mod send {
    use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

    type MyDialogue = Dialogue<State, InMemStorage<State>>;
    type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

    #[derive(Clone)]
    pub enum State {
        Start,
        ReceiveSender,
        ReceivePublicKey { sender: String },
        ReceiveAmount { sender: String, public_key: String },
    }

    impl Default for State {
        fn default() -> Self {
            Self::Start
        }
    }

    #[tokio::main]
    pub async fn transaction() {
        let bot = Bot::from_env().auto_send();

        Dispatcher::builder(
            bot,
            Update::filter_message()
                .enter_dialogue::<Message, InMemStorage<State>, State>()
                .branch(dptree::case![State::Start].endpoint(start))
                .branch(dptree::case![State::ReceiveSender].endpoint(receive_sender))
                .branch(dptree::case![State::ReceivePublicKey { sender }].endpoint(receive_public_key))
                .branch(
                    dptree::case![State::ReceiveAmount { sender, public_key }].endpoint(receive_amount),
                ),
        )
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .build()
        .setup_ctrlc_handler()
        .dispatch()
        .await;
    }

    async fn start(bot: AutoSend<Bot>, msg: Message, dialogue: MyDialogue) -> HandlerResult {
        bot.send_message(msg.chat.id, "Let's go to transfer cryptocurrency!\nEnter sender name:").await?;
        dialogue.update(State::ReceiveSender).await?;
        Ok(())
    }

    async fn receive_sender(
        bot: AutoSend<Bot>,
        msg: Message,
        dialogue: MyDialogue,
    ) -> HandlerResult {
        match msg.text() {
            Some(text) => {
                bot.send_message(msg.chat.id, "Enter the recepient public key:").await?;
                dialogue.update(State::ReceivePublicKey { sender: text.into() }).await?;
            }
            None => {
                bot.send_message(msg.chat.id, "Send me plain text.").await?;
            }
        }

        Ok(())
    }

    async fn receive_public_key(
        bot: AutoSend<Bot>,
        msg: Message,
        dialogue: MyDialogue,
        sender: String,
    ) -> HandlerResult {
        match msg.text() {
            Some(text) => {
                bot.send_message(msg.chat.id, "Enter amount:").await?;
                dialogue.update(State::ReceiveAmount { sender, public_key: text.into() }).await?;
            }
            _ => {
                bot.send_message(msg.chat.id, "Send me a plain text.").await?;
            }
        }

        Ok(())
    }

    async fn receive_amount(
        bot: AutoSend<Bot>,
        msg: Message,
        dialogue: MyDialogue,
        (sender, public_key): (String, String),
    ) -> HandlerResult {
        match msg.text() {
            Some(amount) => {
                let message = format!("Transfer from: {sender}\nTo: {public_key}\nAmount: {amount}");
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