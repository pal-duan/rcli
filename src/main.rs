use clap::Parser;
use zxcvbn::zxcvbn;

use rcli::{process_csv, process_genpass, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(csv_opts) => {
            let output = if let Some(output) = csv_opts.output {
                output.clone()
            } else {
                format!("output.{}", csv_opts.format)
            };
            process_csv(&csv_opts.input, output, csv_opts.format)?;
        }
        SubCommand::GenPass(gp_opts) => {
            let res = process_genpass(
                gp_opts.length,
                gp_opts.uppercase,
                gp_opts.lowercase,
                gp_opts.number,
                gp_opts.symbol,
            )?;
            println!("{}", res);

            let estimate = zxcvbn(&res, &[])?;
            eprintln!("Password strength: {}", estimate.score())
        }
    }
    Ok(())
}
