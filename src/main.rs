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
            let passwd = process_passwd(&opts)?;
            println!("{}", passwd);
        }
        SubCommand::Base64(opts) => match opts {
            Base64Subcommand::Encode(opts) => {
                let encoded = process_encode(&opts)?;
                println!("{:?}", encoded);
            }
            Base64Subcommand::Decode(opts) => {
                let decoded = process_decode(&opts)?;
                println!("{:?}", decoded);
            }
        },
        SubCommand::Crypto(opts) => match opts {
            CryptoSubcommand::Encrypt(opts) => {
                let encrypted = process_encrypt(&opts)?;
                println!("{:?}", encrypted);
            }
            CryptoSubcommand::Decrypt(opts) => {
                let decrypted = process_decrypt(&opts)?;
                println!("{:?}", decrypted);
            }
        },
    }

    Ok(())
}
