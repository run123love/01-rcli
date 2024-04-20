use clap::Parser;
use rcli::{process_csv, process_decode, process_encode, Base64SubCommand, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(csv_opts) => {
            process_csv(csv_opts)?;
        }
        SubCommand::Base64(subcommand) => match subcommand {
            Base64SubCommand::Encode(opts) => {
                process_encode(&opts.input, opts.format)?;
            }
            Base64SubCommand::Decode(opts) => {
                process_decode(&opts.input, opts.format)?;
            }
        },
    }

    Ok(())
}
