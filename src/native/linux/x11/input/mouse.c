#include "mouse.h"

struct Mouse mouse_context;

DLL_EXPORT void mouse_init()
{
    mouse_context.display = XOpenDisplay(0);
    mouse_context.root = DefaultRootWindow(mouse_context.display);
}

DLL_EXPORT void mouse_dispose()
{
    XCloseDisplay(mouse_context.display);
}

DLL_EXPORT void mouse_move(int x, int y)
{
    XWarpPointer(mouse_context.display, None, mouse_context.root, 0, 0, 0, 0, x, y);
    XSync(mouse_context.display, false);
}

DLL_EXPORT void mouse_wheel(enum MouseWheel direction)
{ // -1: up, 1: down
    int button;
    if (direction > 0)
    {
        button = Button4;
    }
    if (direction < 0)
    {
        button = Button5;
    }
    XTestFakeButtonEvent(mouse_context.display, button, true, 0);
    XFlush(mouse_context.display);
    XTestFakeButtonEvent(mouse_context.display, button, false, 0);
    XFlush(mouse_context.display);
}

DLL_EXPORT void mouse_down(enum MouseButton button)
{ // 1: left(Button1), 2: middle(Button2), 3: right(Button3)
    XTestFakeButtonEvent(mouse_context.display, button, true, 0);
    XFlush(mouse_context.display);
}

DLL_EXPORT void mouse_up(enum MouseButton button)
{
    XTestFakeButtonEvent(mouse_context.display, button, false, 0);
    XFlush(mouse_context.display);
}
