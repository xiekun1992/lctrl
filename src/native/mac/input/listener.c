#include "listener.h"

struct Listener context;
static CGPoint fixedMousePosition = {-1, -1};
int prevDeltaX = 0, prevDeltaY = 0;
int wheeling = 0;
static CGEventFlags lastFlags = 0;

// 回调函数：处理鼠标输入事件
void handleHIDInput(void *context1, IOReturn result, void *sender, IOHIDValueRef value)
{
    IOHIDElementRef element = IOHIDValueGetElement(value);
    uint32_t usagePage = IOHIDElementGetUsagePage(element);
    uint32_t usage = IOHIDElementGetUsage(element);
    int deltaX = 0, deltaY = 0;
    // printf("deltaX=%d, deltaY=%d\n", deltaX, deltaY);
    if (usagePage == kHIDPage_GenericDesktop)
    {
        if (usage == kHIDUsage_GD_X)
        {
            deltaX = (int)IOHIDValueGetIntegerValue(value);
        }
        if (usage == kHIDUsage_GD_Y)
        {
            deltaY = (int)IOHIDValueGetIntegerValue(value);
        }
        if (context.blocking && !wheeling)
        {
            if (prevDeltaX != deltaX || prevDeltaY != deltaY)
            {
                prevDeltaX = deltaX;
                prevDeltaY = deltaY;
                int params[5] = {L_MOUSEMOVEREL, deltaX, deltaY, 0, 0};
                context.mouseHanlder(params);
            }
        }
    }
}

