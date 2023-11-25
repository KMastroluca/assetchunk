use serde::{Serialize, Deserialize};
use std::ffi::{CStr, c_void, c_char, CString};
use std::fs::{write, read};
use std::path::PathBuf;
use std::str::FromStr;




#[derive(Serialize, Deserialize, Debug)]
pub enum AssetType {
    Image(String),
    Audio(String),
    Font(String),
    Shader(String),
    Model(String),
    Script(String),
    Other(String),
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Asset {
    name: String, // Asset Name
    asset_type: AssetType, // Asset Type
    location: usize, // Location Of Asset In Chunk
    size: usize, // Size Of Asset In Bytes
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    data: Option<Vec<u8>>, // Vector Of Bytes, Representing The Asset Data
}



impl Asset {

    /**
     * Load Asset From File
     */
    pub fn load(filepath:&str, asset_name:&str, asset_type:AssetType) -> Self {
        let file_data = read(filepath).unwrap();
        let file_size = file_data.len();
        Asset {
            name: asset_name.to_string(),
            asset_type: asset_type,
            location: 0,
            size: file_size,
            data: Some(file_data),
        }
    }

    /**
     * Set Asset Location In Chunk
     */
    pub fn set_chunk_location(&mut self, location:usize) {
        self.location = location;
    }


    /**
     * Get Asset Location In Chunk
     */
    pub fn get_chunk_location(&self) -> usize {
        self.location
    }

    /**
     * Get Asset Size In Bytes
     */
    pub fn get_size(&self) -> usize {
        self.size
    }

    /**
     * Get Asset Data
     */
    pub fn get_data(&self) -> &Option<Vec<u8>> {
        &self.data
    }

    /**
     * Get Asset Name
     */
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /**
     * Get Asset Type
     **/
    pub fn get_type(&self) -> &AssetType {
        &self.asset_type
    }

    /**
     * Set Asset Type
     */
    pub fn set_type(&mut self, asset_type:AssetType) {
        self.asset_type = asset_type;
    }


}


#[derive(Serialize, Deserialize, Debug)]
pub struct AssetManifest {
    assets: Vec<Asset>,
}



impl AssetManifest {
    
    /**
     * Create A New Asset Manifest From An Array Of Assets
     */
    pub fn new(assets:Vec<Asset>) -> Self {
        AssetManifest {
            assets: assets,
        }
    }

    /**
     * Get Asset Manifest Assets
     *
    */
    pub fn get_assets(&self) -> &Vec<Asset> {
        &self.assets
    }


    /**
     * Get Asset Manifest Assets As A Mutable Reference
     */
    pub fn get_assets_mut(&mut self) -> &mut Vec<Asset> {
        &mut self.assets
    }


    /**
     * Serialize The Asset Manifest To A JSON String
     */
    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }


    /**
     * Load An Asset Manifest From A JSON String
     */
    pub fn from_json(json:&str) -> Self {
        serde_json::from_str(json).unwrap()
    }



    /*
    * Load Asset Data From Asset Chunk File
    * You do this to sort of edit the contents of the chunk
    * however using this in a game requires loading the while chunk into memory at once
    * in one contiguous block of memory
    * 
     */
    pub fn load_asset_data(&mut self, filepath:&str) {
        println!("[+] Loading Asset Data From Chunk: {:?}...", filepath);
        let filedata = read(filepath);
        println!("File Data: {:?}", filedata);
        if filedata.is_ok() {
            println!("[+] Chunk Loaded Successfully!");
            let data = filedata.unwrap();

            println!("[+] Loaded {} Bytes", data.len());
            for asset in self.assets.iter_mut() {
                println!("Asset: {:?}", asset);
                let location = asset.get_chunk_location();
                let size = asset.get_size();
                let data = &data[location..location+size];
                println!("Asset: {:?} Size: {}", asset.get_name(), data.len());
                asset.data = Some(data.to_vec());
            }

        }
    }


    /**
     * Write The Asset Manifest To A File
     */
    pub fn write(&self, filepath:&str) {
        let json = self.to_json();
        write(filepath, json).unwrap();
    }


}



/**
 * API: - Represents A Single Asset Within The Chunk
 *  
 */
#[repr(C)]
#[derive(Debug,Clone, Copy)]
pub struct RawCAsset {
    pub data: *mut c_void,
    pub location: usize,
    pub size: usize,
    pub name: *const c_char,
}

/**
 * API: - Represents The Asset Manifest
 */
#[repr(C)]
#[derive(Debug,Clone)]
pub struct RawCAssetManifest {
    pub assets: *mut *mut RawCAsset, // Remember This is an array of assets (pointers)
    pub asset_count: usize,
}

/**
 * API: - Represents The Asset Chunk
 */
#[repr(C)]
#[derive(Debug,Clone, Copy)]
pub struct RawCAssetChunk {
    pub data: *mut c_void,
    pub size: usize,
    pub manifest: *mut RawCAssetManifest,
}


