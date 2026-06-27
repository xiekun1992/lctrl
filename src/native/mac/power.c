#include "power.h"
#include <IOKit/pwr_mgt/IOPMLib.h>

static IOPMAssertionID display_assertion = kIOPMNullAssertionID;
static IOPMAssertionID idle_assertion = kIOPMNullAssertionID;
static bool block_prevent = false;
static bool replay_prevent = false;

static void update_sleep_assertions() {
    bool should_prevent = block_prevent || replay_prevent;
    if (should_prevent) {
        if (display_assertion == kIOPMNullAssertionID) {
            IOPMAssertionCreateWithName(kIOPMAssertionTypePreventUserIdleDisplaySleep, kIOPMAssertionLevelOn, CFSTR("lctrl remote control"), &display_assertion);
        }
        if (idle_assertion == kIOPMNullAssertionID) {
            IOPMAssertionCreateWithName(kIOPMAssertionTypePreventUserIdleDisplaySleep, kIOPMAssertionLevelOn, CFSTR("lctrl remote control"), &idle_assertion);
        }
    } else {
        if (display_assertion != kIOPMNullAssertionID) {
            IOPMAssertionRelease(display_assertion);
            display_assertion = kIOPMNullAssertionID;
        }
        if (idle_assertion != kIOPMNullAssertionID) {
            IOPMAssertionRelease(idle_assertion);
            idle_assertion = kIOPMNullAssertionID;
        }
    }
}

DLL_EXPORT void power_set_block_prevent(bool prevent) {
    block_prevent = prevent;
    update_sleep_assertions();
}

DLL_EXPORT void power_set_replay_prevent(bool prevent) {
    replay_prevent = prevent;
    update_sleep_assertions();
}