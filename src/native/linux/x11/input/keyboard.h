#pragma once
#include "../export.h"
#include "keycode.h"

#include <stdio.h>
#include <stdbool.h>
#include <X11/Xlib.h>
#include <X11/keysym.h>
#include <X11/extensions/XTest.h>

struct Keyboard
{
    Display *display;
};

DLL_EXPORT void keyboard_init();
DLL_EXPORT bool keydown(int *scancodes, int len);
DLL_EXPORT bool keyup(int *scancodes, int len);
void keyboard_dispose();
int scancode_to_keycode(int scancode);
