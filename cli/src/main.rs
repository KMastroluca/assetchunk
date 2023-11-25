
mod pack;


// Asset Chunk Library
use assetchunk::AssetManifest;
use pack::print_manifest_contents;

use std::path::PathBuf;

use clap::Parser;
use clap::Subcommand;
use clap::ValueHint;


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

fn main() {

    let args = CLI::parse();

    match args.command {
        Commands::List {asset_manifest_load_path} => {
            if asset_manifest_load_path.is_none() {
                println!("No Asset Manifest Load Path Specified");
                return;
            }
            print_manifest_contents(asset_manifest_load_path.unwrap());
        },
        Commands::Pack { input_dir, output_path } => {
            pack::pack(input_dir, output_path);
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
            }
        }
    }


}
