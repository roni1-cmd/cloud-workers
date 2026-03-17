#include <stdio.h>
#include <time.h>

void log_system_event(int event_code, const char* msg) {
    // Formats a low-level log entry: [Timestamp][Code][Message]
    printf("[%ld] CODE: %d | MSG: %s\n", (long)time(NULL), event_code, msg);
}
