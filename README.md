# JRPG Game Engine: Asset Packaging and Loading Module 🎮📦

## Description 📝

This module is part of a larger JRPG Game Engine project. It is designed to optimize your game's performance by enabling you to package your game assets into a contiguous block of memory, which can then be loaded into the engine. This process reduces the amount of memory fragmentation, speeding up the game's runtime. 🚀

The assets that can be packaged include images, audio files, fonts, shaders, models, scripts, and other custom types. Each asset is represented by an `Asset` struct, which includes its name, type, location within the chunk, size in bytes, and the actual data as a vector of bytes. 🏞🎵🖼🔤

## Usage 🛠

### Loading an Asset From a File

To load an asset from a file, use the `load` function, providing the file path, asset name, and asset type. This function reads the file data into a byte vector and creates an `Asset` instance.

```rust
rust let asset = Asset::load("path/to/asset", "asset_name", AssetType::Image);
```


### Asset Manipulation

The `Asset` struct provides several methods for manipulation:

- `set_chunk_location(&mut self, location:usize)`: Sets the location of the asset in the chunk.
- `get_chunk_location(&self) -> usize`: Gets the location of the asset in the chunk.
- `get_size(&self) -> usize`: Gets the size of the asset in bytes.
- `get_data(&self) -> &Option<Vec<u8>>`: Gets the data of the asset as a byte vector.
- `get_name(&self) -> &str`: Gets the name of the asset.
- `get_type(&self) -> &AssetType`: Gets the type of the asset.

### Asset Manifest

The `AssetManifest` struct represents a collection of assets. It provides methods for creating a new asset manifest, retrieving assets, modifying assets, serializing the manifest to a JSON string, loading a manifest from a JSON string, and writing the manifest to a file.

### API Structures

The `RawCAsset`, `RawCAssetManifest`, and `RawCAssetChunk` structs represent the C-compatible API to interact with the assets and their manifests.

## Contributing 💡

This project is a work in progress, and contributions are welcome. Please feel free to open an issue or submit a pull request.

## License 📄

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## Acknowledgments 👏

- Thanks to the Rust community for their valuable resources and support.
