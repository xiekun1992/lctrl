#include "mouse.h"

int mouseX = 0,
    mouseY = 0;
bool isMouseDown = false;

DLL_EXPORT void mouse_init()
{
}

DLL_EXPORT void mouse_dispose()
{
}

DLL_EXPORT void mouse_move(int x, int y)
{
    mouseX = x;
    mouseY = y;
    if (isMouseDown)
    {
        // drag
        CGEventRef mouse = CGEventCreateMouseEvent(NULL, kCGEventLeftMouseDragged, CGPointMake(x, y), 0);
        CGEventPost(kCGHIDEventTap, mouse);
        CFRelease(mouse);
    }
    else
    {
        // CGEventSourceRef source = CGEventSourceCreate(kCGEventSourceStateCombinedSessionState);
        CGEventRef mouse = CGEventCreateMouseEvent(NULL, kCGEventMouseMoved, CGPointMake(x, y), 0);
        CGEventPost(kCGHIDEventTap, mouse);
        CFRelease(mouse);
        // CFRelease(source);
    }
}

DLL_EXPORT void mouse_wheel(enum MouseWheel direction)
{ // -1: up, 1: down
    int delta = 120;
    if (direction < 0)
    {
        delta *= -1;
    }
    CGEventRef event;

    event = CGEventCreateScrollWheelEvent(NULL, kCGScrollEventUnitPixel, 1, delta, 0);
    CGEventPost(kCGHIDEventTap, event);

    CFRelease(event);
}

DLL_EXPORT void mouse_down(enum MouseButton button)
{ // 1: left(Button1), 2: middle(Button2), 3: right(Button3)
    isMouseDown = true;
    CGEventType type;
    switch (button)
    {
    case MOUSE_LEFT:
        type = kCGEventLeftMouseDown;
        break;
    case MOUSE_MIDDLE:
        type = kCGEventLeftMouseDown;
        break;
    case MOUSE_RIGHT:
        type = kCGEventRightMouseDown;
        break;
    }
    CGEventRef mouseDown = CGEventCreateMouseEvent(NULL, type, CGPointMake(mouseX, mouseY), 0);
    CGEventPost(kCGHIDEventTap, mouseDown);
    CFRelease(mouseDown);
}

DLL_EXPORT void mouse_up(enum MouseButton button)
{
    isMouseDown = false;
    CGEventType type;
    switch (button)
    {
    case MOUSE_LEFT:
        type = kCGEventLeftMouseUp;
        break;
    case MOUSE_MIDDLE:
        type = kCGEventLeftMouseUp;
        break;
    case MOUSE_RIGHT:
        type = kCGEventRightMouseUp;
        break;
    }
    CGEventRef mouseUp = CGEventCreateMouseEvent(NULL, type, CGPointMake(mouseX, mouseY), 0);
    CGEventPost(kCGHIDEventTap, mouseUp);
    CFRelease(mouseUp);
}
