#pragma once
#include "../export.h"

#include <stdbool.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <stddef.h>
#include <stdint.h>
#include <string.h>
#include <fcntl.h>
#include <unistd.h>
#include <poll.h>
#include <libevdev-1.0/libevdev/libevdev.h>
#include <dirent.h>
#include <sys/stat.h>

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

struct DevNode
{
    char full_path[32];
    struct DevNode *next;
};

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

    struct DevNode *head;
    struct DevNode *tail;
    struct pollfd fds[32];
    struct libevdev *devs[32];

    struct input_event ev_stack[32];
    int stack_top;
};

DLL_EXPORT void listener_init(
    void (*mouseHanlder)(long *),
    void (*keyboardHanlder)(long *),
    void (*hotkeyHandler)(long[5][7]));
DLL_EXPORT void listener_dispose();
DLL_EXPORT void listener_listen();
DLL_EXPORT void listener_close();
DLL_EXPORT void listener_setBlock(bool block);
