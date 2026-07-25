#ifndef ASTRA_SIGNAL_H
#define ASTRA_SIGNAL_H

#include <stdbool.h>
#include <signal.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum
{
    ASTRA_SIGNAL_OK = 0,
    ASTRA_SIGNAL_ERROR = -1
} astra_signal_result;

typedef void (*astra_signal_handler)(int);

astra_signal_result
astra_signal_init(void);

astra_signal_result
astra_signal_install(
    int signal_number,
    astra_signal_handler handler
);

astra_signal_result
astra_signal_ignore(
    int signal_number
);

astra_signal_result
astra_signal_default(
    int signal_number
);

astra_signal_result
astra_signal_block(
    int signal_number
);

astra_signal_result
astra_signal_unblock(
    int signal_number
);

astra_signal_result
astra_signal_send(
    pid_t pid,
    int signal_number
);

#ifdef __cplusplus
}
#endif

#endif
