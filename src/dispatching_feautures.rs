pub mod dispatching {
    use rand::Rng;

    use teloxide::{
        prelude::*,
        types::{Dice, Update, UserId},
        utils::command::BotCommands,
    };

    #[tokio::main]
    pub async fn features() {
        // pretty_env_logger::init();
        // log::info!("Starting dispatching features bot...");

        let bot = Bot::from_env().auto_send();

        let parameters = ConfigParameters {
            bot_maintainer: UserId(0), // Paste your ID to run this bot.
            maintainer_username: None,
        };

        let handler = Update::filter_message()
            // You can use branching to define multiple ways in which an update will be handled. If the
            // first branch fails, an update will be passed to the second branch, and so on.
            .branch(
                dptree::entry()
                    // Filter commands: the next handlers will receive a parsed `SimpleCommand`.
                    .filter_command::<SimpleCommand>()
                    // If a command parsing fails, this handler will not be executed.
                    .endpoint(simple_commands_handler),
            )
            .branch(
                // Filter a maintainer by a used ID.
                dptree::filter(|msg: Message, cfg: ConfigParameters| {
                    msg.from()
                        .map(|user| user.id == cfg.bot_maintainer)
                        .unwrap_or_default()
                })
                .filter_command::<MaintainerCommands>()
                .endpoint(
                    |msg: Message, bot: AutoSend<Bot>, cmd: MaintainerCommands| async move {
                        match cmd {
                            MaintainerCommands::Rand { from, to } => {
                                let mut rng = rand::rngs::OsRng::default();
                                let value: u64 = rng.gen_range(from..=to);

                                bot.send_message(msg.chat.id, value.to_string()).await?;
                                Ok(())
                            }
                        }
                    },
                ),
            )
            .branch(
                // Filtering allow you to filter updates by some condition.
                dptree::filter(|msg: Message| msg.chat.is_group() || msg.chat.is_supergroup())
                    // An endpoint is the last update handler.
                    .endpoint(|msg: Message, bot: AutoSend<Bot>| async move {
                        log::info!("Received a message from a group chat.");
                        bot.send_message(msg.chat.id, "This is a group chat.")
                            .await?;
                        respond(())
                    }),
            )
            .branch(
                // There are some extension filtering functions on `Message`. The following filter will
                // filter only messages with dices.
                Message::filter_dice().endpoint(
                    |msg: Message, dice: Dice, bot: AutoSend<Bot>| async move {
                        bot.send_message(msg.chat.id, format!("Dice value: {}", dice.value))
                            .reply_to_message_id(msg.id)
                            .await?;
                        Ok(())
                    },
                ),
            );

        Dispatcher::builder(bot, handler)
            // Here you specify initial dependencies that all handlers will receive; they can be
            // database connections, configurations, and other auxiliary arguments. It is similar to
            // `actix_web::Extensions`.
            .dependencies(dptree::deps![parameters])
            // If no handler succeeded to handle an update, this closure will be called.
            .default_handler(|upd| async move {
                log::warn!("Unhandled update: {:?}", upd);
            })
            // If the dispatcher fails for some reason, execute this handler.
            .error_handler(LoggingErrorHandler::with_custom_text(
                "An error has occurred in the dispatcher",
            ))
            .build()
            .setup_ctrlc_handler()
            .dispatch()
            .await;
    }

    #[derive(Clone)]
    struct ConfigParameters {
        bot_maintainer: UserId,
        maintainer_username: Option<String>,
    }

    #[derive(BotCommands, Clone)]
    #[command(rename = "lowercase", description = "Simple commands")]
    enum SimpleCommand {
        #[command(description = "shows this message.")]
        Help,
        #[command(description = "shows maintainer info.")]
        Maintainer,
        #[command(description = "shows your ID.")]
        MyId,
    }

    #[derive(BotCommands, Clone)]
    #[command(rename = "lowercase", description = "Maintainer commands")]
    enum MaintainerCommands {
        #[command(parse_with = "split", description = "generate a number within range")]
        Rand { from: u64, to: u64 },
    }

    async fn simple_commands_handler(
        msg: Message,
        bot: AutoSend<Bot>,
        cmd: SimpleCommand,
        cfg: ConfigParameters,
        me: teloxide::types::Me,
    ) -> Result<(), teloxide::RequestError> {
        let text = match cmd {
            SimpleCommand::Help => {
                if msg.from().unwrap().id == cfg.bot_maintainer {
                    format!(
                        "{}\n\n{}",
                        SimpleCommand::descriptions(),
                        MaintainerCommands::descriptions()
                    )
                } else if msg.chat.is_group() || msg.chat.is_supergroup() {
                    SimpleCommand::descriptions()
                        .username_from_me(&me)
                        .to_string()
                } else {
                    SimpleCommand::descriptions().to_string()
                }
            }
            SimpleCommand::Maintainer => {
                if msg.from().unwrap().id == cfg.bot_maintainer {
                    "Maintainer is you!".into()
                } else if let Some(username) = cfg.maintainer_username {
                    format!("Maintainer is @{username}")
                } else {
                    format!("Maintainer ID is {}", cfg.bot_maintainer)
                }
            }
            SimpleCommand::MyId => {
                format!("{}", msg.from().unwrap().id)
            }
        };

        bot.send_message(msg.chat.id, text).await?;

        Ok(())
    }
}
