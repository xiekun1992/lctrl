#pragma once
#include "../export.h"
#include <stdbool.h>
#include <windows.h>
#include <tlhelp32.h>

#define UNICODE
#define SERVICE_NAME L"Lctrl"

DLL_EXPORT int register_service();
DLL_EXPORT VOID delete_service();
DLL_EXPORT VOID create_service();
DLL_EXPORT VOID stop_service();
DLL_EXPORT VOID start_service();
