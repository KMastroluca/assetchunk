
#include <stdio.h>
#include <stdlib.h>
#include "lib.h"
#include "raylib/include/raylib.h"
int main()  
{
   printf("[+] Loading In Asset Manifest\n");
   struct RawCAssetManifest* manifest = load_asset_manifest("out.manifest.asset");
   printf("Asset count: %zu\n", manifest->asset_count);

    printf("[+] Loading In Asset Chunk\n");
    struct RawCAssetChunk* chunk = load_asset_chunk("out.chunk.asset", "out.manifest.asset");
    
    // Lets get the Tileset_Terrain.png asset
    RawCAsset* tileset_terrain1 = get_asset(chunk , "Tileset_Terrain1.png");
    printf("Tileset_Terrain1.png: %zu\n", tileset_terrain1->size);
    printf("Name: %s\n", tileset_terrain1->name);
    printf("Loc: %zu\n", tileset_terrain1->location);

    Texture2D tex = LoadTextureFromImage(LoadImageFromMemory("png", (unsigned char*)tileset_terrain1->data, tileset_terrain1->size));
    printf("Texture: %s\n", (unsigned char*)tileset_terrain1->data);
    const int screenWidth = 800;
    const int screenHeight = 600;

    InitWindow(screenWidth, screenHeight, "TestAssetChunk");

    while(!WindowShouldClose())
    {
        BeginDrawing();
        ClearBackground(RAYWHITE);
        DrawText("Test Asset Chunk", 10, 10, 20, LIGHTGRAY);
        EndDrawing();
    }

    CloseWindow();


    return 0;
}