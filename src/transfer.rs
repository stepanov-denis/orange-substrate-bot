pub mod send {
    use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

    type MyDialogue = Dialogue<State, InMemStorage<State>>;
    type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

    #[derive(Clone)]
    pub enum State {
        Start,
        ReceiveSender,
        ReceivePublicKey {
            sender: String,
        },
        ReceiveAmount {
            sender: String,
            public_key: String,
        },
        TransferConfirm {
            sender: String,
            public_key: String,
            amount: String,
        },
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
                .branch(
                    dptree::case![State::ReceivePublicKey { sender }].endpoint(receive_public_key),
                )
                .branch(
                    dptree::case![State::ReceiveAmount { sender, public_key }]
                        .endpoint(receive_amount),
                )
                .branch(
                    dptree::case![State::TransferConfirm {
                        sender,
                        public_key,
                        amount
                    }]
                    .endpoint(transfer_confirm),
                ),
        )
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .build()
        .setup_ctrlc_handler()
        .dispatch()
        .await;
    }

    async fn start(bot: AutoSend<Bot>, msg: Message, dialogue: MyDialogue) -> HandlerResult {
        bot.send_message(
            msg.chat.id,
            "Let's go to transfer cryptocurrency!\nEnter sender name:",
        )
        .await?;
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
                bot.send_message(msg.chat.id, "Enter the recepient public key:")
                    .await?;
                dialogue
                    .update(State::ReceivePublicKey {
                        sender: text.into(),
                    })
                    .await?;
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
                dialogue
                    .update(State::ReceiveAmount {
                        sender,
                        public_key: text.into(),
                    })
                    .await?;
            }
            _ => {
                bot.send_message(msg.chat.id, "Send me a plain text.")
                    .await?;
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
            Some(text) => {
                let message = format!("Transfer from: {sender}\nTo: {public_key}\nAmount: {text}\nFor confirm transfer enter: confirm transfer\nFor cancel transfer enter: some text");
                bot.send_message(msg.chat.id, message).await?;
                dialogue
                    .update(State::TransferConfirm {
                        sender,
                        public_key,
                        amount: text.into(),
                    })
                    .await?;
                // dialogue.exit().await?;
            }
            None => {
                bot.send_message(msg.chat.id, "Send me plain text.").await?;
            }
        }
        Ok(())
    }

    async fn transfer_confirm(
        bot: AutoSend<Bot>,
        msg: Message,
        dialogue: MyDialogue,
        (sender, public_key, amount): (String, String, String),
    ) -> HandlerResult {
        match msg.text() {
            Some(text) => {
                if text == "confirm transfer" {
                    let message = format!("Transfer from: {sender}\nTo: {public_key}\nAmount: {amount}\nState transfer: Ok");
                    bot.send_message(msg.chat.id, message).await?;
                    dialogue.exit().await?;
                } else {
                    let message = format!("Transfer from: {sender}\nTo: {public_key}\nAmount: {amount}\nState transfer: Cancel");
                    bot.send_message(msg.chat.id, message).await?;
                    dialogue.exit().await?;
                }
            }
            None => {
                bot.send_message(msg.chat.id, "For confirm transfer enter: confirm transfer\nFor cancel transfer enter: some text").await?;
            }
        }
        Ok(())
    }
}
