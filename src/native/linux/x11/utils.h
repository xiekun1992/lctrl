#pragma once
#include "export.h"

#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <X11/Xlib.h>
#include <X11/extensions/Xinerama.h>

typedef struct Rect
{
  int left;
  int top;
  int right;
  int bottom;
} RECT;

DLL_EXPORT RECT get_screen_size();
DLL_EXPORT bool is_run_as_admin();
DLL_EXPORT void run_as_admin();
