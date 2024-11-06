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

DLL_EXPORT bool write_text(wchar_t *text)
{
    bool res = false;
    return res;
}

DLL_EXPORT wchar_t *read_text()
{
    wchar_t *empty = L"";
    return empty;
}

DLL_EXPORT void capture(void (*fn)(void))
{
    clipboard_context.fn = fn;

    clipboard_context.notify = true;
    Display *display = XOpenDisplay(NULL);
    unsigned long color = BlackPixel(display, DefaultScreen(display));
    Window window = XCreateSimpleWindow(display, DefaultRootWindow(display), 0, 0, 1, 1, 0, color, color);

    const char *bufname_clipboard = "CLIPBOARD";
    const char *bufname_primary = "PRIMARY";
    int event_base, error_base;
    XEvent event;
    Atom bufid_clip = XInternAtom(display, bufname_clipboard, False);
    Atom bufid_pri = XInternAtom(display, bufname_primary, False);

    XFixesQueryExtension(display, &event_base, &error_base);
    XFixesSelectSelectionInput(display, DefaultRootWindow(display), bufid_clip, XFixesSetSelectionOwnerNotifyMask);
    XFixesSelectSelectionInput(display, DefaultRootWindow(display), bufid_pri, XFixesSetSelectionOwnerNotifyMask);

    while (clipboard_context.notify)
    {
        if (XPending(display))
        {
            XNextEvent(display, &event);
            if (
                event.type == event_base + XFixesSelectionNotify &&
                (((XFixesSelectionNotifyEvent *)&event)->selection == bufid_clip || ((XFixesSelectionNotifyEvent *)&event)->selection == bufid_pri))
            {
                // printf("update\n");
                lambda_update_handler();
            }
        }
        // std::this_thread::sleep_for(std::chrono::milliseconds(1)); // prevent high cpu usage
    }

    XDestroyWindow(display, window);
    XCloseDisplay(display);
}

DLL_EXPORT void release()
{
    clipboard_context.notify = false;
}
