#include "utils.h"

DLL_EXPORT void utils_open(wchar_t *path)
{
#if _WIN32 == 1
    setlocale(LC_ALL, "");
    SHELLEXECUTEINFO info = {0};
    info.cbSize = sizeof(info);
    info.nShow = SW_SHOWDEFAULT;
    info.lpFile = L"explorer.exe";
    info.lpParameters = path;
    ShellExecuteEx(&info);
#elif __linux == 1
#endif
}

#if _WIN32 == 1

DLL_EXPORT BOOL is_run_as_admin()
{
    BOOL isAdmin = FALSE;
    PSID adminGroup = NULL;
    SID_IDENTIFIER_AUTHORITY ntAuthority = SECURITY_NT_AUTHORITY;

    if (AllocateAndInitializeSid(&ntAuthority, 2, SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_ALIAS_RID_ADMINS, 0, 0, 0, 0, 0, 0, &adminGroup))
    {
        CheckTokenMembership(NULL, adminGroup, &isAdmin);
        FreeSid(adminGroup);
    }
    return isAdmin;
}

DLL_EXPORT VOID run_as_admin()
{
    TCHAR szPath[MAX_PATH];
    if (GetModuleFileName(NULL, szPath, ARRAYSIZE(szPath)))
    {
        SHELLEXECUTEINFO sei = {sizeof(sei)};
        sei.lpVerb = L"runas";
        sei.lpFile = szPath;
        sei.hwnd = NULL;
        sei.nShow = SW_NORMAL;
        if (ShellExecuteEx(&sei))
        {
            // The program has been successfully started with elevated privileges.
            ExitProcess(0);
        }
    }
}

BOOL CALLBACK monitor_fn(HMONITOR h_monitor, HDC hdc, LPRECT lp_rect, LPARAM dw_data)
{
    RECT *screen_rect = (RECT *)dw_data;
    MONITORINFO info;
    info.cbSize = sizeof(MONITORINFO);
    if (GetMonitorInfo(h_monitor, &info))
    {
        if (info.rcMonitor.left < screen_rect->left)
        {
            screen_rect->left = info.rcMonitor.left;
        }
        if (info.rcMonitor.top < screen_rect->top)
        {
            screen_rect->top = info.rcMonitor.top;
        }
        if (info.rcMonitor.right > screen_rect->right)
        {
            screen_rect->right = info.rcMonitor.right;
        }
        if (info.rcMonitor.bottom > screen_rect->bottom)
        {
            screen_rect->bottom = info.rcMonitor.bottom;
        }
    }
    return TRUE;
}

#endif

DLL_EXPORT RECT get_screen_size()
{
#if _WIN32 == 1
    //   size[0] = GetSystemMetrics(SM_CXSCREEN);
    //   size[1] = GetSystemMetrics(SM_CYSCREEN);
    RECT rect = {0};
    EnumDisplayMonitors(NULL, NULL, monitor_fn, (LPARAM)&rect);
    // size[0] = rect.right - rect.left;
    // size[1] = rect.bottom - rect.top;
    return rect;
#elif __linux == 1
#endif
}
