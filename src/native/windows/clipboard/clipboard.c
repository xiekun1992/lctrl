#include "clipboard.h"

struct Clipboard clipboard_context;

DLL_EXPORT void clipboard_init()
{
    clipboard_context.content = (wchar_t *)malloc(sizeof(wchar_t) * 1024 * 1024);
    clipboard_context.notify = false;
}

DLL_EXPORT void clipboard_dispose()
{
    free(clipboard_context.content);
}

LRESULT CALLBACK WndProc(HWND hWnd, UINT message, WPARAM wParam, LPARAM lParam)
{
    switch (message)
    {
    case WM_CLIPBOARDUPDATE:
        // handle change
        clipboard_context.fn();
        break;
    // case WM_PAINT:
    // {
    // 	PAINTSTRUCT ps;
    // 	HDC hdc = BeginPaint(hWnd, &ps);
    // 	EndPaint(hWnd, &ps);
    // }
    // break;
    case WM_DESTROY:
        PostQuitMessage(0);
        RemoveClipboardFormatListener(hWnd);
        break;
    default:
        return DefWindowProc(hWnd, message, wParam, lParam);
    }
    return 0;
}

DLL_EXPORT bool write_text(wchar_t *text)
{
    bool res = false;
    OpenClipboard(0);
    HGLOBAL clipbuffer = GlobalAlloc(GMEM_MOVEABLE, (wcslen(text) + 1) * sizeof(wchar_t));
    if (clipbuffer != NULL)
    {
        wchar_t *buffer = (wchar_t *)GlobalLock(clipbuffer);
        if (buffer != NULL)
        {
            EmptyClipboard();
            wcscpy(buffer, text);
            SetClipboardData(CF_UNICODETEXT, clipbuffer);

            GlobalUnlock(clipbuffer);
            res = true;
        }
    }
    CloseClipboard();
    return res;
}

DLL_EXPORT wchar_t *read_text()
{
    wchar_t *empty = L"";
    if (!OpenClipboard(0))
    {
        return empty;
    }

    if (!IsClipboardFormatAvailable(CF_UNICODETEXT))
    {
        CloseClipboard();
        return empty;
    }
    HANDLE handle = (HANDLE)GetClipboardData(CF_UNICODETEXT);
    if (handle != NULL)
    {
        wchar_t *buffer = GlobalLock(handle);
        if (buffer != NULL)
        {
            wcscpy(clipboard_context.content, buffer);
            GlobalUnlock(handle);
            CloseClipboard();
            return clipboard_context.content;
        }
    }
    CloseClipboard();
    return empty;
}

// bool write_files(std::vector<std::u16string> files) {
// #if _WIN32 == 1
//   OpenClipboard(0);

//   std::u16string sFiles; // 对文件名以双字节存放
//   for (std::u16string file : files) {
//     sFiles += file;
//     sFiles.push_back(0); // 追加'\0'
//   }

//   DROPFILES dobj = { 20, { 0, 0 }, 0, 1 };
//   int nLen = (sFiles.size() + 1) * sizeof(char16_t);
//   int nGblLen = sizeof(dobj) + nLen;

//   HGLOBAL hGbl = GlobalAlloc(GMEM_ZEROINIT | GMEM_MOVEABLE | GMEM_DDESHARE, nGblLen);
//   if (hGbl != NULL) {
//     char* buffer = (char*)GlobalLock(hGbl);
//     if (buffer != NULL) {
//       memcpy(buffer, &dobj, sizeof(dobj));
//       memcpy(buffer + sizeof(dobj), sFiles.c_str(), nLen);
//       EmptyClipboard();
//       SetClipboardData(CF_HDROP, hGbl);
//       GlobalUnlock(hGbl);

//       CloseClipboard();
//       return true;
//     }
//   }
//   CloseClipboard();
// #endif
//   return false;
// }

// std::vector<std::u16string> read_files() {
//   std::vector<std::basic_string<char16_t>> filenames;
// #if _WIN32 == 1
//   OpenClipboard(0);

//   if (!IsClipboardFormatAvailable(CF_HDROP)) return filenames;

//   HGLOBAL hClipboardText = (HGLOBAL)GetClipboardData(CF_HDROP);
//   if (hClipboardText != NULL) {
//     HDROP  h = (HDROP)GlobalLock(hClipboardText);
//     if (h != NULL) {
//       int fileCount = DragQueryFile(h, 0xFFFFFFFF, nullptr, 0);
//       for (int i = 0; i < fileCount; i++) {
//         std::basic_string<char16_t> str;
//         filenames.push_back(str);
//         int filenameLength = DragQueryFile(h, i, nullptr, 0);
//         filenames[i].reserve(filenameLength);
//         DragQueryFile(h, i, (LPWSTR)&(filenames[i][0]), filenameLength + 1);
//       }
//       GlobalUnlock(hClipboardText);

//       CloseClipboard();
//       return filenames;
//     }
//   }
//   CloseClipboard();
// #endif
//   return filenames;
// }

DLL_EXPORT void capture(void (*fn)(void))
{
    clipboard_context.fn = fn;

    const wchar_t CLASS_NAME[] = L"Clipboard Window Class";

    HINSTANCE hInstance = GetModuleHandle(0);
    WNDCLASS wc = {0};
    wc.lpfnWndProc = WndProc;
    wc.hInstance = hInstance;
    wc.lpszClassName = CLASS_NAME;

    RegisterClass(&wc);
    HWND hwnd = CreateWindowEx(
        0,                             // Optional window styles.
        CLASS_NAME,                    // Window class
        L"Clipboard Listener Windows", // Window text
        WS_OVERLAPPEDWINDOW,           // Window style
        // Size and position
        CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT,
        NULL,      // Parent window
        NULL,      // Menu
        hInstance, // Instance handle
        NULL       // Additional application data
    );
    if (hwnd != NULL)
    {
        AddClipboardFormatListener(hwnd);
        MSG msg;
        while (GetMessage(&msg, NULL, 0, 0))
        {
            TranslateMessage(&msg);
            DispatchMessage(&msg);
        }
    }
}

DLL_EXPORT void release()
{
}
