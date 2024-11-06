#pragma once
#include "../export.h"
#include "./key.h"

#include <stdio.h>
#include <stdbool.h>
#include <string.h>
#include <stdint.h>

#include <linux/uinput.h>
#include <linux/input.h>
#include <fcntl.h>

typedef struct Input
{
  int fd;
} INPUT;

enum MouseButton
{
  MOUSE_LEFT = 1,
  MOUSE_MIDDLE,
  MOUSE_RIGHT
};

enum MouseWheel
{
  WHEEL_UP = -1,
  WHEEL_DOWN = 1
};

DLL_EXPORT void keyboard_init();
DLL_EXPORT bool keydown(int *scancodes, int len);
DLL_EXPORT bool keyup(int *scancodes, int len);
// void keyboard_dispose();
// int scancode_to_keycode(int scancode);

DLL_EXPORT void mouse_init(int left, int top, int right, int bottom);
DLL_EXPORT void mouse_dispose();
DLL_EXPORT void mouse_move(int x, int y);
DLL_EXPORT void mouse_wheel(enum MouseWheel direction);
DLL_EXPORT void mouse_down(enum MouseButton button);
DLL_EXPORT void mouse_up(enum MouseButton button);