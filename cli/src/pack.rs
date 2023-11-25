
use assetchunk::Asset;
use assetchunk::AssetType;
use assetchunk::AssetManifest;


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
      println!("[+] Packing Assets Into Chunk...");
      let chunk = pack_assets(manifest.get_assets_mut());
      
      


      if chunk.is_empty() {
            println!("Error: Chunk is empty after packing assets.");
            return;
      }

      println!("[+] Chunk Size: {} Bytes", chunk.len());
      println!("[+] Chunk Packed Successfully!");

      let chunk_path = PathBuf::from(output_path.clone().unwrap());
      let manifest_path = PathBuf::from(output_path.clone().unwrap());

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
            // We only want the file name not its extension
            let file_name = path.file_stem().unwrap().to_str().unwrap();
            let asset_type = AssetType::Other("UNIDENTIFIED".to_string()); // Default Asset Type
            let asset = Asset::load(path.to_str().unwrap(), file_name, asset_type);
            assets.push(asset);
      }
      assets
}

/**
 * Identify Assets After The Data Has Been Loaded
 */
pub fn identify_assets(assets:&mut Vec<Asset>) {
      println!("[+] Identifying Assets...");
      for asset in assets {
            println!("[+] Identifying Asset: {}", asset.get_name());
            let asset_data = asset.get_data().as_deref().expect("[-] Error While Identifying Asset: Asset Data Doesnt Exist.");
            let asset_type = identify_asset(asset_data);
            asset.set_type(asset_type);
      }
      println!("[+] Assets Identified Successfully!");
}



/**
 * Print A List Of The Contents Of The Chunk
 */
