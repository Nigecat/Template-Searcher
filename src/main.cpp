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


    return 0;
}