#ifndef ASTRA_ENV_H
#define ASTRA_ENV_H

#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum
{
    ASTRA_ENV_OK = 0,
    ASTRA_ENV_ERROR = -1
} astra_env_result;

typedef struct
{
    char *key;
    char *value;
} astra_env_pair;

const char *
astra_env_get(
    const char *key
);

astra_env_result
astra_env_set(
    const char *key,
    const char *value,
    bool overwrite
);

astra_env_result
astra_env_unset(
    const char *key
);

char *
astra_env_current_directory(
    char *buffer,
    size_t size
);

astra_env_result
astra_env_change_directory(
    const char *path
);

const char *
astra_env_home(void);

const char *
astra_env_shell(void);

const char *
astra_env_path(void);

char *
astra_env_find_executable(
    const char *program,
    char *buffer,
    size_t size
);

bool
astra_env_exists(
    const char *key
);

#ifdef __cplusplus
}
#endif

#endif
