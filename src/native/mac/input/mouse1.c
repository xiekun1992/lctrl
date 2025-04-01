#include "mouse.h"

int mouseX = 0,
    mouseY = 0;
CGEventRef mouseEv;

DLL_EXPORT void mouse_init()
{
    mouseEv = CGEventCreateMouseEvent(NULL, kCGEventMouseMoved, CGPointMake(0, 0), 0);
}

DLL_EXPORT void mouse_dispose()
{
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

DLL_EXPORT void mouse_move(int x, int y)
{
    mouseX = x;
    mouseY = y;
    // CGEventSourceRef source = CGEventSourceCreate(kCGEventSourceStateCombinedSessionState);
    CGEventSetType(mouseEv, kCGEventMouseMoved);
    CGPoint newPoint = CGPointMake(mouseX, mouseY);
    CGEventSetLocation(mouseEv, newPoint);

    CGEventPost(kCGHIDEventTap, mouseEv);
    // CFRelease(mouse);
    // CFRelease(source);
}

DLL_EXPORT void mouse_down(enum MouseButton button)
{ // 1: left(Button1), 2: middle(Button2), 3: right(Button3)
    CFRelease(mouseEv);
    mouse_init();

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
    // CGEventRef mouseDown = CGEventCreateMouseEvent(NULL, type, CGPointMake(mouseX, mouseY), kCGEventSourceStateHIDSystemState);
    CGEventSetType(mouseEv, type);
    CGPoint newPoint = CGPointMake(mouseX, mouseY);
    CGEventSetLocation(mouseEv, newPoint);

    CGEventPost(kCGHIDEventTap, mouseEv);
    // CFRelease(mouseDown);
}

DLL_EXPORT void mouse_up(enum MouseButton button)
{
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
    // CGEventRef mouseUp = CGEventCreateMouseEvent(NULL, type, CGPointMake(mouseX, mouseY), kCGEventSourceStateHIDSystemState);
    CGEventSetType(mouseEv, type);
    CGPoint newPoint = CGPointMake(mouseX, mouseY);
    CGEventSetLocation(mouseEv, newPoint);

    CGEventPost(kCGHIDEventTap, mouseEv);
    // CFRelease(mouseUp);

    CFRelease(mouseEv);
    mouse_init();
}
