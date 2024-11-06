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
    Display *display;
    RECT rect = {0};
    int screenCount;
    // 打开与 X 服务器的连接
    display = XOpenDisplay(NULL);
    if (display == NULL)
    {
        // fprintf(stderr, "Unable to open X display\n");
        return rect;
    }
    // 检查 Xinerama 是否可用
    if (XineramaIsActive(display))
    {
        // 获取屏幕数量及其信息
        XineramaScreenInfo *screens = XineramaQueryScreens(display, &screenCount);
        if (screens != NULL)
        {
            // printf("Number of screens: %d\n", screenCount);
            for (int i = 0; i < screenCount; ++i)
            {
                if (screens[i].x_org <= rect.left)
                {
                    rect.left = screens[i].x_org;
                }
                if (screens[i].y_org <= rect.top)
                {
                    rect.top = screens[i].y_org;
                }
                int _w = screens[i].x_org + screens[i].width;
                int _h = screens[i].y_org + screens[i].height;
                if (_w >= rect.right)
                {
                    rect.right = _w;
                }
                if (_h >= rect.bottom)
                {
                    rect.bottom = _h;
                }
                // printf("Screen %d: x = %d, y = %d, width = %d, height = %d\n", i, screens[i].x_org, screens[i].y_org, screens[i].width, screens[i].height);
            }
            XFree(screens);
        }
        else
        {
            // fprintf(stderr, "Failed to query Xinerama screens\n");
        }
    }
    else
    {
        // fprintf(stderr, "Xinerama is not active\n");
    }
    // printf("%d %d %d %d\n", x, y, w, h);
    // 关闭与 X 服务器的连接
    XCloseDisplay(display);
    return rect;
}
