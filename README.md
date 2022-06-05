# Orange Substrate Bot
Telegram bot for substrate wallet
## Run docker images locally
* Run in the background
```
$ docker-compose up -d
```
* Look at the logs
```
$ docker-compose logs -f
```
* Tear it all down
```
$ docker-compose down
```
* Tear it all down with removing volumes
```
$ docker-compose down --volumes
```
## Prerequisites outside docker
* Install Rust for Linux or macOS
```
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```
For Windows, visit [this page](https://www.rust-lang.org/tools/install)
## Initial setup outside docker
* Clone the repository
```
$ git clone git@github.com:stepanov-denis/ats-monitoring.git
```
* Create a new bot using [@Botfather](https://t.me/botfather) to get a token in the format 123456789:blablabla
* Initialise the TELOXIDE_TOKEN environmental variable to your token:
```
# Unix-like
$ export TELOXIDE_TOKEN=<Your token here>

# Windows command line
$ set TELOXIDE_TOKEN=<Your token here>

# Windows PowerShell
$ $env:TELOXIDE_TOKEN=<Your token here>
```
* Run with printing logs from env_logger in your terminal
```
$ RUST_LOG=trace cargo run --release
```
* Run with logs from env_logger with writing to a file
```
$ RUST_LOG=trace cargo run --release > log.txt
```
* Run with cargo-make
```
$ cargo make --makefile script.toml app
```