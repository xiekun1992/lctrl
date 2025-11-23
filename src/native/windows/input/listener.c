#include "listener.h"

struct Listener context;

int resolve_ex_scancode(PKBDLLHOOKSTRUCT hookStruct)
{
    if (hookStruct->flags & 0x01 == 1)
    {
        return 0xe000 + hookStruct->scanCode;
    }
    return hookStruct->scanCode;
}

int keycode_to_scancode(int keycode)
{
    int scancode = MapVirtualKey(keycode, MAPVK_VK_TO_VSC_EX);
    return scancode;
}

LRESULT CALLBACK keyboardHookProc(int nCode, WPARAM wParam, LPARAM lParam)
{
    PKBDLLHOOKSTRUCT hookStruct = (PKBDLLHOOKSTRUCT)lParam;
    switch (wParam)
    {
    case WM_KEYDOWN:
    case WM_SYSKEYDOWN:
    {
        long params[7] = {L_KEYDOWN, (long)hookStruct->vkCode, (long)resolve_ex_scancode(hookStruct)};
        if (hookStruct->vkCode == VK_LCONTROL)
        {
            context.is_lcontrol_down = true;
        }
        if (hookStruct->vkCode == VK_LSHIFT)
        {
            context.is_lshift_down = true;
        }
        if (hookStruct->vkCode == VK_LWIN)
        {
            context.is_lwin_down = true;
        }
        if (hookStruct->vkCode == VK_LMENU)
        {
            context.is_lalt_down = true;
        }
        if (hookStruct->vkCode == VK_ESCAPE)
        {
            context.is_escape_down = true;
        }
        if (
            // context.is_lcontrol_down &&
            // context.is_lshift_down &&
            context.is_lwin_down &&
            context.is_lalt_down &&
            // context.is_escape_down
        )
        {
            long hotkeys[5][7] = {
                // {L_KEYUP, (long)VK_LCONTROL, (long)keycode_to_scancode(VK_LCONTROL), 0, 0, 0, 0},
                // {L_KEYUP, (long)VK_LSHIFT, (long)keycode_to_scancode(VK_LSHIFT), 0, 0, 0, 0},
                {L_KEYUP, (long)VK_LWIN, (long)keycode_to_scancode(VK_LWIN), 0, 0, 0, 0},
                {L_KEYUP, (long)VK_LMENU, (long)keycode_to_scancode(VK_LMENU), 0, 0, 0, 0},
                // {L_KEYUP, (long)VK_ESCAPE, (long)keycode_to_scancode(VK_ESCAPE), 0, 0, 0, 0},
            };
            context.hotkeyHandler(hotkeys);
        }
        context.keyboardHanlder(params);
        break;
    }
    case WM_KEYUP:
    case WM_SYSKEYUP:
    {
        long params[7] = {L_KEYUP, (long)hookStruct->vkCode, (long)resolve_ex_scancode(hookStruct)};
        if (hookStruct->vkCode == VK_LCONTROL)
        {
            context.is_lcontrol_down = false;
        }
        if (hookStruct->vkCode == VK_LSHIFT)
        {
            context.is_lshift_down = false;
        }
        if (hookStruct->vkCode == VK_LWIN)
        {
            context.is_lwin_down = false;
        }
        if (hookStruct->vkCode == VK_LMENU)
        {
            context.is_lalt_down = false;
        }
        if (hookStruct->vkCode == VK_ESCAPE)
        {
            context.is_escape_down = false;
        }
        context.keyboardHanlder(params);
        break;
    }
    }
    // if (hookStruct->flags & 0x01 == 1)
    // {
    //     // expand key
    //     printf("nCode=%ld, wParam=%ld, lParam=%ld, flags=%ld, vkcode=%ld, scancode=%ld\n", nCode, wParam, lParam, hookStruct->flags, hookStruct->vkCode, resolve_ex_scancode(hookStruct));
    // }
    // else
    // {
    //     printf("nCode=%ld, wParam=%ld, lParam=%ld, flags=%ld, vkcode=%ld, scancode=%ld\n", nCode, wParam, lParam, hookStruct->flags, hookStruct->vkCode, resolve_ex_scancode(hookStruct));
    // }
    if (context.blocking)
    {
        // Not thses keys: caps lock / scroll lock
        // if (hookStruct->vkCode != VK_CAPITAL && hookStruct->vkCode != VK_SCROLL)
        {
            return 1; // disable key
        }
    }
    return CallNextHookEx(NULL, nCode, wParam, lParam);
}

