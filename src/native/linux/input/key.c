#include "key.h"

int scancode_to_key(int scancode)
{
    return sc_to_key[scancode];
}
int keycode_to_scancode(int keycode)
{
    return keys[keycode];
}