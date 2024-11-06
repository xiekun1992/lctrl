#include "mouse.h"

DLL_EXPORT void mouse_init()
{
}

DLL_EXPORT void mouse_dispose()
{
}

DLL_EXPORT void mouse_move(int x, int y)
{
    switch_to_input_desktop();
    double fScreenWidth = (double)GetSystemMetrics(SM_CXSCREEN) - 1;
    double fScreenHeight = (double)GetSystemMetrics(SM_CYSCREEN) - 1;
    double fx = x * (65535.0f / fScreenWidth);
    double fy = y * (65535.0f / fScreenHeight);

    INPUT input;
    // https://stackoverflow.com/questions/17315672/screen-goes-black-when-i-use-sendinput-to-send-mouse-cursor-positions
    ZeroMemory(&input, sizeof(input));
    input.type = INPUT_MOUSE;
    input.mi.dwFlags = MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE;
    // printf("%d %d\n", fx, fy);
    input.mi.dx = fx;
    input.mi.dy = fy;
    SendInput(1, &input, sizeof(INPUT));
}

DLL_EXPORT void mouse_wheel(enum MouseWheel direction)
{ // -1: up, 1: down
    switch_to_input_desktop();
    INPUT input;
    ZeroMemory(&input, sizeof(input));
    input.type = INPUT_MOUSE;
    input.mi.mouseData = direction > 0 ? WHEEL_DELTA : -1 * WHEEL_DELTA;
    input.mi.dwFlags = MOUSEEVENTF_WHEEL | MOUSEEVENTF_ABSOLUTE;
    input.mi.dx = 0;
    input.mi.dy = 0;
    SendInput(1, &input, sizeof(INPUT));
}

DLL_EXPORT void mouse_down(enum MouseButton button)
{ // 1: left(Button1), 2: middle(Button2), 3: right(Button3)
    switch_to_input_desktop();
    INPUT input;
    ZeroMemory(&input, sizeof(input));
    input.type = INPUT_MOUSE;
    input.mi.dx = 0;
    input.mi.dy = 0;
    switch (button)
    {
    case MOUSE_LEFT:
        input.mi.dwFlags = MOUSEEVENTF_LEFTDOWN | MOUSEEVENTF_ABSOLUTE;
        break;
    case MOUSE_MIDDLE:
        input.mi.dwFlags = MOUSEEVENTF_MIDDLEDOWN | MOUSEEVENTF_ABSOLUTE;
        break;
    case MOUSE_RIGHT:
        input.mi.dwFlags = MOUSEEVENTF_RIGHTDOWN | MOUSEEVENTF_ABSOLUTE;
        break;
    }
    SendInput(1, &input, sizeof(INPUT));
}

DLL_EXPORT void mouse_up(enum MouseButton button)
{
    switch_to_input_desktop();
    INPUT input;
    ZeroMemory(&input, sizeof(input));
    input.type = INPUT_MOUSE;
    input.mi.dx = 0;
    input.mi.dy = 0;
    switch (button)
    {
    case MOUSE_LEFT:
        input.mi.dwFlags = MOUSEEVENTF_LEFTUP | MOUSEEVENTF_ABSOLUTE;
        break;
    case MOUSE_MIDDLE:
        input.mi.dwFlags = MOUSEEVENTF_MIDDLEUP | MOUSEEVENTF_ABSOLUTE;
        break;
    case MOUSE_RIGHT:
        input.mi.dwFlags = MOUSEEVENTF_RIGHTUP | MOUSEEVENTF_ABSOLUTE;
        break;
    }
    SendInput(1, &input, sizeof(INPUT));
}
