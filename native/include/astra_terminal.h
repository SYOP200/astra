#ifndef ASTRA_TERMINAL_H
#define ASTRA_TERMINAL_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <termios.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum
{
    ASTRA_TERM_OK = 0,
    ASTRA_TERM_ERROR = -1
} astra_term_result;

typedef struct
{
    struct termios original;
    struct termios raw;

    bool raw_enabled;

    int rows;
    int cols;

} astra_terminal;

astra_term_result astra_terminal_init(astra_terminal *terminal);

astra_term_result astra_terminal_enable_raw(astra_terminal *terminal);

astra_term_result astra_terminal_disable_raw(astra_terminal *terminal);

astra_term_result astra_terminal_update_size(astra_terminal *terminal);

void astra_terminal_clear(void);

void astra_terminal_move_cursor(int row, int column);

void astra_terminal_hide_cursor(void);

void astra_terminal_show_cursor(void);

void astra_terminal_flush(void);

#ifdef __cplusplus
}
#endif

#endif
