
#include <stdio.h>
#include <stdlib.h>
#include "lib.h"

int main()
{
   printf("[+] Loading In Asset Manifest\n");
   struct RawCAssetManifest* manifest = load_asset_manifest("out.manifest.asset");
   printf("Asset count: %zu\n", manifest->asset_count);

    return 0;
}