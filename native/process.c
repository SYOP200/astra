#include "astra_process.h"

#include <errno.h>
#include <signal.h>
#include <stdlib.h>
#include <string.h>
#include <sys/wait.h>
#include <unistd.h>

static void setup_stdio(
    astra_stdio_mode mode,
    int target_fd,
    int pipe_fd[2],
    bool is_input
)
{
    switch (mode)
    {
        case ASTRA_STDIO_PIPE:

            if (is_input)
            {
                dup2(pipe_fd[0], target_fd);
            }
            else
            {
                dup2(pipe_fd[1], target_fd);
            }

            break;

        case ASTRA_STDIO_NULL:
        {
            int fd = open("/dev/null", is_input ? O_RDONLY : O_WRONLY);

            if (fd >= 0)
            {
                dup2(fd, target_fd);
                close(fd);
            }

            break;
        }

        case ASTRA_STDIO_INHERIT:
        default:
            break;
    }
}

astra_process_result
astra_process_spawn(
    astra_process *process,
    const astra_process_options *options
)
{
    if (process == NULL || options == NULL || options->argv == NULL)
    {
        return ASTRA_PROCESS_ERROR;
    }

    memset(process, 0, sizeof(*process));

    int stdin_pipe[2] = {-1, -1};
    int stdout_pipe[2] = {-1, -1};
    int stderr_pipe[2] = {-1, -1};

    if (options->stdin_mode == ASTRA_STDIO_PIPE)
    {
        if (pipe(stdin_pipe) != 0)
            return ASTRA_PROCESS_ERROR;
    }

    if (options->stdout_mode == ASTRA_STDIO_PIPE)
    {
        if (pipe(stdout_pipe) != 0)
            return ASTRA_PROCESS_ERROR;
    }

    if (options->stderr_mode == ASTRA_STDIO_PIPE)
    {
        if (pipe(stderr_pipe) != 0)
            return ASTRA_PROCESS_ERROR;
    }

    pid_t pid = fork();

    if (pid < 0)
    {
        return ASTRA_PROCESS_ERROR;
    }

    if (pid == 0)
    {
        if (options->working_directory)
        {
            chdir(options->working_directory);
        }

        setup_stdio(
            options->stdin_mode,
            STDIN_FILENO,
            stdin_pipe,
            true
        );

        setup_stdio(
            options->stdout_mode,
            STDOUT_FILENO,
            stdout_pipe,
            false
        );

        setup_stdio(
            options->stderr_mode,
            STDERR_FILENO,
            stderr_pipe,
            false
        );

        if (options->environment)
        {
            execvpe(
                options->argv[0],
                options->argv,
                options->environment
            );
        }
        else
        {
            execvp(
                options->argv[0],
                options->argv
            );
        }

        _exit(127);
    }

    process->pid = pid;
    process->running = true;
    process->exit_code = 0;

    process->stdin_fd = stdin_pipe[1];
    process->stdout_fd = stdout_pipe[0];
    process->stderr_fd = stderr_pipe[0];

    if (stdin_pipe[0] != -1)
        close(stdin_pipe[0]);

    if (stdout_pipe[1] != -1)
        close(stdout_pipe[1]);

    if (stderr_pipe[1] != -1)
        close(stderr_pipe[1]);

    return ASTRA_PROCESS_OK;
}

astra_process_result
astra_process_wait(
    astra_process *process
)
{
    if (!process)
    {
        return ASTRA_PROCESS_ERROR;
    }

    int status;

    if (waitpid(process->pid, &status, 0) < 0)
    {
        return ASTRA_PROCESS_ERROR;
    }

    process->running = false;

    if (WIFEXITED(status))
    {
        process->exit_code = WEXITSTATUS(status);
    }
    else
    {
        process->exit_code = -1;
    }

    return ASTRA_PROCESS_OK;
}

astra_process_result
astra_process_kill(
    astra_process *process,
    int signal_number
)
{
    if (!process)
    {
        return ASTRA_PROCESS_ERROR;
    }

    if (kill(process->pid, signal_number) != 0)
    {
        return ASTRA_PROCESS_ERROR;
    }

    return ASTRA_PROCESS_OK;
}

astra_process_result
astra_process_pipe(
    astra_pipe *pipe_handle
)
{
    if (!pipe_handle)
    {
        return ASTRA_PROCESS_ERROR;
    }

    int fds[2];

    if (pipe(fds) != 0)
    {
        return ASTRA_PROCESS_ERROR;
    }

    pipe_handle->read_fd = fds[0];
    pipe_handle->write_fd = fds[1];

    return ASTRA_PROCESS_OK;
}

void
astra_process_close(
    int fd
)
{
    if (fd >= 0)
    {
        close(fd);
    }
}
