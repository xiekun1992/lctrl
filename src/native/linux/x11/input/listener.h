#pragma once
#include "../export.h"

#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

#include <unistd.h>
#include <X11/Xlibint.h>
#include <X11/Xlib.h>
#include <X11/Xutil.h>
#include <X11/cursorfont.h>
#include <X11/keysymdef.h>
#include <X11/keysym.h>
#include <X11/extensions/record.h>
#include <X11/extensions/XTest.h>

/* for this struct, refer to libxnee */
typedef union
{
    unsigned char type;
    xEvent event;
    xResourceReq req;
    xGenericReply reply;
    xError error;
    xConnSetupPrefix setup;
} XRecordDatum;

#define L_MOUSEWHEEL 0
#define L_MOUSEMOVE 1
#define L_MOUSEDOWN 2
#define L_MOUSEUP 3
#define L_KEYDOWN 4
#define L_KEYUP 5
#define L_MOUSEMOVEREL 6

#define L_MOUSE_BUTTON_LEFT 1
#define L_MOUSE_BUTTON_MIDLLE 2
#define L_MOUSE_BUTTON_RIGHT 3

struct Listener
{
    void (*mouseHanlder)(long *);
    void (*keyboardHanlder)(long *);
    void (*hotkeyHandler)(long[5][7]);
    bool blocking;
    bool is_lcontrol_down;
    bool is_lshift_down;
    bool is_lwin_down;
    bool is_lalt_down;
    bool is_escape_down;
    Display *display;
    Window root_window;
};

DLL_EXPORT void listener_init(
    void (*mouseHanlder)(long *),
    void (*keyboardHanlder)(long *),
    void (*hotkeyHandler)(long[5][7]));
DLL_EXPORT void listener_dispose();
DLL_EXPORT void listener_listen();
DLL_EXPORT void listener_close();
DLL_EXPORT void listener_setBlock(bool block);
