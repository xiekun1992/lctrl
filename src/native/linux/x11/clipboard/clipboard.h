#pragma once
#include "../export.h"

#include <stdbool.h>
#include <X11/extensions/Xfixes.h>

struct Clipboard
{
  bool notify;
  wchar_t *content;
  void (*fn)(void);
};
DLL_EXPORT void clipboard_init();
DLL_EXPORT void clipboard_dispose();
DLL_EXPORT bool write_text(wchar_t *text);
DLL_EXPORT wchar_t *read_text();
// bool write_files(std::vector<std::u16string> files);
// std::vector<std::u16string> read_files();
DLL_EXPORT void capture(void (*fn)(void));
DLL_EXPORT void release();