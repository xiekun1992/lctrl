#include "keyboard.h"

DLL_EXPORT void keyboard_init()
{
}

void keyboard_dispose()
{
}

int scancode_to_keycode(int scancode)
{
    int keycode = scancode_to_key(scancode);
    printf("scancode=%d, keycode=%d\n", scancode, keycode);
    return keycode;
}

DLL_EXPORT bool keydown(int *scancodes, int len)
{
    CGKeyCode keyCode = scancode_to_keycode(scancodes[0]);
    if (keyCode == -1)
    {
        return false;
    }
    CGEventRef keyDownEvent = CGEventCreateKeyboardEvent(NULL, keyCode, true);
    CGEventPost(kCGEventSourceStateHIDSystemState, keyDownEvent);
    CFRelease(keyDownEvent);
    return true;
}

DLL_EXPORT bool keyup(int *scancodes, int len)
{
    CGKeyCode keyCode = scancode_to_keycode(scancodes[0]);
    if (keyCode == -1)
    {
        return false;
    }
    CGEventRef keyUpEvent = CGEventCreateKeyboardEvent(NULL, keyCode, false);
    CGEventPost(kCGEventSourceStateHIDSystemState, keyUpEvent);
    CFRelease(keyUpEvent);
    return true;
}
