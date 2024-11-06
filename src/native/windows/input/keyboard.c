#include "keyboard.h"

DLL_EXPORT void keyboard_init()
{
}

void keyboard_dispose()
{
}

int scancode_to_keycode(int scancode)
{
    // VkKeyScanEx(keystr.at(0), context.kbl);
    int virtualKeyCode = MapVirtualKey(scancode, MAPVK_VSC_TO_VK_EX);
    return virtualKeyCode;
}

DLL_EXPORT bool keydown(int *scancodes, int len)
{
    switch_to_input_desktop();
    INPUT input;
    int scancode = scancodes[0];
    int code = scancode_to_keycode(scancode);
    if (code > 0)
    {
        input.type = INPUT_KEYBOARD;
        input.ki.wVk = code;
        input.ki.dwFlags = 0 | KEYEVENTF_EXTENDEDKEY;
        input.ki.time = 0;
        input.ki.dwExtraInfo = 0;
        input.ki.wScan = 0;
        SendInput(1, &input, sizeof(INPUT));
    }
    return true;
}

DLL_EXPORT bool keyup(int *scancodes, int len)
{
    switch_to_input_desktop();
    INPUT input;
    int scancode = scancodes[0];
    int code = scancode_to_keycode(scancode);
    if (code > 0)
    {
        input.type = INPUT_KEYBOARD;
        input.ki.wVk = code;
        input.ki.dwFlags = KEYEVENTF_KEYUP | KEYEVENTF_EXTENDEDKEY;
        input.ki.time = 0;
        input.ki.dwExtraInfo = 0;
        input.ki.wScan = 0;
        SendInput(1, &input, sizeof(INPUT));
    }
    return true;
}
