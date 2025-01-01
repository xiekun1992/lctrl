#include "service.h"

SERVICE_STATUS_HANDLE service_status_handle = NULL;
SERVICE_STATUS service_status = {0};
HANDLE hPipe;

DWORD get_process_id_by_name(const wchar_t *process_name)
{
    DWORD processId = 0;
    HANDLE hSnapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
    if (hSnapshot == INVALID_HANDLE_VALUE)
    {
        // std::wcerr << L"CreateToolhelp32Snapshot failed. Error: " << GetLastError() << std::endl;
        return 0;
    }

    PROCESSENTRY32W pe32;
    pe32.dwSize = sizeof(PROCESSENTRY32W);

    if (Process32First(hSnapshot, &pe32))
    {
        do
        {
            if (wcscmp(process_name, pe32.szExeFile) == 0)
            {
                processId = pe32.th32ProcessID;
                break;
            }
        } while (Process32Next(hSnapshot, &pe32));
    }
    else
    {
        // std::wcerr << L"Process32FirstW failed. Error: " << GetLastError() << std::endl;
    }

    CloseHandle(hSnapshot);
    return processId;
}

DLL_EXPORT void connectPipe()
{
    HANDLE hPipe;
    DWORD dwWritten;
    hPipe = CreateFile(PIPE_NAME, GENERIC_READ | GENERIC_WRITE, 0, NULL, OPEN_EXISTING, 0, NULL);

    if (hPipe == INVALID_HANDLE_VALUE)
    {
        // std::cerr << "Failed to open pipe. Error: " << GetLastError() << std::endl;
        return;
    }
    while (true)
    {
        char buffer[512];
        DWORD dwRead;
        if (ReadFile(hPipe, buffer, sizeof(buffer), &dwRead, NULL))
        {
            // printf("Received from pipe: %s\n", buffer);
            // std::cout << "Received from service: " << buffer << std::endl;
            // if (strcmp(buffer, "app:close") == 0)
            // {
            // }
            CloseHandle(hPipe);
            ExitProcess(0);
            // MessageBox(
            //     NULL,
            //     (LPCWSTR)L"Resource not available\nDo you want to try again?",
            //     (LPCWSTR)L"Account Details",
            //     MB_ICONWARNING | MB_CANCELTRYCONTINUE | MB_DEFBUTTON2);
        }
        else
        {
            // printf("read failed from pipe\n");
        }
        Sleep(1000);
    }
}

DLL_EXPORT VOID delete_service()
{
    SC_HANDLE hSCManager = OpenSCManager(NULL, NULL, SC_MANAGER_ALL_ACCESS);
    if (hSCManager != NULL)
    {
        SC_HANDLE hService = OpenService(hSCManager, SERVICE_NAME, SERVICE_ALL_ACCESS);
        if (hService != NULL)
        {
            DeleteService(hService);
            CloseServiceHandle(hService);
        }
        CloseServiceHandle(hSCManager);
    }
}

DLL_EXPORT VOID create_service()
{
    SC_HANDLE hSCManager = OpenSCManager(NULL, NULL, SC_MANAGER_ALL_ACCESS);
    if (hSCManager != NULL)
    {
        wchar_t path[MAX_PATH];
        if (GetModuleFileName(NULL, path, MAX_PATH) != 0)
        {
            wcscat(path, L" --run_as_service");
            SC_HANDLE hService = CreateService(
                hSCManager,
                SERVICE_NAME,
                NULL,
                SERVICE_ALL_ACCESS,
                SERVICE_WIN32_OWN_PROCESS,
                SERVICE_AUTO_START,
                SERVICE_ERROR_NORMAL,
                path,
                NULL,
                NULL,
                NULL,
                NULL,
                NULL);

            CloseServiceHandle(hService);
        }
        CloseServiceHandle(hSCManager);
    }
}

DLL_EXPORT VOID stop_service()
{
    SC_HANDLE hSCManager = OpenSCManager(NULL, NULL, SC_MANAGER_ALL_ACCESS);
    if (hSCManager != NULL)
    {
        SC_HANDLE hService = OpenService(hSCManager, SERVICE_NAME, SERVICE_ALL_ACCESS);
        if (hService != NULL)
        {
            SERVICE_STATUS serviceStatus;
            if (QueryServiceStatus(hService, &serviceStatus))
            {
                if (serviceStatus.dwCurrentState == SERVICE_RUNNING)
                {
                    SERVICE_STATUS status;
                    if (ControlService(hService, SERVICE_CONTROL_STOP, &status))
                    {
                        // std::cout << "stopped" << "\n";
                    }
                    else
                    {
                        // std::cout << "stop fail: " << GetLastError() << "\n";
                    }
                }
            }
            CloseServiceHandle(hService);
        }
        CloseServiceHandle(hSCManager);
    }
}

DLL_EXPORT VOID start_service()
{
    SC_HANDLE hSCManager = OpenSCManager(NULL, NULL, SC_MANAGER_ALL_ACCESS);
    if (hSCManager != NULL)
    {
        SC_HANDLE hService = OpenService(hSCManager, SERVICE_NAME, SERVICE_ALL_ACCESS);
        if (hService != NULL)
        {
            SERVICE_STATUS serviceStatus;
            if (QueryServiceStatus(hService, &serviceStatus))
            {
                if (serviceStatus.dwCurrentState == SERVICE_STOPPED)
                {
                    // std::cout << serviceStatus.dwCurrentState << "\n";
                    if (StartService(hService, 0, NULL))
                    {
                        // std::cout << "started" << "\n";
                    }
                    else
                    {
                        // std::cout << "start fail: " << GetLastError() << "\n";
                    }
                }
            }
            CloseServiceHandle(hService);
        }
        CloseServiceHandle(hSCManager);
    }
}

