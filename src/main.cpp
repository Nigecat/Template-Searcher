#undef UNICODE
#include <iostream>
#include <windows.h>
#include <filesystem>
#include "../lib/INIReader.h"

namespace fs = std::filesystem;

int main() {
    // Assume our config file is in the same directory as the application
    char pathBuf[MAX_PATH];
    GetModuleFileName(NULL, pathBuf, sizeof(pathBuf));
    fs::path tempPath(pathBuf);

    INIReader config((tempPath.parent_path() / "config.ini").string());
    std::string shortcut = config.Get("config", "shortcut", "");
    fs::path path(config.Get("config", "path", ""));

    // Determine required modifiers for shortcut
    int modifiers = 0;
    if (shortcut.find("control") != std::string::npos) modifiers += MOD_CONTROL;
    if (shortcut.find("shift") != std::string::npos) modifiers += MOD_SHIFT;
    if (shortcut.find("alt") != std::string::npos) modifiers += MOD_ALT;
    if (shortcut.find("windows") != std::string::npos) modifiers += MOD_WIN;

    // Register a hotkey with the modifiers and the last character of the shortcut (after converting it to a virutal key code)
    RegisterHotKey(NULL, 1, modifiers, VkKeyScanExA(shortcut.back(), GetKeyboardLayout(0)));
    
    // Wait for out hotkey to be triggered
    MSG msg;
    while (GetMessage(&msg, NULL, 0, 0) != 0)
    {
        if (msg.message == WM_HOTKEY)
        {
            std::cout << "Starting search!" << std::endl;
        }
    }

    return 0;
}