#[no_mangle]
pub extern "C" fn load_asset_manifest(filepath:*const c_char) -> *mut RawCAssetManifest {
    let filestr:&str;
    unsafe {
        filestr = CStr::from_ptr(filepath).to_str().unwrap();
    }

    if filestr.is_empty() {
        println!("[-] Error: Invalid / Empty File Path");
        return std::ptr::null_mut();
    }

    let path = PathBuf::from_str(filestr).unwrap();
    let filedata = read(path);

    if filedata.is_err() {
        println!("[-] Error: Failed To Load Asset Manifest: {:?}", filedata.err());
        return std::ptr::null_mut();
    } else {
        let filedata = filedata.unwrap();

        let datastr = String::from_utf8(filedata);

        if datastr.is_err() {
            println!("[-] Error: Failed To Convert Asset Manifest Data To JSON String: {:?}", datastr.err());
            return std::ptr::null_mut();
        }

        let datastr =  datastr.unwrap();

        let manifest:AssetManifest = AssetManifest::from_json(&datastr);

        let mut raw_assets:Vec<RawCAsset> = Vec::new();

        

        
        for asset in manifest.get_assets().iter() {
            println!("Asset: {:?}", asset);
            let raw_asset = RawCAsset {
                data:std::ptr::null_mut(), // We're just loading the manifest, not the data, yet
                location: asset.get_chunk_location(),
                name: CString::new(asset.get_name()).unwrap().into_raw(),
                size: asset.get_size(),
            };
            raw_assets.push(raw_asset);
        }
        
        

        if raw_assets.is_empty() {
            println!("[-] Error: Failed To Load Asset Manifest");
            return std::ptr::null_mut();
        }

        let raw_assets_len = raw_assets.len();
        let mut raw_ptr_vec:Vec<*mut RawCAsset> = raw_assets.iter_mut().map(|asset| asset as *mut RawCAsset).collect();
        let raw_assets_ptr = raw_ptr_vec.as_mut_ptr();
        

        std::mem::forget(raw_assets);

        let return_manifest = RawCAssetManifest {
            assets: raw_assets_ptr,
            asset_count: raw_assets_len,
        };

        let return_manifest_ptr = Box::new(return_manifest);

        return Box::into_raw(return_manifest_ptr);

    }

}


#[no_mangle]
pub extern "C" fn load_asset_chunk(filepath_chunk:*const c_char, filepath_manifest:*const c_char) -> *mut RawCAssetChunk {
    let filestr = unsafe {
        CStr::from_ptr(filepath_chunk).to_str().unwrap()
    };

    if filestr.is_empty() {
        println!("[-] Error: Invalid / Empty Chunk File Path");
        return std::ptr::null_mut();
    }

    let path = PathBuf::from_str(filestr).unwrap();
    let chunk_data = read(path);

    if chunk_data.is_err() {
        println!("[-] Error: Failed To Load Asset Chunk: {:?}", chunk_data.err());
        return std::ptr::null_mut();
    }

    let chunk_data_block = chunk_data.unwrap();


    let chunk_data_len = chunk_data_block.len();

    let boxed_chunk_data = Box::new(chunk_data_block.as_slice());

    let manifest_str = unsafe {
        CStr::from_ptr(filepath_manifest).to_str().unwrap()
    };

    if manifest_str.is_empty() {
        println!("[-] Error: Invalid / Empty Manifest File Path");
        return std::ptr::null_mut();
    }

    let manifest_path = PathBuf::from_str(manifest_str).unwrap();
    let manifest_path_str = manifest_path.to_str().unwrap();
    let manifest_c_str = CString::new(manifest_path_str).unwrap().into_raw();
    let manifest_data = load_asset_manifest(manifest_c_str);

    let raw_chunk = RawCAssetChunk {
        data: boxed_chunk_data.as_ptr() as *mut c_void,
        size: chunk_data_len,
        manifest: manifest_data
    };

    return Box::into_raw(Box::new(raw_chunk));   

}


#[no_mangle]
pub extern "C" fn get_asset(asset_chunk:*const RawCAssetChunk, name:*const c_char) -> *mut RawCAsset {

    let rust_asset_chunk: Box<RawCAssetChunk> = unsafe { Box::from_raw(asset_chunk as *mut RawCAssetChunk) };
    
    let rust_asset_manifest: Box<RawCAssetManifest> = unsafe { Box::from_raw(rust_asset_chunk.manifest as *mut RawCAssetManifest) };


    // Okay lets get the vector of assets back from the struct C sent us.
    let asset_slice = unsafe {
        std::slice::from_raw_parts_mut(rust_asset_manifest.assets, rust_asset_manifest.asset_count)
    };
    let asset_vec:Vec<RawCAsset> = asset_slice.iter_mut().map(|&mut ptr| unsafe{*ptr}).collect();

    // Now we can get the asset we want
    let asset_name = unsafe {
        CStr::from_ptr(name).to_str().unwrap()
    };

    let asset = asset_vec.iter().find(|asset| {
        let asset_name_str = unsafe {
            CStr::from_ptr(asset.name).to_str().unwrap()
        };
        asset_name_str == asset_name
    });

    if asset.is_none() {
        println!("[-] Error: Asset Not Found: {}", asset_name);
        return std::ptr::null_mut();
    }

    println!("Asset Found: {:?}", asset.unwrap());

    // TODO: Do this better or something
    // Im cloning to shut the compiler up.
    let mut asset = asset.unwrap().clone();
    asset.data = unsafe {rust_asset_chunk.data.offset(asset.location as isize) as *mut c_void};
    println!("Asset Data Pointer: {:?}", asset.data);

    let raw_asset = Box::new(asset.clone());
    Box::into_raw(raw_asset)

}
