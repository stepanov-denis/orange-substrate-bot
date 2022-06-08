use env_logger::{Builder, Target};
mod admin;
mod command;
mod dispatching_feautures;
mod shared_state;
mod transfer;

fn main() {
    // For run with pretty_env_logger in terminal
    // pretty_env_logger::init();
    // log::info!("Starting dialogue bot...");

    // For run with env_logger and record logs to file
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();

    transfer::send::transaction();
}
