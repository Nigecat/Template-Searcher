#NoEnv
#MaxThreads, 2
#SingleInstance, force

; Usage:
; NOTE: The bulk of the processing is expected to be done by the caller.
;
; Arguments:
;  1 - The type of display, this can either be 'image' or 'text'
;  Image:
;    2 - The width of the image
;    3 - The height of the image
;    4 - The path to the image
;  Text:
;    2 - The text string

; Create the gui window
Gui, +LastFound +AlwaysOnTop -Caption

; If this is a text file display the text in the gui
if A_Args[1] = "text" 
{
    Gui, Add, text, Content, % A_Args[2]
    Gui, Show, AutoSize
}
; Otherwise assume it is an image
else
{
    w := % A_Args[2]
    h := % A_Args[3]
    path := % A_Args[4]

    Gui, Add, Picture, x0 y0 w%w% h%h%, %path%
    WinSet, Top
    WinSet, ExStyle, ^0x20
    WinSet, Transparent, 150
    Gui, Show, w%w% h%h%
}