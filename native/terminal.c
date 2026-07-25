#include "astra_terminal.h"

#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <sys/ioctl.h>

static astra_term_result astra_terminal_apply(
    const struct termios *settings
)
{
    if (tcsetattr(STDIN_FILENO, TCSAFLUSH, settings) == -1)
    {
        return ASTRA_TERM_ERROR;
    }

    return ASTRA_TERM_OK;
}

astra_term_result astra_terminal_init(
    astra_terminal *terminal
)
{
    if (terminal == NULL)
    {
        return ASTRA_TERM_ERROR;
    }

    memset(terminal, 0, sizeof(*terminal));

    if (tcgetattr(STDIN_FILENO, &terminal->original) == -1)
    {
        return ASTRA_TERM_ERROR;
    }

    terminal->raw = terminal->original;
    terminal->raw_enabled = false;

    return astra_terminal_update_size(terminal);
}

astra_term_result astra_terminal_enable_raw(
    astra_terminal *terminal
)
{
    if (terminal == NULL)
    {
        return ASTRA_TERM_ERROR;
    }

    terminal->raw = terminal->original;

    terminal->raw.c_iflag &=
        ~(BRKINT |
          ICRNL |
          INPCK |
          ISTRIP |
          IXON);

    terminal->raw.c_oflag &= ~(OPOST);

    terminal->raw.c_cflag |= (CS8);

    terminal->raw.c_lflag &=
        ~(ECHO |
          ICANON |
          IEXTEN |
          ISIG);

    terminal->raw.c_cc[VMIN] = 1;
    terminal->raw.c_cc[VTIME] = 0;

    if (astra_terminal_apply(&terminal->raw) != ASTRA_TERM_OK)
    {
        return ASTRA_TERM_ERROR;
    }

    terminal->raw_enabled = true;

    return ASTRA_TERM_OK;
}

astra_term_result astra_terminal_disable_raw(
    astra_terminal *terminal
)
{
    if (terminal == NULL)
    {
        return ASTRA_TERM_ERROR;
    }

    if (!terminal->raw_enabled)
    {
        return ASTRA_TERM_OK;
    }

    if (astra_terminal_apply(&terminal->original) != ASTRA_TERM_OK)
    {
        return ASTRA_TERM_ERROR;
    }

    terminal->raw_enabled = false;

    return ASTRA_TERM_OK;
}

astra_term_result astra_terminal_update_size(
    astra_terminal *terminal
)
{
    if (terminal == NULL)
    {
        return ASTRA_TERM_ERROR;
    }

    struct winsize ws;

    if (ioctl(STDOUT_FILENO, TIOCGWINSZ, &ws) == -1)
    {
        return ASTRA_TERM_ERROR;
    }

    terminal->rows = ws.ws_row;
    terminal->cols = ws.ws_col;

    return ASTRA_TERM_OK;
}

void astra_terminal_clear(void)
{
    fputs("\x1b[2J\x1b[H", stdout);
}

void astra_terminal_move_cursor(
    int row,
    int column
)
{
    fprintf(
        stdout,
        "\x1b[%d;%dH",
        row,
        column
    );
}

void astra_terminal_hide_cursor(void)
{
    fputs("\x1b[?25l", stdout);
}

void astra_terminal_show_cursor(void)
{
    fputs("\x1b[?25h", stdout);
}

void astra_terminal_flush(void)
{
    fflush(stdout);
}
