#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Map3D {
  int64_t x;
  int64_t y;
  int64_t z;
  uint64_t sx;
  uint64_t sy;
  uint64_t sz;
  int32_t *a;
} Map3D;

/**
 * # Safety
 *
 * The input `err` must have been created using the `c_err` function, and it must have not been
 * modified in any way.
 */
void free_error_msg(char *err);

/**
 * # Safety
 *
 * The input pointers must be valid.
 */
char *read_seed_from_mc_world(const char *input_zip_path, const char *mc_version, int64_t *seed);

/**
 * # Safety
 *
 * The pointer `map.a` must be the same as when this `Map3D` was initialized. The contents of the
 * array `map.a` may have been modified. The values `map.sx`, `map.sy`, and `map.sz` must be the
 * same as when this `Map3D` was initialized.
 */
void free_map(struct Map3D map);

/**
 * # Safety
 *
 * The input pointers must be valid.
 */
char *read_biome_map_from_mc_world(const char *input_zip_path,
                                   const char *mc_version,
                                   struct Map3D *biome_map);

/**
 * # Safety
 *
 * The input pointers must be valid.
 */
char *draw_map3d_image_to_file(const struct Map3D *biome_map, const char *output_file_path);