// 回调函数，用于处理捕获的鼠标事件
CGEventRef eventCallback(CGEventTapProxy proxy, CGEventType type, CGEventRef event, void *refcon)
{
    if (type == kCGEventFlagsChanged)
    {
        CGEventFlags currentFlags = CGEventGetFlags(event);
        CGEventFlags changedFlags = currentFlags ^ lastFlags; // 计算状态变化

        // 检查 Command 键的状态变化
        if (changedFlags & kCGEventFlagMaskCommand)
        {
            if (currentFlags & kCGEventFlagMaskCommand)
            {
                printf("Command right键按下\n");
                int params[7] = {L_KEYDOWN, (int)kVK_Command, (int)keys[kVK_Command]};
                context.keyboardHanlder(params);
                context.is_lwin_down = true;
            }
            else
            {
                printf("Command right键释放\n");
                int params[7] = {L_KEYUP, (int)kVK_Command, (int)keys[kVK_Command]};
                context.keyboardHanlder(params);
                context.is_lwin_down = false;
            }
        }
        if (changedFlags & kCGEventFlagMaskShift)
        {
            if (currentFlags & kCGEventFlagMaskShift)
            {
                printf("shift left键按下\n");
                int params[7] = {L_KEYDOWN, (int)kVK_Shift, (int)keys[kVK_Shift]};
                context.keyboardHanlder(params);
                context.is_lshift_down = true;
            }
            else
            {
                printf("shift left键释放\n");
                int params[7] = {L_KEYUP, (int)kVK_Shift, (int)keys[kVK_Shift]};
                context.keyboardHanlder(params);
                context.is_lshift_down = false;
            }
        }
        if (changedFlags & kCGEventFlagMaskAlphaShift)
        {
            if (currentFlags & kCGEventFlagMaskAlphaShift)
            {
                printf("shift right键按下\n");
                int params[7] = {L_KEYDOWN, (int)kVK_RightShift, (int)keys[kVK_RightShift]};
                context.keyboardHanlder(params);
            }
            else
            {
                printf("shift right键释放\n");
                int params[7] = {L_KEYUP, (int)kVK_RightShift, (int)keys[kVK_RightShift]};
                context.keyboardHanlder(params);
            }
        }
        if (changedFlags & kCGEventFlagMaskControl)
        {
            if (currentFlags & kCGEventFlagMaskControl)
            {
                printf("control left键按下\n");
                int params[7] = {L_KEYDOWN, (int)kVK_Control, (int)keys[kVK_Control]};
                context.keyboardHanlder(params);
                context.is_lcontrol_down = true;
            }
            else
            {
                printf("control left键释放\n");
                int params[7] = {L_KEYUP, (int)kVK_Control, (int)keys[kVK_Control]};
                context.keyboardHanlder(params);
                context.is_lcontrol_down = false;
            }
        }
        if (changedFlags & kCGEventFlagMaskAlternate)
        {
            if (currentFlags & kCGEventFlagMaskAlternate)
            {
                printf("Alternate left键按下\n");
                int params[7] = {L_KEYDOWN, (int)kVK_Option, (int)keys[kVK_Option]};
                context.keyboardHanlder(params);
                context.is_lalt_down = true;
            }
            else
            {
                printf("Alternate left键释放\n");
                int params[7] = {L_KEYUP, (int)kVK_Option, (int)keys[kVK_Option]};
                context.keyboardHanlder(params);
                context.is_lalt_down = false;
            }
        }

        lastFlags = currentFlags; // 更新状态

        printf("hotkey down: %d %d %d %d\n", context.is_lcontrol_down, context.is_lshift_down, context.is_lwin_down, context.is_lalt_down);
        if (
            // context.is_lcontrol_down &&
            context.is_lwin_down &&
            context.is_lalt_down)
        {
            printf("hotkeys pressed\n");
            int hotkeys[][7] = {
                // {L_KEYUP, (int)kVK_Control, (int)keys[kVK_Control], 0, 0, 0, 0},
                {L_KEYUP, (int)kVK_Command, (int)keys[kVK_Command], 0, 0, 0, 0},
                {L_KEYUP, (int)kVK_Option, (int)keys[kVK_Option], 0, 0, 0, 0},
                // {L_KEYUP, (int)kVK_Shift, (int)keys[kVK_Shift], 0, 0, 0, 0},
                // {L_KEYUP, (int)kVK_Escape, (int)keys[kVK_Escape], 0, 0, 0, 0},
            };
            context.hotkeyHandler(hotkeys);
        }
    }
    CGPoint mouseLocation = CGEventGetLocation(event);
    int x = mouseLocation.x, y = mouseLocation.y;
    if ((int)(mouseLocation.x * 10) > ((int)(mouseLocation.x) * 10))
    {
        x = (int)(mouseLocation.x) + 1;
    }
    else
    {
        x = (int)(mouseLocation.x);
    }
    if ((int)(mouseLocation.y * 10) > ((int)(mouseLocation.y) * 10))
    {
        y = (int)(mouseLocation.y) + 1;
    }
    else
    {
        y = (int)(mouseLocation.y);
    }
    // printf("context.blocking=%d, mouseLocation=%f %f, %ld %ld\n", context.blocking, mouseLocation.x, mouseLocation.y, (int)x, (int)y);
    switch (type)
    {
    case kCGEventKeyDown:
    { // 获取按下的键码（虚拟键码，如 kVK_ANSI_A 对应 "A"）
        CGKeyCode keyCode = (CGKeyCode)CGEventGetIntegerValueField(event, kCGKeyboardEventKeycode);

        // 获取事件类型描述
        const char *eventType = "按下";

        // 打印键码和事件类型
        printf("检测到键盘事件：键码 %d %s\n", keys[keyCode], eventType);
        int params[7] = {L_KEYDOWN, (int)keyCode, (int)keys[keyCode]};
        context.keyboardHanlder(params);
        break;
    }
    case kCGEventKeyUp:
    { // 获取按下的键码（虚拟键码，如 kVK_ANSI_A 对应 "A"）
        CGKeyCode keyCode = (CGKeyCode)CGEventGetIntegerValueField(event, kCGKeyboardEventKeycode);

        // 获取事件类型描述
        const char *eventType = "释放";

        // 打印键码和事件类型
        printf("检测到键盘事件：键码 %d %s\n", keys[keyCode], eventType);
        int params[7] = {L_KEYUP, (int)keyCode, (int)keys[keyCode]};
        context.keyboardHanlder(params);
        break;
    }
    case kCGEventScrollWheel:
    { // 滚轮滚动
        int64_t delta = CGEventGetIntegerValueField(event, kCGScrollWheelEventDeltaAxis1);
        double delta1 = CGEventGetIntegerValueField(event, kCGScrollWheelEventDeltaAxis1);
        printf("Scroll wheel moved: %f\n", delta1);
        if (delta != 0)
        {
            wheeling = 1;
            int params[5] = {L_MOUSEWHEEL, (int)x, (int)y, 0, delta};
            context.mouseHanlder(params);
        }
        break;
    }
    case kCGEventMouseMoved: // 鼠标移动
    {
        wheeling = 0;
        int params[5] = {L_MOUSEMOVE, (int)x, (int)y, 0, (int)(0)};
        context.mouseHanlder(params);
        break;
    }
    case kCGEventLeftMouseDown: // 左键按下
    {
        wheeling = 0;
        printf("Left mouse button down\n");
        int params[5] = {L_MOUSEDOWN, (int)x, (int)y, L_MOUSE_BUTTON_LEFT, 0};
        context.mouseHanlder(params);
        break;
    }
    case kCGEventLeftMouseUp: // 左键释放
    {
        wheeling = 0;
        printf("Left mouse button up\n");
        int params[5] = {L_MOUSEUP, (int)x, (int)y, L_MOUSE_BUTTON_LEFT, 0};
        context.mouseHanlder(params);
        break;
    }
    case kCGEventRightMouseDown: // 右键按下
    {
        wheeling = 0;
        printf("Right mouse button down\n");
        int params[5] = {L_MOUSEDOWN, (int)x, (int)y, L_MOUSE_BUTTON_RIGHT, 0};
        context.mouseHanlder(params);
        break;
    }
    case kCGEventRightMouseUp: // 右键释放
    {
        wheeling = 0;
        printf("Right mouse button up\n");
        int params[5] = {L_MOUSEUP, (int)x, (int)y, L_MOUSE_BUTTON_RIGHT, 0};
        context.mouseHanlder(params);
        break;
    }
    // case kCGEventOtherMouseDown: // 其他鼠标按钮按下（如中键）
    //     printf("Other mouse button down\n");
    //     break;
    // case kCGEventOtherMouseUp: // 其他鼠标按钮释放
    //     printf("Other mouse button up\n");
    //     break;
    default:
        break;
    }
    if (context.blocking)
    {
        if (fixedMousePosition.x == -1 && fixedMousePosition.y == -1)
        {
            fixedMousePosition.x = mouseLocation.x;
            fixedMousePosition.y = mouseLocation.y;
        }

        CGWarpMouseCursorPosition(fixedMousePosition);
        return NULL;
    }
    // 将事件传递给下一个监听器
    return event;
}

