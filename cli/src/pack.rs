
use assetchunk::Asset;
use assetchunk::AssetType;
use assetchunk::AssetManifest;


use clap::builder::OsStr;
use file_format::{FileFormat, Kind};
use std::path::PathBuf;
use std::fs::write;

/**
 * Pack assets from a directory into a chunk, then map the chunk to a manifest file.
 * and export the chunk and manifest to a specified output directory.
 */
pub fn pack(input_dir:Option<PathBuf>, output_path:Option<PathBuf>) {
      
      println!("Packing Assets Into Chunk...");

      // Load the assets from the input directory into an array of assets
      println!("[+] Loading Assets...");
      let assets = load_assets(input_dir);
      println!("[+] Loaded {} Assets", assets.len());
      
      // Create a new asset manifest from asset array
      println!("[+] Creating Asset Manifest...");
      let mut manifest = AssetManifest::new(assets);


      // Pack the assets into a contiguous chunk of memory
      let mut chunk = Vec::new();
      println!("[+] Packing Assets Into Chunk...");
      unsafe {
            chunk = pack_assets(manifest.get_assets_mut());
      }
      


      if (chunk.is_empty()) {
            println!("Error: Chunk is empty after packing assets.");
            return;
      }

      println!("[+] Chunk Size: {} Bytes", chunk.len());
      println!("[+] Chunk Packed Successfully!");

      let mut chunk_path = PathBuf::from(output_path.clone().unwrap());
      let mut manifest_path = PathBuf::from(output_path.clone().unwrap());

      let mut filename = String::from(chunk_path.file_name().unwrap().to_str().unwrap());
      filename.push_str(".chunk.asset");
      let new_chunk_path = chunk_path.with_file_name(filename);

      filename = String::from(manifest_path.file_name().unwrap().to_str().unwrap());
      filename.push_str(".manifest.asset");
      let new_manifest_path = manifest_path.with_file_name(filename);



      //println!("[+] Writing Chunk To File: {:?}", );
      let write_result = write( new_chunk_path, chunk);
      match write_result {
            Ok(_) => {
               println!("Chunk Written Successfully!")
            },
            Err(e) => {
                  println!("Error: Failed To Write Chunk To File: {:?}", e);
                  return;
            }
      }
      // Write The Manifest To Disk
      //println!("[+] Writing Manifest To File: {:?}", );
      manifest.write(new_manifest_path.to_str().unwrap());

      println!("Assets Packed Successfully!");


}

/**
 * Load assets from a directory into an array of assets
 */
pub fn load_assets(input_dir:Option<PathBuf>) -> Vec<Asset> {
      let mut assets:Vec<Asset> = Vec::new();
      let input_dir = input_dir.unwrap();
      for entry in std::fs::read_dir(input_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let asset_type = AssetType::Other;
            let asset = Asset::load(path.to_str().unwrap(), file_name, asset_type);
            assets.push(asset);
      }
      assets
}


/**
 * This function identifies the asset type based on the makeup of the file.
 * It uses the file format crate to identify the file type.
 * ---
 * This may not exactly work all the way for all file types, but it should work for most.
 * It will be updated as needed.
 */
pub fn identify_asset(data:&[u8]) -> AssetType {
   let format = FileFormat::from_bytes(data);
   match format.kind() {
      Kind::Image => {
         match format.short_name() {
            Some("PNG") => AssetType::Image,
            Some("JPG") => AssetType::Image,
            Some("JPEG") => AssetType::Image,
            Some("GIF") => AssetType::Image,
            Some("BMP") => AssetType::Image,
            Some("WEBP") => AssetType::Image,
            Some("ICO") => AssetType::Image,
            Some("TIFF") => AssetType::Image,
            Some("PICT") => AssetType::Image,
            Some("PSD") => AssetType::Image,
            Some("SVG") => AssetType::Image,
            Some(_) => AssetType::Other,
            None => AssetType::Other,
         }
      },
      Kind::Audio => {
         match format.short_name() {
            Some("MP3") => AssetType::Audio,
            Some("WAV") => AssetType::Audio,
            Some("AIFF") => AssetType::Audio,
            Some("FLAC") => AssetType::Audio,
            Some("OGG") => AssetType::Audio,
            Some("MIDI") => AssetType::Audio,
            Some("AAC") => AssetType::Audio,
            Some("WMA") => AssetType::Audio,
            Some("3GP") => AssetType::Audio,
            Some(_) => AssetType::Other,
            None => AssetType::Other,
         }
      },
      Kind::Model => {
         match format.short_name() {
            Some("OBJ") => AssetType::Model,
            Some("FBX") => AssetType::Model,
            Some("MAX") => AssetType::Model,
            Some("3DS") => AssetType::Model,
            Some("C4D") => AssetType::Model,
            Some("BLEND") => AssetType::Model,
            Some("MAYA") => AssetType::Model,
            Some(_) => AssetType::Other,
            None => AssetType::Other,
         }
      },
      Kind::Text => {
         match format.short_name() {
            Some("SHADER") => AssetType::Shader,
            Some("GLSL") => AssetType::Shader,
            Some("HLSL") => AssetType::Shader,
            Some("CGFX") => AssetType::Shader,
            Some("FX") => AssetType::Shader,
            Some("FXH") => AssetType::Shader,
            Some("FXO") => AssetType::Shader,
            Some("FXB") => AssetType::Shader,
            Some("FXC") => AssetType::Shader,
            Some("FXL") => AssetType::Shader,
            Some("FXM") => AssetType::Shader,
            Some("FXP") => AssetType::Shader,
            Some("FXR") => AssetType::Shader,
            Some("FXS") => AssetType::Shader,
            Some("FXV") => AssetType::Shader,
            Some("FXZ") => AssetType::Shader,
            Some("GLSLV") => AssetType::Shader,
            Some("GLSLF") => AssetType::Shader,
            Some("GLSLG") => AssetType::Shader,
            Some("GLSLC") => AssetType::Shader,

            Some("JS") => AssetType::Script,
            Some("PY") => AssetType::Script,
            Some("LUA") => AssetType::Script,
            Some("VB") => AssetType::Script,     
            Some(_) => AssetType::Other,
            None => AssetType::Other,  
         }
      },
      Kind::Font => {
         match format.short_name() {
            Some("TTF") => AssetType::Font,
            Some("OTF") => AssetType::Font,
            Some("WOFF") => AssetType::Font,
            Some("WOFF2") => AssetType::Font,
            Some("EOT") => AssetType::Font,
            Some("SVG") => AssetType::Font,
            Some(_) => AssetType::Other,
            None => AssetType::Other,
         }
      },
      _ => AssetType::Other,
   }
}


/**
 * Pack assets into a contiguous chunk of memory
 */
pub unsafe fn pack_assets(assets: &mut Vec<Asset>) -> Vec<u8> {
      println!("[+] Packing Assets Into Chunk...");
      let mut chunk:Vec<u8> = Vec::new();
      let mut offset:usize = 0;
      for asset in assets {
            asset.set_chunk_location(offset); // Set the location of the asset in the chunk at the beginnging
            // of the loop, so that it accurately reflects the offset of the asset in the chunk.
            let asset_size = asset.get_size();
            let asset_data = asset.get_data();
            if asset_data.is_none() {
                  println!("Error: Asset Data Doesnt Exist.");
                  return Vec::new();
            }
            chunk.extend_from_slice(asset_data.as_ref().unwrap());
            offset += asset_size;
            
      }
      chunk
}