pub fn print_manifest_contents(manifest_path:PathBuf) {

      // Load The Manifest From Disk
      let manifest_file = std::fs::read(manifest_path);
      if manifest_file.is_err() {
            println!("[-] Error: Failed To Load Manifest From Disk.");
            return;
      }
      let manifest_file = manifest_file.unwrap();
      let manifest_file = String::from_utf8(manifest_file);
      if manifest_file.is_err() {
            println!("[-] Error: Failed To Get Manifest As JSON String.");
            return;
      }
      let manifest_file = manifest_file.unwrap();
      let mut manifest = AssetManifest::from_json(&manifest_file);
 
   
      let assets = manifest.get_assets_mut();


      println!("Chunk Contents:");
      for asset in assets {
            println!("Asset Name: {}", asset.get_name());
            println!("Asset Type: {:?}", asset.get_type());
            println!("Asset Size: {} Bytes", asset.get_size());
            println!("Asset Offset: {} Bytes", asset.get_chunk_location());
            println!("Asset Data: {:?}", asset.get_data());
      }
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
            Some("PNG") | Some("png") => AssetType::Image("PNG".to_string()),
            Some("JPG") | Some("jpg") => AssetType::Image("JPG".to_string()),
            Some("JPEG") | Some("jpeg") => AssetType::Image("JPEG".to_string()),
            Some("GIF") | Some("gif") => AssetType::Image("GIF".to_string()),
            Some("BMP") | Some("bmp") => AssetType::Image("BMP".to_string()),
            Some("WEBP") | Some("webp") => AssetType::Image("WEBP".to_string()),
            Some("ICO") | Some("ico") => AssetType::Image("ICO".to_string()),
            Some("TIFF") | Some("tiff") => AssetType::Image("TIFF".to_string()),
            Some("PICT") | Some("pict") => AssetType::Image("PICT".to_string()),
            Some("PSD") | Some("psd") => AssetType::Image("PSD".to_string()),
            Some("SVG") | Some("svg") => AssetType::Image("SVG".to_string()),
            Some(_) => AssetType::Other("IMAGE".to_string()), // Supply A Generic Name For The Image Type
            None => AssetType::Other("IMAGE".to_string()),
         }
      },
      Kind::Audio => {
         match format.short_name() {
            Some("MP3") | Some("mp3") => AssetType::Audio("MP3".to_string()),
            Some("WAV") | Some("wav") => AssetType::Audio("WAV".to_string()),
            Some("AIFF") | Some("aiff") => AssetType::Audio("AIFF".to_string()),
            Some("FLAC") | Some("flac") => AssetType::Audio("FLAC".to_string()),
            Some("OGG") | Some("ogg") => AssetType::Audio("OGG".to_string()),
            Some("MIDI") | Some("midi") => AssetType::Audio("MIDI".to_string()),
            Some("AAC") | Some("aac") => AssetType::Audio("AAC".to_string()),
            Some("WMA") | Some("wma") => AssetType::Audio("WMA".to_string()),
            Some("3GP") | Some("3gp") => AssetType::Audio("3GP".to_string()),
            Some(_) => AssetType::Other("AUDIO".to_string()),
            None => AssetType::Other("AUDIO".to_string()),
         }
      },
      Kind::Model => {
         match format.short_name() {
            Some("OBJ") | Some("obj") => AssetType::Model("OBJ".to_string()),
            Some("FBX") | Some("fbx") => AssetType::Model("FBX".to_string()),
            Some("MAX") | Some("max") => AssetType::Model("MAX".to_string()),
            Some("3DS") | Some("3ds") => AssetType::Model("3DS".to_string()),
            Some("C4D") | Some("c4d") => AssetType::Model("C4D".to_string()),
            Some("BLEND") | Some("blend") => AssetType::Model("BLEND".to_string()),
            Some("MAYA") | Some("maya") => AssetType::Model("MAYA".to_string()),
            Some(_) => AssetType::Other("MODEL".to_string()),
            None => AssetType::Other("MODEL".to_string()),
         }
      },
      Kind::Text => {
         match format.short_name() {
            Some("SHADER") | Some("shader") => AssetType::Shader("SHADER".to_string()),
            Some("GLSL") | Some("glsl") => AssetType::Shader("GLSL".to_string()),
            Some("HLSL") | Some("hlsl") => AssetType::Shader("HLSL".to_string()),
            Some("CGFX") | Some("cgfx") => AssetType::Shader("CGFX".to_string()),
            Some("FX") | Some("fx") => AssetType::Shader("FX".to_string()),

            Some("JS") | Some("js") => AssetType::Script("JS".to_string()),
            Some("PY") | Some("py") => AssetType::Script("PY".to_string()),
            Some("LUA")| Some("lua") => AssetType::Script("LUA".to_string()),
            Some("VB") | Some("vb") => AssetType::Script("VB".to_string()),

            // For Dialog Scripts
            Some("SCRIPT") | Some("script") => AssetType::Script("SCRIPT".to_string()),     
            Some(_) => AssetType::Other("TEXT".to_string()),
            None => AssetType::Other("TEXT".to_string()),  
         }
      },
      Kind::Font => {
         match format.short_name() {
            Some("TTF") | Some("ttf") => AssetType::Font("TTF".to_string()),
            Some("OTF") | Some("otf") => AssetType::Font("OTF".to_string()),
            Some("WOFF")| Some("woff") => AssetType::Font("WOFF".to_string()),
            Some("WOFF2")| Some("woff2") => AssetType::Font("WOFF2".to_string()),
            Some("EOT") | Some("eot") => AssetType::Font("EOT".to_string()),
            Some("SVG") | Some("svg") => AssetType::Font("SVG".to_string()),
            Some(_) => AssetType::Other("FONT".to_string()),
            None => AssetType::Other("FONT".to_string()),
         }
      },
      _ => AssetType::Other("UNIDENTIFIED".to_string()),
   }
}


/**
 * Pack assets into a contiguous chunk of memory
 */
pub fn pack_assets(assets: &mut Vec<Asset>) -> Vec<u8> {
      println!("[+] Packing Assets Into Chunk...");
      let mut chunk:Vec<u8> = Vec::new();
      let mut offset:usize = 0;
      for asset in assets {
            println!("[+] Processing Asset: {}", asset.get_name());
            asset.set_chunk_location(offset); // Set the location of the asset in the chunk at the beginnging
            // of the loop, so that it accurately reflects the offset of the asset in the chunk.
            let asset_size = asset.get_size();
            println!("Asset Size: {}", asset_size);
            let asset_data = asset.get_data();
            println!("Asset Data: {:?}", asset_data);
            if asset_data.is_none() {
                  println!("Error: Asset Data Doesnt Exist.");
                  return Vec::new();
            }
            // Extend the asset chunk using extend/into_iter
            let asset_data_slice = asset_data.as_ref().unwrap().as_slice(); // Finegling the bagel
            chunk.extend(asset_data_slice.into_iter());
            offset += asset_size; // Pointer arithmetic to get the offset of the next asset in the chunk
            println!("Next Asset Offset: {}", offset);     
      }
      chunk
}