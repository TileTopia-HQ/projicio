use clap::Parser;

#[derive(Parser)]
#[command(name = "projicio", about = "Coordinate transformation CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Transform coordinates between CRS
    Transform {
        /// Source CRS (e.g. EPSG:4326)
        #[arg(long)]
        from: String,
        /// Target CRS (e.g. EPSG:3857)
        #[arg(long)]
        to: String,
        /// X coordinate (or longitude)
        x: f64,
        /// Y coordinate (or latitude)
        y: f64,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Transform { from, to, x, y } => match projicio_core::Transform::new(&from, &to) {
            Ok(t) => match t.convert(x, y) {
                Ok((rx, ry)) => println!("{rx} {ry}"),
                Err(e) => eprintln!("Error: {e}"),
            },
            Err(e) => eprintln!("Error: {e}"),
        },
    }
}
