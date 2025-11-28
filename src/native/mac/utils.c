#include "utils.h"

DLL_EXPORT bool is_run_as_admin()
{
    return true;
}

DLL_EXPORT void run_as_admin()
{
}

DLL_EXPORT RECT get_screen_size()
{
    RECT rect = {
        .left = 0,
        .top = 0,
        .right = 0,
        .bottom = 0};

    CGDirectDisplayID displays[10];
    int displayCount = 0;
    CGError err = CGGetActiveDisplayList(10, displays, &displayCount);
    if (err == kCGErrorSuccess)
    {
        int x = 0, y = 0, width = 0, height = 0;
        for (int i = 0; i < displayCount; ++i)
        {
            // 输出每个显示器的信息
            CGRect bounds = CGDisplayBounds(displays[i]);
            printf("Display %d: (%f, %f) - %f x %f\n", i, bounds.origin.x, bounds.origin.y, bounds.size.width, bounds.size.height);

            if (bounds.origin.x < x)
            {
                x = bounds.origin.x;
            }
            if (bounds.origin.y < y)
            {
                y = bounds.origin.y;
            }
            int w = bounds.origin.x + bounds.size.width;
            int h = bounds.origin.y + bounds.size.height;
            if (w > width)
            {
                width = w;
            }
            if (h > height)
            {
                height = h;
            }
        }
        printf("x=%d, y=%d, w=%d, h=%d\n", x, y, width, height);
        rect.left = x;
        rect.top = y;
        rect.right = width;
        rect.bottom = height;
    }
    return rect;
}

RECT screens_rect[16];

DLL_EXPORT RECT *get_screens(int *count)
{
    CGDirectDisplayID displays[10];
    int displayCount = 0;
    CGError err = CGGetActiveDisplayList(10, displays, &displayCount);
    if (err == kCGErrorSuccess)
    {
        for (int i = 0; i < displayCount; ++i)
        {
            CGRect bounds = CGDisplayBounds(displays[i]);
            screens_rect[i].left = bounds.origin.x;
            screens_rect[i].top = bounds.origin.y;
            screens_rect[i].right = bounds.origin.x + bounds.size.width;
            screens_rect[i].bottom = bounds.origin.y + bounds.size.height;
        }
        *count = displayCount;
    }
    return &screens_rect;
}