LRESULT CALLBACK mouseHookProc(int nCode, WPARAM wParam, LPARAM lParam)
{
    // Get event information
    PMSLLHOOKSTRUCT p = (PMSLLHOOKSTRUCT)lParam;
    long mouseData = p->mouseData;
    switch (wParam)
    {
    case WM_MOUSEWHEEL:
    {
        long params[5] = {L_MOUSEWHEEL, (long)p->pt.x, (long)p->pt.y, 0, (long)(mouseData >> 16)};
        context.mouseHanlder(params);
        break;
    }
    case WM_MOUSEMOVE:
    {
        long params[5] = {L_MOUSEMOVE, (long)p->pt.x, (long)p->pt.y, 0, (long)(mouseData >> 16)};
        context.mouseHanlder(params);
        break;
    }
    case WM_LBUTTONDOWN:
    {
        long params[5] = {L_MOUSEDOWN, (long)p->pt.x, (long)p->pt.y, L_MOUSE_BUTTON_LEFT, (long)(mouseData >> 16)};
        context.mouseHanlder(params);
        break;
    }
    case WM_LBUTTONUP:
    {
        long params[5] = {L_MOUSEUP, (long)p->pt.x, (long)p->pt.y, L_MOUSE_BUTTON_LEFT, (long)(mouseData >> 16)};
        context.mouseHanlder(params);
        break;
    }
    case WM_RBUTTONDOWN:
    {
        long params[5] = {L_MOUSEDOWN, (long)p->pt.x, (long)p->pt.y, L_MOUSE_BUTTON_RIGHT, (long)(mouseData >> 16)};
        context.mouseHanlder(params);
        break;
    }
    case WM_RBUTTONUP:
    {
        long params[5] = {L_MOUSEUP, (long)p->pt.x, (long)p->pt.y, L_MOUSE_BUTTON_RIGHT, (long)(mouseData >> 16)};
        context.mouseHanlder(params);
        break;
    }
    case WM_MBUTTONDOWN:
    {
        long params[5] = {L_MOUSEDOWN, (long)p->pt.x, (long)p->pt.y, L_MOUSE_BUTTON_MIDLLE, (long)(mouseData >> 16)};
        context.mouseHanlder(params);
        break;
    }
    case WM_MBUTTONUP:
    {
        long params[5] = {L_MOUSEUP, (long)p->pt.x, (long)p->pt.y, L_MOUSE_BUTTON_MIDLLE, (long)(mouseData >> 16)};
        context.mouseHanlder(params);
        break;
    }
    }
    if (context.blocking)
    {
        return 1; // disable key
    }
    return CallNextHookEx(NULL, nCode, wParam, lParam);
}

