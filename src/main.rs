use clap::Parser;
use rcli::*;

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    match opts.cmd {
        SubCommand::Csv(opts) => {
            let records = read_with_csv_builder(opts.clone())?;
            serialize_record(records, opts.clone())?;

            let palyers = deserialize_csv(opts.clone())?;
            serialize_player(palyers, opts.clone())?;
        }
        SubCommand::GenPass(opts) => {
            println!("{:?}", opts);
        }
    }

    Ok(())
}