VOID run()
{
    // create named pipe
    hPipe = CreateNamedPipe(PIPE_NAME, PIPE_ACCESS_DUPLEX, PIPE_TYPE_MESSAGE | PIPE_READMODE_MESSAGE | PIPE_WAIT,
                            PIPE_UNLIMITED_INSTANCES, 512, 512, 0, NULL);

    if (hPipe == INVALID_HANDLE_VALUE)
    {
        // std::cerr << "Failed to create pipe server. Error: " << GetLastError() << std::endl;
        return;
    }

    wchar_t path[MAX_PATH];
    wchar_t *appname = NULL;
    if (GetModuleFileName(NULL, path, MAX_PATH) != 0)
    {
        WCHAR *filename = wcsrchr(path, '\\');
        wchar_t directory[MAX_PATH];
        if (filename != NULL)
        {
            size_t dir_length = filename - path + 1;
            wcsncpy(directory, path, dir_length);
            directory[dir_length] = L'\0';

            filename++;
            appname = filename;
            // if (get_process_id_by_name(filename) > 0)
            // {
            //     return;
            // }
        }

        DWORD pid = get_process_id_by_name(L"winlogon.exe");
        if (pid <= 0)
        {
            return;
        }

        HANDLE handle = OpenProcess(PROCESS_ALL_ACCESS, FALSE, pid);
        HANDLE token = NULL;
        if (OpenProcessToken(handle, TOKEN_ALL_ACCESS, &token))
        {

            STARTUPINFO si;
            ZeroMemory(&si, sizeof si);
            si.cb = sizeof si;
            si.dwFlags = STARTF_USESHOWWINDOW;

            const wchar_t *desktop1 = L"winsta0\\Default";
            wchar_t desktop[MAX_PATH];
            wcscpy(desktop, desktop1);
            si.lpDesktop = (LPWSTR)desktop;

            PROCESS_INFORMATION pi;

            // const wchar_t *cmd1 = L"D:\\rust_workspaces\\lctrl_rust\\target\\debug\\lctrl_rust.exe";
            // wchar_t cmd[MAX_PATH];
            // wcscpy(cmd, cmd1);
            wcscat(path, L" --run_as_app");
            CreateProcessAsUser(
                token, NULL,
                (LPWSTR)path,
                NULL, NULL, FALSE,
                DETACHED_PROCESS,
                NULL,
                directory,
                &si, &pi);

            CloseHandle(pi.hProcess);
            CloseHandle(pi.hThread);
            CloseHandle(token);
            CloseHandle(handle);
        }

        BOOL isConnected = ConnectNamedPipe(hPipe, NULL);
        if (!isConnected)
        {
            // std::cerr << "Failed to connect pipe. Error: " << GetLastError() << std::endl;
            CloseHandle(hPipe);
            return;
        }

        while (service_status.dwCurrentState != SERVICE_STOPPED)
        {
            Sleep(1000);
        }
    }

    // if (appname != NULL)
    // {
    //     DWORD app_pid = get_process_id_by_name(appname);
    //     HANDLE hProcess = OpenProcess(PROCESS_ALL_ACCESS, FALSE, app_pid);
    //     if (hProcess > 0)
    //     {
    //         TerminateProcess(hProcess, 0);
    //     }
    // }

    // service_status.dwWin32ExitCode = 0;
    // service_status.dwCurrentState = SERVICE_STOPPED;
    // SetServiceStatus(service_status_handle, &service_status);
}

VOID WINAPI service_handler(DWORD dwControl)
{
    switch (dwControl)
    {
    case SERVICE_CONTROL_STOP:
    {
        service_status.dwCurrentState = SERVICE_STOP_PENDING;
        SetServiceStatus(service_status_handle, &service_status);

        DWORD dwWritten;
        const char *response = "app:close";
        WriteFile(hPipe, response, strlen(response), &dwWritten, NULL);

        Sleep(2000);
        CloseHandle(hPipe);

        service_status.dwWin32ExitCode = 0;
        service_status.dwCurrentState = SERVICE_STOPPED;
        SetServiceStatus(service_status_handle, &service_status);
        break;
    }
    }
}

VOID WINAPI service_main(DWORD dwNumServicesArgs, LPWSTR *lpServiceArgVectors)
{
    service_status_handle = RegisterServiceCtrlHandler(SERVICE_NAME, service_handler);
    if (!service_status_handle)
    {
        return;
    }

    service_status.dwServiceType = SERVICE_WIN32_OWN_PROCESS | SERVICE_INTERACTIVE_PROCESS;
    service_status.dwCurrentState = SERVICE_START_PENDING;
    service_status.dwControlsAccepted = SERVICE_ACCEPT_STOP | SERVICE_ACCEPT_SHUTDOWN;
    SetServiceStatus(service_status_handle, &service_status);

    service_status.dwCurrentState = SERVICE_RUNNING;
    SetServiceStatus(service_status_handle, &service_status);

    run();
}

DLL_EXPORT int register_service()
{
    SERVICE_TABLE_ENTRY service_table[] = {
        {(LPWSTR)SERVICE_NAME, service_main},
        {NULL, NULL}};
    if (!StartServiceCtrlDispatcher(service_table))
    {
        return GetLastError();
    }
    return 0;
}
