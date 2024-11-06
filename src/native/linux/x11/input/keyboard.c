#include "keyboard.h"

struct Keyboard keyboard_context;

DLL_EXPORT void keyboard_init()
{
    keyboard_context.display = XOpenDisplay(NULL);
}

void keyboard_dispose()
{
    XCloseDisplay(keyboard_context.display);
    // display = NULL;
}

DLL_EXPORT bool keydown(int *scancodes, int len)
{
    unsigned int keycode = scancode_to_keycode(scancodes[0]);
    // unsigned int keycode = XKeysymToKeycode(keyboard_context.display, scancodes[0]);
    XTestFakeKeyEvent(keyboard_context.display, keycode, True, 0);
    XFlush(keyboard_context.display);
    return true;
}

DLL_EXPORT bool keyup(int *scancodes, int len)
{
    unsigned int keycode = scancode_to_keycode(scancodes[0]);
    // unsigned int keycode = XKeysymToKeycode(keyboard_context.display, scancodes[0]);
    XTestFakeKeyEvent(keyboard_context.display, keycode, False, 0);
    XFlush(keyboard_context.display);
    return true;
}
