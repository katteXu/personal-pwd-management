use anyhow::bail;
use clap::Parser;
use encryptor::password::generate_password;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    seed: String,

    #[clap(short, long, default_value_t = 16)]
    length: usize,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    if args.seed.len() < 4 {
        bail!("seed {} must be at least 4 characters long", &args.seed);
    }

    let (seed, length) = (args.seed, args.length);

    let password = generate_password(&seed, length);

    match password {
        Ok(p) => println!("{}", p),
        Err(e) => bail!("{}", e),
    }

    Ok(())
}
