#pragma once
#include "../export.h"
#include "helper.h"

#include <stdio.h>
#include <stdbool.h>
// #define WINVER 0x0500 // SendInput needs this
#include <windows.h>

DLL_EXPORT void keyboard_init();
DLL_EXPORT bool keydown(int *scancodes, int len);
DLL_EXPORT bool keyup(int *scancodes, int len);
void keyboard_dispose();
int scancode_to_keycode(int scancode);
