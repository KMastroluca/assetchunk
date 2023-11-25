
mod pack;

mod term;

// Asset Chunk Library
use assetchunk::AssetManifest;
use pack::print_manifest_contents;

use std::io::Error;
use std::path::PathBuf;

use clap::Parser;
use clap::Subcommand;
use clap::ValueHint;

// Some Crossterm Shit
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::execute;
use std::io::{stdout, Write};





#[derive(Debug, Parser)]
#[command(name = "assetchunk")]
#[command(about = "Asset Chunking Tool For The JRPGine")]
struct CLI {
    #[command(subcommand)]
    command: Commands
}

#[derive(Debug, Subcommand)]
enum Commands {

    #[command(arg_required_else_help = true)]
    // Pack Assets Into A Chunk
    Pack {
        #[clap(value_hint = ValueHint::DirPath)]
        input_dir: Option<PathBuf>,
        #[clap(short = 'o', long = "output", value_hint = ValueHint::FilePath)]
        output_path: Option<PathBuf>,
    },

    #[command(arg_required_else_help = true)]
    List {
        #[clap(value_hint = ValueHint::FilePath)]
        asset_manifest_load_path: Option<PathBuf>,
    },

    #[command(arg_required_else_help = true)]
    Test {
        #[clap(short = 'l', long = "load", value_hint = ValueHint::FilePath)]
        asset_file_load_path: Option<PathBuf>,
    }



    // Unpack Assets From A Chunk


}

fn main() -> std::io::Result<()> {

    let args = CLI::parse();

    let mut stdout = std::io::stdout();

    term::interactive_term(&mut stdout);

    match args.command {
        Commands::List {asset_manifest_load_path} => {
            if asset_manifest_load_path.is_none() {
                
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("[-] No Asset Manifest File Specified.")
                ));

            } else {
                print_manifest_contents(asset_manifest_load_path.unwrap());
                return Ok(());
            }
        },
        Commands::Pack { input_dir, output_path } => {
            pack::pack(input_dir, output_path);
            return Ok(());
        },
        Commands::Test { asset_file_load_path } => {
            let filedata = std::fs::read(asset_file_load_path.unwrap());
            if filedata.is_ok() {
                let filedata = filedata.unwrap();
                let datastr = String::from_utf8(filedata);
                if datastr.is_ok() {
                    let datastr = datastr.unwrap();
                    let mut manifest:AssetManifest = AssetManifest::from_json(&datastr);
                    println!("{:?}", manifest);
                    manifest.load_asset_data("./out.chunk.asset");
                }
                return Ok(());
            } else {
                return Err(Error::new(std::io::ErrorKind::InvalidInput, "[-] Failed To Load Asset File From Disk."));
            }
            
        }
    }
}
