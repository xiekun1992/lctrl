#include "utils.h"

DLL_EXPORT bool is_run_as_admin()
{
    return true;
}

DLL_EXPORT void run_as_admin()
{
}

struct wl_display *display;
struct wl_registry *registry;
struct wl_output *output;
struct wl_list output_list;
struct output
{
    int32_t x, y;
    int32_t width, height;
    struct wl_list link;
};

// Listener for wl_output events
static void handle_geometry(void *data, struct wl_output *wl_output, int32_t x, int32_t y,
                            int32_t physical_width, int32_t physical_height, int32_t subpixel,
                            const char *make, const char *model, int32_t transform)
{
    // This callback is for output geometry, usually we don't need to handle this for resolution
    struct output *output = data;
    output->x = x;
    output->y = y;
}

static void handle_mode(void *data, struct wl_output *wl_output, uint32_t flags, int32_t width,
                        int32_t height, int32_t refresh)
{
    if (flags & WL_OUTPUT_MODE_CURRENT)
    {
        struct output *output = data;
        output->width = width;
        output->height = height;
    }
}

static void handle_done(void *data, struct wl_output *wl_output)
{
    // This callback is for when all output information is sent
}

static void handle_scale(void *data, struct wl_output *wl_output, int32_t factor)
{
    // This callback is for output scale, usually not needed for resolution
}

static const struct wl_output_listener output_listener = {
    handle_geometry,
    handle_mode,
    handle_done,
    handle_scale};

// Listener for registry events
static void registry_handler(void *data, struct wl_registry *registry, uint32_t id,
                             const char *interface, uint32_t version)
{
    if (strcmp(interface, wl_output_interface.name) == 0)
    {
        struct wl_output *output = wl_registry_bind(registry, id, &wl_output_interface, 1);
        struct output *output_info = malloc(sizeof(struct output));
        wl_list_insert(&output_list, &output_info->link);
        wl_output_add_listener(output, &output_listener, output_info);
    }
}

static void registry_remover(void *data, struct wl_registry *registry, uint32_t id)
{
    // Handle removal of registry objects if necessary
}

static const struct wl_registry_listener registry_listener = {
    registry_handler,
    registry_remover};

DLL_EXPORT RECT get_screen_size()
{
    RECT rect = {
        .left = 0,
        .top = 0,
        .right = 0,
        .bottom = 0};

    display = wl_display_connect(NULL);
    if (display == NULL)
    {
        fprintf(stderr, "Failed to connect to Wayland display\n");
        return rect;
    }

    wl_list_init(&output_list);

    // Get the Wayland registry
    registry = wl_display_get_registry(display);
    wl_registry_add_listener(registry, &registry_listener, NULL);
    wl_display_roundtrip(display);

    // Wait for the output information to be received
    wl_display_roundtrip(display);

    int x = 0, y = 0, width = 0, height = 0;
    struct output *obj;
    wl_list_for_each(obj, &output_list, link)
    {
        if (obj->x < x)
        {
            x = obj->x;
        }
        if (obj->y < y)
        {
            y = obj->y;
        }
        int w = obj->x + obj->width;
        int h = obj->y + obj->height;
        if (w > width)
        {
            width = w;
        }
        if (h > height)
        {
            height = h;
        }
        // printf("Data: %d %d %d %d\n", obj->x, obj->y, obj->width, obj->height);
        // wl_list_remove(&obj->link);
    }

    // printf("%d %d %d %d\n", x, y, width, height);
    rect.left = x;
    rect.top = y;
    rect.right = width;
    rect.bottom = height;

    // Clean up and disconnect from Wayland display
    wl_display_disconnect(display);

    return rect;
}
