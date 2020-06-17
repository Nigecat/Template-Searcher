#Include gdip.ahk
#SingleInstance, force
Menu, Tray, Icon, icon.ico
SetTitleMatchMode, 2

IniRead, shortcut, config.ini, Config, shortcut
Hotkey, %shortcut%, start
return

start:
    log("-----Beginning search-----")
    log("Saving active window id")
    WinGet, winid, , A

    log("Creating keyboard hook")
    ih := InputHook("M", "{ENTER}{Esc}")
    ih.KeyOpt("{All}", "N")
    ih.OnKeyDown := Func("displayImage")
    ih.Start()
    ih.Wait()
    Gui, Destroy
    if (not ih.EndKey == "Escape")
    {
        sendImage(findMatch(ih.Input), winid)
    }
return

displayImage(ih)
{
    Log("Displaying image")
    IniRead, preview, config.ini, Config, preview

    if (preview == "true") 
    {
        path := findMatch(ih.Input)

        ; get the dimensions of the matched image
        pToken := Gdip_StartUp()
        Gdip_GetImageDimensions(pBitmap := Gdip_CreateBitmapFromFile(path), w, h)
        Gdip_DisposeImage(pBitmap)
        Gdip_ShutDown(pToken)

        ; display the image on the user's screen
        Gui, Destroy
        Gui, +LastFound +AlwaysOnTop -Caption
        Gui, Add, Picture, x0 y0 w%w% h%h%, %path%
        WinSet, Top
        WinSet, ExStyle, ^0x20
        WinSet, Transparent, 150
        Gui, Show, w%w% h%h%
    }
}

findMatch(searchStr)
{
    Log("--Finding image match--")
    IniRead, path, config.ini, Config, path

    ; detect whether the search str ends in a number and if not default to a match level of 1
    if (RegexMatch(SubStr(searchStr, 0, 1), "[^0-9]")) 
    {
        matchLevel := 1
    } 
    else 
    {
        matchLevel := SubStr(searchStr, 0, 1) 
        StringTrimRight, searchStr, searchStr, 1    ; remove the extra character
    }
    log("Setting match level to " matchLevel)

    searchStr := cleanse(searchStr)
    words := StrSplit(searchStr, " ")
    match := ""

    log("Reading files")
    ; load the files into an array
    files := []
    Loop, Files, %path%\*.*, R 
        files[A_Index] := [A_LoopFileName, A_LoopFileFullPath]

    log("Checking beginning")
    ; check if a file starts with the search string
    match := matchLevel
    For i, file In files
    {
        if (SubStr(cleanse(file[1]), 1, StrLen(searchStr)) == searchStr)
        {
            match -= 1
            if (match == 0)
            {
                return file[2]
            }
        } 
    }

    log("Checking contains")
    ; check if a file contains the search string
    match := matchLevel
    For i, file In files
    {
        if (InStr(cleanse(file[1]), searchStr))
        {
            match -= 1
            if (match == 0)
            {
                return file[2]
            }
        }
    }

    log("Checking contains words")
    ; if a file contains all the words in the search str
    match := matchLevel
    For i, file In files
    {
        valid := true
        for index, search in words
        {
            if (not InStr(file[1], search))
            {
                valid := false
            }
        } 

        if (valid)
        {
            match -= 1
            if (match == 0)
            {
                return file[2]
            }
        }
    } 
    
}

sendImage(img, winid)
{
    log("Sending image")
    IniRead, activate, config.ini, Config, activate
    IniRead, window, config.ini, Config, window
    IniRead, restore, config.ini, Config, restore

    ; temporarily save clipboard contents
    clip := ClipboardAll

    if (activate == "true") 
    {
        WinActivate, %window%
        WinWaitActive, %window%, , 1
    }

    ; if winwaitactive does not time out (after 1 second)
    if (!ErrorLevel) 
    {
        ; copy image to clipboard
        copyToClip(img)
        
        ; paste image and send
        Send, ^v{ENTER}  
        Sleep, 100

        ; restore the original clipboard
        Clipboard := clip

        ; free the memory in case the clipboard was very large.
        clip := ""

        if (restore == "true")
        {
            WinActivate ahk_id %winid%
        }
    }
}

copyToClip(path)
{
    log("Copying image to clipboard")
    pToken := Gdip_Startup()
    Gdip_SetBitmapToClipboard(pBitmap := Gdip_CreateBitmapFromFile(path))
    Gdip_DisposeImage(pBitmap)
    Gdip_Shutdown(pToken)
}

cleanse(str)    ; remove all non-alphanumeric characters from input string while preserving spaces
{
    StringLower, str, str
    return RegExReplace(str, "[^\w\s]", "")
}

log(data)
{
    IniRead, debug, config.ini, Config, debug
    if (debug == "true") 
    {
        FileAppend, %data%`n, log.txt
    }
}