LRESULT CALLBACK deviceHookProc(HWND hwnd, UINT msg, WPARAM wParam, LPARAM lParam)
{
    // switch (msg)
    // {
    // case WM_CREATE:
    //     if (WTSRegisterSessionNotification(hwnd, NOTIFY_FOR_ALL_SESSIONS))
    //     {
    //         printf("Session notifications registered successfully.");
    //     }
    //     else
    //     {
    //         printf("Failed to register session notifications. Error: ");
    //     }
    //     break;
    // case WM_WTSSESSION_CHANGE:
    //     switch (wParam)
    //     {
    //     case WTS_SESSION_LOCK:
    //         printf("Session locked.");
    //         break;
    //     case WTS_SESSION_UNLOCK:
    //         printf("Session unlocked.");
    //         break;
    //     case WTS_SESSION_LOGON:
    //         printf("Session logon.");
    //         break;
    //     case WTS_SESSION_LOGOFF:
    //         printf("Session logoff.");
    //         break;
    //     }
    //     break;
    // case WM_DESTROY:
    //     WTSUnRegisterSessionNotification(hwnd);
    //     PostQuitMessage(0);
    //     break;
    // }
    if (msg == WM_INPUT)
    {
        HRAWINPUT hRawInput = (HRAWINPUT)lParam;
        RAWINPUT input = {0};
        UINT size = sizeof(input);
        GetRawInputData(hRawInput, RID_INPUT, &input, &size, sizeof(RAWINPUTHEADER));
        // printf("mouse rel move = %ld, %ld\n", input.data.mouse.lLastX, input.data.mouse.lLastY);
        if (MOUSE_MOVE_RELATIVE == input.data.mouse.usFlags)
        {
            // printf("mouse rel %d, move = %ld, %ld\n", input.data.mouse.usFlags, input.data.mouse.lLastX, input.data.mouse.lLastY);
            if (context.blocking)
            {
                long params[5] = {L_MOUSEMOVEREL, input.data.mouse.lLastX, input.data.mouse.lLastY, 0, 0};
                context.mouseHanlder(params);
            }
        }
    }
    return DefWindowProc(hwnd, msg, wParam, lParam);
}

DLL_EXPORT void listener_init(
    void (*mouseHanlder)(long *),
    void (*keyboardHanlder)(long *),
    void (*hotkeyHandler)(long[5][7]))
{
    context.mouseHanlder = mouseHanlder;
    context.keyboardHanlder = keyboardHanlder;
    context.hotkeyHandler = hotkeyHandler;
    context.is_lcontrol_down = false;
    context.is_lshift_down = false;
    context.is_lwin_down = false;
    context.is_lalt_down = false;
    context.is_escape_down = false;
    context.blocking = false;

    context.mouseHook = SetWindowsHookEx(
        WH_MOUSE_LL,   /* Type of hook */
        mouseHookProc, /* Hook process */
        NULL,          /* Instance */
        0);
    context.keyboardHook = SetWindowsHookEx(
        WH_KEYBOARD_LL,
        keyboardHookProc,
        NULL,
        0);

    WNDCLASSEX wcx = {0};
    wcx.cbSize = sizeof(WNDCLASSEX);
    wcx.lpfnWndProc = deviceHookProc;
    wcx.hInstance = GetModuleHandle(NULL);
    wcx.lpszClassName = TEXT("RawInputClass");
    RegisterClassEx(&wcx);
    HWND hwnd = CreateWindowEx(0, TEXT("RawInputClass"), NULL, 0, 0, 0, 0, 0, HWND_MESSAGE, NULL, GetModuleHandle(NULL), NULL);

    RAWINPUTDEVICE rid = {0};
    rid.usUsagePage = 0x01;
    rid.usUsage = 0x02; // mouse
    // rid.usUsage = 0x06; // keybaord
    rid.dwFlags = RIDEV_INPUTSINK;
    rid.hwndTarget = hwnd;

    RegisterRawInputDevices(&rid, 1, sizeof(RAWINPUTDEVICE));
}

DLL_EXPORT void listener_dispose()
{
}

DLL_EXPORT void listener_listen()
{
    // https://stackoverflow.com/questions/4509521/does-getmessage-need-a-gui
    // MSG msg;
    // PeekMessage(&msg, NULL, WM_USER, WM_USER, PM_NOREMOVE);
    // http://www.winprog.org/tutorial/message_loop.html
    // message loop check, without this, hooks won't work
    MSG Msg;
    while (GetMessage(&Msg, NULL, 0, 0) > 0)
    {
        if (Msg.message == WM_QUIT)
        {
            break;
        }
        TranslateMessage(&Msg);
        DispatchMessage(&Msg);
    }
}

DLL_EXPORT void listener_close()
{
    UnhookWindowsHookEx(context.mouseHook);
    UnhookWindowsHookEx(context.keyboardHook);
}

DLL_EXPORT void listener_setBlock(bool block)
{
    context.blocking = block;
}
