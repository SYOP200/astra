#ifndef ASTRA_PROCESS_H
#define ASTRA_PROCESS_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <sys/types.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum
{
    ASTRA_PROCESS_OK = 0,
    ASTRA_PROCESS_ERROR = -1
} astra_process_result;

typedef enum
{
    ASTRA_STDIO_INHERIT = 0,
    ASTRA_STDIO_PIPE,
    ASTRA_STDIO_NULL
} astra_stdio_mode;

typedef struct
{
    int read_fd;
    int write_fd;
} astra_pipe;

typedef struct
{
    pid_t pid;

    int stdin_fd;
    int stdout_fd;
    int stderr_fd;

    bool running;
    int exit_code;

} astra_process;

typedef struct
{
    char **argv;

    const char *working_directory;

    char **environment;

    astra_stdio_mode stdin_mode;
    astra_stdio_mode stdout_mode;
    astra_stdio_mode stderr_mode;

} astra_process_options;

astra_process_result
astra_process_spawn(
    astra_process *process,
    const astra_process_options *options
);

astra_process_result
astra_process_wait(
    astra_process *process
);

astra_process_result
astra_process_kill(
    astra_process *process,
    int signal_number
);

astra_process_result
astra_process_pipe(
    astra_pipe *pipe_handle
);

void
astra_process_close(
    int fd
);

#ifdef __cplusplus
}
#endif

#endif
