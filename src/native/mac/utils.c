#include "utils.h"

DLL_EXPORT bool is_run_as_admin()
{
    // BOOL isAdmin = FALSE;
    // PSID adminGroup = NULL;
    // SID_IDENTIFIER_AUTHORITY ntAuthority = SECURITY_NT_AUTHORITY;

    // if (AllocateAndInitializeSid(&ntAuthority, 2, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_ALIAS_RID_ADMINS, 0, 0, 0, 0, 0, 0, &adminGroup))
    // {
    //     CheckTokenMembership(NULL, adminGroup, &isAdmin);
    //     FreeSid(adminGroup);
    // }
    return true;
}

DLL_EXPORT void run_as_admin()
{
    // TCHAR szPath[MAX_PATH];
    // if (GetModuleFileName(NULL, szPath, ARRAYSIZE(szPath)))
    // {
    //     SHELLEXECUTEINFO sei = {sizeof(sei)};
    //     sei.lpVerb = L"runas";
    //     sei.lpFile = szPath;
    //     sei.hwnd = NULL;
    //     sei.nShow = SW_NORMAL;
    //     if (ShellExecuteEx(&sei))
    //     {
    //         // The program has been successfully started with elevated privileges.
    //         ExitProcess(0);
    //     }
    // }
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