DLL_EXPORT void listener_init(
    void (*mouseHanlder)(int *),
    void (*keyboardHanlder)(int *),
    void (*hotkeyHandler)(int[5][7]))
{
    context.mouseHanlder = mouseHanlder;
    context.keyboardHanlder = keyboardHanlder;
    context.hotkeyHandler = hotkeyHandler;
    context.is_lcontrol_down = false;
    context.is_lshift_down = false;
    context.is_lwin_down = false;
    context.is_lalt_down = false;
    context.is_escape_down = false;
    context.blocking = false;

    // 创建 HID 管理器
    IOHIDManagerRef manager = IOHIDManagerCreate(kCFAllocatorDefault, kIOHIDOptionsTypeNone);

    // 设置设备匹配字典（匹配鼠标设备）
    CFMutableDictionaryRef matchDict = CFDictionaryCreateMutable(kCFAllocatorDefault, 0,
                                                                 &kCFTypeDictionaryKeyCallBacks,
                                                                 &kCFTypeDictionaryValueCallBacks);
    int usagePage = 1; // Generic Desktop Controls
    int usage = 2;     // Mouse
    CFNumberRef usagePageRef = CFNumberCreate(kCFAllocatorDefault, kCFNumberIntType, &usagePage);
    CFNumberRef usageRef = CFNumberCreate(kCFAllocatorDefault, kCFNumberIntType, &usage);
    CFDictionarySetValue(matchDict, CFSTR(kIOHIDDeviceUsagePageKey), usagePageRef);
    CFDictionarySetValue(matchDict, CFSTR(kIOHIDDeviceUsageKey), usageRef);

    IOHIDManagerSetDeviceMatching(manager, matchDict);

    // 设置回调函数
    IOHIDManagerRegisterInputValueCallback(manager, handleHIDInput, NULL);

    // 启用 HID 管理器
    IOHIDManagerScheduleWithRunLoop(manager, CFRunLoopGetCurrent(), kCFRunLoopDefaultMode);
    IOReturn ret = IOHIDManagerOpen(manager, kIOHIDOptionsTypeNone);
    if (ret != kIOReturnSuccess)
    {
        fprintf(stderr, "错误：无法打开 HID Manager（权限不足？）\n");
        return;
    }

    CFMachPortRef eventTap = CGEventTapCreate(
        kCGHIDEventTap,           // 监听硬件层的事件
        kCGHeadInsertEventTap,    // 插入到事件队列的头部
        kCGEventTapOptionDefault, // 默认选项
        CGEventMaskBit(kCGEventKeyUp) |
            CGEventMaskBit(kCGEventKeyDown) |
            CGEventMaskBit(kCGEventLeftMouseDragged) |
            CGEventMaskBit(kCGEventRightMouseDragged) |
            CGEventMaskBit(kCGEventMouseMoved) |
            CGEventMaskBit(kCGEventLeftMouseDown) |
            CGEventMaskBit(kCGEventLeftMouseUp) |
            CGEventMaskBit(kCGEventRightMouseDown) |
            CGEventMaskBit(kCGEventRightMouseUp) |
            CGEventMaskBit(kCGEventScrollWheel) |
            CGEventMaskBit(kCGEventOtherMouseDown) |
            CGEventMaskBit(kCGEventOtherMouseUp) |
            CGEventMaskBit(kCGEventOtherMouseDragged) |
            CGEventMaskBit(kCGEventFlagsChanged), // 监听鼠标移动事件
        eventCallback,                            // 回调函数
        NULL                                      // 用户数据
    );

    if (!eventTap)
    {
        fprintf(stderr, "Failed to create event tap. Enable accessibility permissions.\n");
        return;
    }

    // 创建运行循环源
    CFRunLoopSourceRef runLoopSource = CFMachPortCreateRunLoopSource(kCFAllocatorDefault, eventTap, 0);

    // 将源添加到当前运行循环
    CFRunLoopAddSource(CFRunLoopGetCurrent(), runLoopSource, kCFRunLoopCommonModes);

    // 启用事件监听
    CGEventTapEnable(eventTap, true);

    // ------------------------------------------

    printf("Listening for mouse input...\n");

    printf("Listening for mouse movements...\n");

    // 运行事件循环
    CFRunLoopRun();

    // 清理资源（理论上不会执行到这里）
    CFRelease(runLoopSource);
    CFRelease(eventTap);

    // 清理资源（理论上不会到达这里）
    CFRelease(manager);
    CFRelease(matchDict);
    CFRelease(usagePageRef);
    CFRelease(usageRef);
}

DLL_EXPORT void listener_dispose()
{
}

DLL_EXPORT void listener_listen()
{
}

DLL_EXPORT void listener_close()
{
}

DLL_EXPORT void listener_setBlock(bool block)
{
    context.blocking = block;
    if (!block)
    {
        fixedMousePosition.x = -1;
        fixedMousePosition.y = -1;
    }
}