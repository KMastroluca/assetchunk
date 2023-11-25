
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

///  * API: - Represents A Single Asset Within The Chunk  *
///
typedef struct RawCAsset {
  void *data;
  uintptr_t location;
  uintptr_t size;
  const char *name;
} RawCAsset;

///  * API: - Represents The Asset Manifest
typedef struct RawCAssetManifest {
  RawCAsset *assets;
  uintptr_t asset_count;
} RawCAssetManifest;


typedef struct RawCAssetChunk {
  void *data;
  uintptr_t size;
  RawCAssetManifest *manifest;
} RawCAssetChunk;


extern "C" RawCAssetManifest* load_asset_manifest(const char *filepath);
extern "C" RawCAssetChunk* load_asset_chunk(const char *filepath_chunk, const char* filepath_manifest);
extern "C" RawCAsset* get_asset(RawCAssetChunk *asset_chunk, const char *name);


