#pragma once
#include "export.h"

typedef struct
{
    int left;
    int top;
    int right;
    int bottom;
} RECT;

DLL_EXPORT RECT get_screen_size();
DLL_EXPORT bool is_run_as_admin();
DLL_EXPORT void run_as_admin();
DLL_EXPORT RECT *get_screens(int *count);