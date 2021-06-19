#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Map {
  int64_t x;
  int64_t z;
  uint64_t w;
  uint64_t h;
  int32_t *a;
} Map;

/**
 * # Safety
 *
 * The input `err` must have been created using the `c_err` function, and it must have not been
 * modified in any way.
 */
void free_error_msg(char *err);

char *read_seed_from_mc_world(const char *input_zip_path, const char *mc_version, int64_t *seed);

/**
 * # Safety
 *
 * The pointer `map.a` must be the same as when this `Map` was initialized, but it may have been
 * modified. The values `map.w` and `map.h` must be the same as when this `Map` was initialized.
 */
void free_map(struct Map map);

char *read_biome_map_from_mc_world(const char *input_zip_path,
                                   const char *mc_version,
                                   struct Map *biome_map);
