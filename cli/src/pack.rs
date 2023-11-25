
use assetchunk::Asset;
use assetchunk::AssetType;
use assetchunk::AssetManifest;


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
      if assets.is_none() {
            eprintln!("[-] Error: Failed To Load Assets.");
            return;
      }
      let assets = assets.unwrap();
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

      println!("[+] Chunk Size: {} Bytes / {} KB / {} MB", chunk.len(), chunk.len() / 1024, chunk.len() / (1024 * 1024));
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
               println!("[+] Chunk Written Successfully!")
            },
            Err(e) => {
                  println!("[-] Error: Failed To Write Chunk To File: {:?}", e);
                  return;
            }
      }
      // Write The Manifest To Disk
      //println!("[+] Writing Manifest To File: {:?}", );
      manifest.write(new_manifest_path.to_str().unwrap());

      println!("[+] Assets Packed Successfully!");


}

/**
 * Load assets from a directory into an array of assets
 */
pub fn load_assets(input_dir:Option<PathBuf>) -> Option<Vec<Asset>> {
      let mut assets:Vec<Asset> = Vec::new();
      let input_dir = input_dir.unwrap();
      if input_dir.is_dir() == false {
            eprintln!("[-] Error: Input Directory Does Not Exist. {:?}", input_dir);
            eprintln!("[-] Current Working Directory: {:?}", std::env::current_dir().unwrap());
            return None;
      }
      for entry in std::fs::read_dir(input_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            // We only want the file name not its extension
            let file_name = path.file_stem().unwrap().to_str().unwrap();
            let asset_type = AssetType::Other("UNIDENTIFIED".to_string()); // Default Asset Type
            let asset = Asset::load(path.to_str().unwrap(), file_name, asset_type);
            assets.push(asset);
      }
      Some(assets)
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


      println!("[+]Chunk Contents:");
      for asset in assets {
            println!("[|]- Asset Name: {}", asset.get_name());
            println!("[|]- Asset Type: {:?}", asset.get_type());
            println!("[|]- Asset Size: {} Bytes / {} MB", asset.get_size(), asset.get_size() / (1024 * 1024));
            println!("[|]- Asset Offset: {} Bytes", asset.get_chunk_location());
            println!("[|]- Asset Data: {:?}", asset.get_data());
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