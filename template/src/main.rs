use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    name: String,
}

fn main() {
    let cli = Cli::parse();
    println!("Hello, {}!", cli.name);
}
