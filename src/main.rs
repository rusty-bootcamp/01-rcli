use clap::Parser;
use rcli::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let opts = Opts::parse();

    match opts.cmd {
        SubCommand::Csv(opts) => opts.execute().await?,
        SubCommand::GenPass(opts) => opts.execute().await?,
        SubCommand::Base64(opts) => opts.execute().await?,
        SubCommand::Crypto(opts) => opts.execute().await?,
        SubCommand::Http(opts) => opts.execute().await?,
    }

    Ok(())
}
