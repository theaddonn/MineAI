#pragma once
#include "rust.hpp"
#include <Windows.h>
#include <amethyst/runtime/AmethystContext.hpp>
#include <string>

#define ModFunction extern "C" __declspec(dllexport)

BOOL APIENTRY DllMain(HMODULE hModule, DWORD ul_reason_for_call, LPVOID lpReserved) {
    return TRUE;
}

void OnStartJoinGame(ClientInstance* client);

void MineAIThread();

extern "C" {
    MineAIInitResult mine_ai_init(MineAIInitOptions options);
    MineAIGenerationResult mine_ai_generate(MineAIObject object, std::string prompt);
}
