#NoEnv
#MaxThreads, 2
#Include gdip.ahk
#SingleInstance, force
SetTitleMatchMode, 2

; Read the config file
IniRead, shortcut, config.ini, Config, shortcut
IniRead, path, config.ini, Config, path
IniRead, transparency, config.ini, Config, transparency

; Setup the tray 
Menu, Tray, NoStandard 
Menu, Tray, Add, Restart
Menu, Tray, Add, Exit

; Create the main hotkey
Hotkey, %shortcut%, start

; Create a gdip instance used for image processing
pToken := Gdip_StartUp()

; Create a variable to store a temporary cache of the files in the search dir
files := []

; Function to restart the script, bound to the 'Restart' tray button
Restart()
{
    Reload
}

; Function to exit the script, bound to the 'Exit' tray button
Exit()
{
    ExitApp
}

; Called to start a search
start()
{
    global path
    global files
    ; Load the files into an array
    files := []
    Loop, Files, %path%\*.*, R 
    {
        files[A_Index] := [A_LoopFileName, A_LoopFileFullPath]
    }

    ; Save the active window so we can restore it later
    WinGet, winid, , A

    ; Create a keyboard hook that calls the displayImage function each time a key is pressed
    ih := InputHook("M", "{ENTER}{Esc}")
    ih.KeyOpt("{All}", "N")
    ih.OnKeyDown := Func("displayImage")
    ih.Start()
    
    ; Wait for one of the termination keys to be pressed (enter or escape)
    ih.Wait()

    ; Destroy the gui window
    Gui, Destroy

    ; If the search was not cancelled
    if (not ih.EndKey == "Escape")
    {
        sendImage(findMatch(ih.Input), winid)
    }
}

displayImage(ih)
{
    global transparency

    path := findMatch(ih.Input)

    ; Get the dimensions of the matched image
    Gdip_GetImageDimensions(pBitmap := Gdip_CreateBitmapFromFile(path), w, h)
    Gdip_DisposeImage(pBitmap)

    ; Display the image on the user's screen
    Gui, Destroy
    Gui, +LastFound +AlwaysOnTop -Caption
    Gui, Add, Picture, x0 y0 w%w% h%h%, %path%
    WinSet, Top
    WinSet, ExStyle, ^0x20
    WinSet, Transparent, %transparency%
    Gui, Show, w%w% h%h%
}

; Will return the closest match to a string from the filenames in %path%
findMatch(searchStr)
{
    global files

    ; Extract any digits at the end of the search string to serve as the match level
    RegExMatch(searchStr, "\d+$", matchLevel)
    ; If the string does not end in digits then default the match level to 1
    if !matchLevel
    {
        matchLevel := 1
    }
    ; If digits were found then remove them from the original search string
    else
    {
        searchStr := RegExReplace(searchStr, "\d+$", "")
    }

    searchStr := cleanse(searchStr)
    words := StrSplit(searchStr, " ")

    ; Check if a file starts with the search string
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

    ; Check if a file contains the search string
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

    ; If a file contains all the words in the search str
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
    ; Temporarily save clipboard contents
    clip := ClipboardAll

    ; Make discord the active window
    WinActivate, Discord

    ; Copy the image to the clipboard
    Gdip_SetBitmapToClipboard(pBitmap := Gdip_CreateBitmapFromFile(img))
    Gdip_DisposeImage(pBitmap)
        
    Sleep, 25

    ; Paste image and send
    Send, ^v{ENTER}
    Sleep, 100

    ; Restore the original clipboard
    Clipboard := clip

    ; Free the memory in case the clipboard was very large.
    clip := ""

    ; Activate the original window
    WinActivate ahk_id %winid%
}

; Remove all non-alphanumeric characters from input string while preserving spaces
cleanse(str)    
{
    StringLower, str, str
    return RegExReplace(str, "[^\w\s]", "")
}