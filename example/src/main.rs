#![allow(non_snake_case)]
mod IRemoteService;
mod cli;
mod client;
mod server;

fn main() -> anyhow::Result<()> {
    cli::run()
}
