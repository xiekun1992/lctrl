#include "helper.h"

void switch_to_input_desktop()
{
    HDESK currentDesk = GetThreadDesktop(GetCurrentThreadId());
    if (currentDesk == NULL)
    {
        return;
    }
    WCHAR currentDesktopName[256];
    DWORD curNeeded;
    if (!GetUserObjectInformationW(currentDesk, UOI_NAME, currentDesktopName, sizeof(currentDesktopName), &curNeeded))
    {
        CloseDesktop(currentDesk);
        return;
    }

    HDESK inputDesk = OpenInputDesktop(0, false, MAXIMUM_ALLOWED);
    if (inputDesk == NULL)
    {
        CloseDesktop(currentDesk);
        return;
    }
    WCHAR inputDesktopName[256];
    DWORD needed;
    if (!GetUserObjectInformationW(inputDesk, UOI_NAME, inputDesktopName, sizeof(inputDesktopName), &needed))
    {
        CloseDesktop(inputDesk);
        CloseDesktop(currentDesk);
        return;
    }

    if (wcscmp(currentDesktopName, inputDesktopName) == 0)
    {
        CloseDesktop(inputDesk);
        CloseDesktop(currentDesk);
        return;
    }

    if (!SwitchDesktop(inputDesk))
    {
        CloseDesktop(inputDesk);
        CloseDesktop(currentDesk);
        return;
    }
    if (!SetThreadDesktop(inputDesk))
    {
        CloseDesktop(inputDesk);
        CloseDesktop(currentDesk);
        return;
    }
    CloseDesktop(inputDesk);
    CloseDesktop(currentDesk);
}
