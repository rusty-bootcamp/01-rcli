use std::fs;

use clap::Parser;
use rcli::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
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
            println!("{:?}", passwd);
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
            CryptoSubcommand::Generate(opts) => {
                let key = process_generate(&opts)?;
                match opts.format {
                    EncryptFormat::Blake3 => {
                        let path = opts.output.join("blake3.txt");
                        fs::write(path, &key[0])?;
                    }
                    EncryptFormat::Ed25519 => {
                        let path = opts.output;
                        fs::write(path.join("ed25519.sk"), &key[0])?;
                        fs::write(path.join("ed25519.pk"), &key[1])?;
                    }
                }
                println!("{:?}", key);
            }
        },
        SubCommand::Http(cmd) => match cmd {
            HttpSubCommand::Serve(opts) => {
                let _ = process_http(opts.dir, opts.port).await;
            }
        },
    }

    Ok(())
}
