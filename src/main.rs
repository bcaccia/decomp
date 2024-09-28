use decompress::ExtractOptsBuilder;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    // Get the file path
    input_path: std::path::PathBuf,

    output_path: std::path::PathBuf,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

fn main() {
    let cli = Cli::parse();

    let decompresser = decompress::Decompress::default();
    let archive = cli.input_path;
    let to = cli.output_path;

    let result = decompresser.decompress(archive, to, &ExtractOptsBuilder::default().build().unwrap());

    match result {
        Ok(_) => println!("Decompression successful!"),
        Err(e) => println!("Error during decompression: {:?}", e),
    }

}
