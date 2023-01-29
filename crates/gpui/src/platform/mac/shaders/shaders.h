#include <simd/simd.h>

typedef struct {
  vector_float2 viewport_size;
  float scale;
  float rotate_x;
  float rotate_y;
  float rotate_z;
  float fov;
  float opacity;
} GPUIUniforms;

typedef enum {
  GPUIQuadInputIndexVertices = 0,
  GPUIQuadInputIndexQuads = 1,
  GPUIQuadInputIndexUniforms = 2,
} GPUIQuadInputIndex;

typedef struct {
  vector_float2 origin;
  vector_float2 size;
  vector_uchar4 background_color;
  float border_top;
  float border_right;
  float border_bottom;
  float border_left;
  vector_uchar4 border_color;
  float corner_radius;
  float z;
} GPUIQuad;

typedef enum {
  GPUISpriteVertexInputIndexVertices = 0,
  GPUISpriteVertexInputIndexSprites = 1,
  GPUISpriteVertexInputIndexUniforms = 2,
  GPUISpriteVertexInputIndexAtlasSize = 3,
} GPUISpriteVertexInputIndex;

typedef enum {
  GPUISpriteFragmentInputIndexAtlas = 0,
} GPUISpriteFragmentInputIndex;

typedef struct {
  vector_float2 origin;
  vector_float2 target_size;
  vector_float2 source_size;
  vector_float2 atlas_origin;
  vector_uchar4 color;
  uint8_t compute_winding;
  float z;
} GPUISprite;
