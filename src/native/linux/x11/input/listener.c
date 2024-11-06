#include "listener.h"

struct Listener listener_context;

DLL_EXPORT void listener_init(
    void (*mouseHanlder)(long *),
    void (*keyboardHanlder)(long *),
    void (*hotkeyHandler)(long[5][7]))
{
    listener_context.mouseHanlder = mouseHanlder;
    listener_context.keyboardHanlder = keyboardHanlder;
    listener_context.hotkeyHandler = hotkeyHandler;
    listener_context.is_lcontrol_down = false;
    listener_context.is_lshift_down = false;
    listener_context.is_lwin_down = false;
    listener_context.is_lalt_down = false;
    listener_context.is_escape_down = false;
    listener_context.blocking = false;

    listener_context.display = XOpenDisplay(NULL);
    if (listener_context.display == NULL)
    {
        return;
    }

    int screen = DefaultScreen(listener_context.display);
    listener_context.root_window = RootWindow(listener_context.display, screen);
}

DLL_EXPORT void listener_dispose()
{
    // PostQuitMessage(0);
}

DLL_EXPORT void listener_listen()
{
    XEvent event;
    while (1)
    {
        XNextEvent(listener_context.display, &event);
        switch (event.type)
        {
        case KeyPress:
        {
            long params[3] = {L_KEYDOWN, (long)XStringToKeysym(XKeysymToString(XKeycodeToKeysym(listener_context.display, event.xkey.keycode, 0))), 0};
            // long params[3] = {L_KEYDOWN, (long)XStringToKeysym(XKeysymToString(XKeycodeToKeysym(ctrl_display, keycode, 0))), 0};
            listener_context.keyboardHanlder(params);
            break;
        }
        case KeyRelease:
        {
            long params[3] = {L_KEYUP, (long)XStringToKeysym(XKeysymToString(XKeycodeToKeysym(listener_context.display, event.xkey.keycode, 0))), 0};
            // long params[3] = {L_KEYUP, (long)XStringToKeysym(XKeysymToString(XKeycodeToKeysym(ctrl_display, keycode, 0))), 0};
            listener_context.keyboardHanlder(params);
            break;
        }
        case ButtonPress:
            switch (event.xbutton.button)
            {
            case 1:
            {
                long params[5] = {L_MOUSEDOWN, event.xbutton.x, event.xbutton.y, L_MOUSE_BUTTON_LEFT, 0};
                listener_context.mouseHanlder(params);
                break;
            }
            case 2:
            {
                long params[5] = {L_MOUSEDOWN, event.xbutton.x, event.xbutton.y, L_MOUSE_BUTTON_MIDLLE, 0};
                listener_context.mouseHanlder(params);
                break;
            }
            case 3:
            {
                long params[5] = {L_MOUSEDOWN, event.xbutton.x, event.xbutton.y, L_MOUSE_BUTTON_RIGHT, 0};
                listener_context.mouseHanlder(params);
                break;
            }
            case 4:
            {
                long params[5] = {L_MOUSEWHEEL, event.xbutton.x, event.xbutton.y, 0, -1};
                listener_context.mouseHanlder(params);
                break; // scroll up
            }
            case 5:
            {
                long params[5] = {L_MOUSEWHEEL, event.xbutton.x, event.xbutton.y, 0, 1};
                listener_context.mouseHanlder(params);
                break; // scroll down
            }
            }
            break;
        case ButtonRelease:
            switch (event.xbutton.button)
            {
            case 1:
            {
                long params[5] = {L_MOUSEUP, event.xbutton.x, event.xbutton.y, L_MOUSE_BUTTON_LEFT, 0};
                listener_context.mouseHanlder(params);
                break;
            }
            case 2:
            {
                long params[5] = {L_MOUSEUP, event.xbutton.x, event.xbutton.y, L_MOUSE_BUTTON_MIDLLE, 0};
                listener_context.mouseHanlder(params);
                break;
            }
            case 3:
            {
                long params[5] = {L_MOUSEUP, event.xbutton.x, event.xbutton.y, L_MOUSE_BUTTON_RIGHT, 0};
                listener_context.mouseHanlder(params);
                break;
            }
            case 4:
                break; // scroll up
            case 5:
                break; // scroll down
            }
            break;
        case MotionNotify:
            long params[5] = {L_MOUSEMOVE, event.xmotion.x, event.xmotion.y, 0, 0};
            listener_context.mouseHanlder(params);
            break;
        }
        // if (event.type == MotionNotify)
        // {
        //     int x = event.xmotion.x;
        //     int y = event.xmotion.y;
        //     printf("Mouse motion event: x=%d, y=%d\n", x, y);
        //     // Limit mouse movement here, for example:
        //     if (x != 1 && y != 100)
        //     {
        //         // Move the mouse to a valid position
        //         XWarpPointer(
        //             listener_context.display,
        //             None,
        //             listener_context.root_window,
        //             0, 0,
        //             0, 0,
        //             1, 100);
        //     }
        // }
    }
}

DLL_EXPORT void listener_close()
{
    // listener_context.blocking = 1;
    XCloseDisplay(listener_context.display);
}

DLL_EXPORT void listener_setBlock(bool block)
{
    listener_context.blocking = block;
    if (block)
    {
        XGrabPointer(
            listener_context.display,
            listener_context.root_window,
            False,
            PointerMotionMask | ButtonPressMask | ButtonReleaseMask,
            GrabModeAsync,
            GrabModeAsync,
            None,
            None,
            CurrentTime);
        XGrabKeyboard(listener_context.display, listener_context.root_window, False, GrabModeAsync, GrabModeAsync, CurrentTime);
    }
    else
    {
        XUngrabPointer(listener_context.display, CurrentTime);
        XUngrabKeyboard(listener_context.display, CurrentTime);
    }
}
