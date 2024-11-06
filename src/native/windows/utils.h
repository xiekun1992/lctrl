#pragma once
#include "export.h"

#include <locale.h>
#include <Windows.h>
#include <shellapi.h>
#include <tchar.h>

DLL_EXPORT void utils_open(wchar_t *path);
DLL_EXPORT RECT get_screen_size();
DLL_EXPORT BOOL is_run_as_admin();
DLL_EXPORT VOID run_as_admin();
