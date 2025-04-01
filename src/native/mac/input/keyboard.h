#pragma once
#include "../export.h"
#include "./key.h"

DLL_EXPORT void keyboard_init();
DLL_EXPORT bool keydown(int *scancodes, int len);
DLL_EXPORT bool keyup(int *scancodes, int len);
void keyboard_dispose();
int scancode_to_keycode(int scancode);
