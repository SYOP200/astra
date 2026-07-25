#include "astra_signal.h"

#include <errno.h>
#include <string.h>
#include <unistd.h>

astra_signal_result
astra_signal_init(void)
{
    if (astra_signal_ignore(SIGPIPE) != ASTRA_SIGNAL_OK)
    {
        return ASTRA_SIGNAL_ERROR;
    }

    return ASTRA_SIGNAL_OK;
}

astra_signal_result
astra_signal_install(
    int signal_number,
    astra_signal_handler handler
)
{
    struct sigaction action;

    memset(&action, 0, sizeof(action));

    action.sa_handler = handler;
    sigemptyset(&action.sa_mask);
    action.sa_flags = SA_RESTART;

    if (sigaction(signal_number, &action, NULL) != 0)
    {
        return ASTRA_SIGNAL_ERROR;
    }

    return ASTRA_SIGNAL_OK;
}

astra_signal_result
astra_signal_ignore(
    int signal_number
)
{
    struct sigaction action;

    memset(&action, 0, sizeof(action));

    action.sa_handler = SIG_IGN;
    sigemptyset(&action.sa_mask);

    if (sigaction(signal_number, &action, NULL) != 0)
    {
        return ASTRA_SIGNAL_ERROR;
    }

    return ASTRA_SIGNAL_OK;
}

astra_signal_result
astra_signal_default(
    int signal_number
)
{
    struct sigaction action;

    memset(&action, 0, sizeof(action));

    action.sa_handler = SIG_DFL;
    sigemptyset(&action.sa_mask);

    if (sigaction(signal_number, &action, NULL) != 0)
    {
        return ASTRA_SIGNAL_ERROR;
    }

    return ASTRA_SIGNAL_OK;
}

astra_signal_result
astra_signal_block(
    int signal_number
)
{
    sigset_t set;

    sigemptyset(&set);
    sigaddset(&set, signal_number);

    if (sigprocmask(SIG_BLOCK, &set, NULL) != 0)
    {
        return ASTRA_SIGNAL_ERROR;
    }

    return ASTRA_SIGNAL_OK;
}

astra_signal_result
astra_signal_unblock(
    int signal_number
)
{
    sigset_t set;

    sigemptyset(&set);
    sigaddset(&set, signal_number);

    if (sigprocmask(SIG_UNBLOCK, &set, NULL) != 0)
    {
        return ASTRA_SIGNAL_ERROR;
    }

    return ASTRA_SIGNAL_OK;
}

astra_signal_result
astra_signal_send(
    pid_t pid,
    int signal_number
)
{
    if (kill(pid, signal_number) != 0)
    {
        return ASTRA_SIGNAL_ERROR;
    }

    return ASTRA_SIGNAL_OK;
}
