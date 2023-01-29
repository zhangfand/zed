#include <metal_stdlib>
#include "shaders.h"

using namespace metal;

float4 coloru_to_colorf(uchar4 coloru) {
    return float4(coloru) / float4(0xff, 0xff, 0xff, 0xff);
}

float deg_to_rad(float deg)
{
    return deg * (M_PI_F / 180);
}

float4x4 identity()
{
    float4 X = float4(1, 0, 0, 0);
    float4 Y = float4(0, 1, 0, 0);
    float4 Z = float4(0, 0, 1, 0);
    float4 W = float4(0, 0, 0, 1);
    float4x4 mat = float4x4(X, Y, Z, W);
    return mat;
}

float4x4 perspective_projection(float aspect, float fovy, float near, float far)
{
    float yScale = 1 / tan(fovy * 0.5);
    float xScale = yScale / aspect;
    float zRange = far - near;
    float zScale = -(far + near) / zRange;
    float wzScale = -far * near / zRange;
    
    float4 P = float4(xScale, 0, 0, 0);
    float4 Q = float4(0, yScale, 0, 0);
    float4 R = float4(0, 0, zScale, -1);
    float4 S = float4(0, 0, wzScale, 0);
    
    float4x4 mat = float4x4(P, Q, R, S);
    return mat;
}

float4x4 rotation(float3 axis, float angle)
{
    float c = cos(angle);
    float s = sin(angle);
    
    float4 X;
    X.x = axis.x * axis.x + (1 - axis.x * axis.x) * c;
    X.y = axis.x * axis.y * (1 - c) - axis.z*s;
    X.z = axis.x * axis.z * (1 - c) + axis.y * s;
    X.w = 0.0;
    
    float4 Y;
    Y.x = axis.x * axis.y * (1 - c) + axis.z * s;
    Y.y = axis.y * axis.y + (1 - axis.y * axis.y) * c;
    Y.z = axis.y * axis.z * (1 - c) - axis.x * s;
    Y.w = 0.0;
    
    float4 Z;
    Z.x = axis.x * axis.z * (1 - c) - axis.y * s;
    Z.y = axis.y * axis.z * (1 - c) + axis.x * s;
    Z.z = axis.z * axis.z + (1 - axis.z * axis.z) * c;
    Z.w = 0.0;
    
    float4 W;
    W.x = 0.0;
    W.y = 0.0;
    W.z = 0.0;
    W.w = 1.0;
    
    float4x4 mat = float4x4(X, Y, Z, W);
    return mat;
}

float4 to_device_position_3d(float2 pixel_position_, GPUIUniforms uniforms, float z) {
    float2 viewport_size = uniforms.viewport_size;
    float4 pixel_position = float4(pixel_position_ / viewport_size * float2(2., -2.) + float2(-1., 1.), z, 1.);
    
    float4x4 model_matrix = 
        rotation(float3(1., 0., 0.), deg_to_rad(uniforms.rotate_x)) * 
        rotation(float3(0., 1., 0.), deg_to_rad(uniforms.rotate_y)) * 
        rotation(float3(0., 0., 1.), deg_to_rad(uniforms.rotate_z));
    float4x4 view_matrix = identity();
    view_matrix.columns[3].z = uniforms.scale;
    
    float near = 0.1;
    float far = 100.;
    float aspect = 1.;
    float4x4 projection_matrix = perspective_projection(aspect, deg_to_rad(uniforms.fov), near, far);

    float4x4 model_view = view_matrix * model_matrix;
    float4x4 model_view_proj = projection_matrix * model_view;
    return model_view_proj * pixel_position;
}

// A standard gaussian function, used for weighting samples
float gaussian(float x, float sigma) {
    return exp(-(x * x) / (2. * sigma * sigma)) / (sqrt(2. * M_PI_F) * sigma);
}

// This approximates the error function, needed for the gaussian integral
float2 erf(float2 x) {
    float2 s = sign(x);
    float2 a = abs(x);
    x = 1. + (0.278393 + (0.230389 + 0.078108 * (a * a)) * a) * a;
    x *= x;
    return s - s / (x * x);
}

float blur_along_x(float x, float y, float sigma, float corner, float2 halfSize) {
    float delta = min(halfSize.y - corner - abs(y), 0.);
    float curved = halfSize.x - corner + sqrt(max(0., corner * corner - delta * delta));
    float2 integral = 0.5 + 0.5 * erf((x + float2(-curved, curved)) * (sqrt(0.5) / sigma));
    return integral.y - integral.x;
}

struct QuadFragmentInput {
    float4 position [[position]];
    float2 pixel_position;
    float2 atlas_position; // only used in the image shader
    float2 origin;
    float2 size;
    float4 background_color;
    float border_top;
    float border_right;
    float border_bottom;
    float border_left;
    float4 border_color;
    float corner_radius;
    float opacity;
};

float4 quad_sdf(QuadFragmentInput input) {
    float2 half_size = input.size / 2.;
    float2 center = input.origin + half_size;
    float2 center_to_point = input.pixel_position.xy - center;
    float2 rounded_edge_to_point = abs(center_to_point) - half_size + input.corner_radius;
    float distance = length(max(0., rounded_edge_to_point)) + min(0., max(rounded_edge_to_point.x, rounded_edge_to_point.y)) - input.corner_radius;

    float vertical_border = center_to_point.x <= 0. ? input.border_left : input.border_right;
    float horizontal_border = center_to_point.y <= 0. ? input.border_top : input.border_bottom;
    float2 inset_size = half_size - input.corner_radius - float2(vertical_border, horizontal_border);
    float2 point_to_inset_corner = abs(center_to_point) - inset_size;
    float border_width;
    if (point_to_inset_corner.x < 0. && point_to_inset_corner.y < 0.) {
        border_width = 0.;
    } else if (point_to_inset_corner.y > point_to_inset_corner.x) {
        border_width = horizontal_border;
    } else {
        border_width = vertical_border;
    }

    float4 color;
    if (border_width == 0.) {
        color = input.background_color;
    } else {
        float inset_distance = distance + border_width;

        // Decrease border's opacity as we move inside the background.
        input.border_color.a *= 1. - saturate(0.5 - inset_distance);

        // Alpha-blend the border and the background.
        float output_alpha = input.border_color.a + input.background_color.a * (1. - input.border_color.a);
        float3 premultiplied_border_rgb = input.border_color.rgb * input.border_color.a;
        float3 premultiplied_background_rgb = input.background_color.rgb * input.background_color.a;
        float3 premultiplied_output_rgb = premultiplied_border_rgb + premultiplied_background_rgb * (1. - input.border_color.a);
        color = float4(premultiplied_output_rgb / output_alpha, output_alpha);
    }

    return color * float4(1., 1., 1., saturate(0.5 - distance) * input.opacity);
}

vertex QuadFragmentInput quad_vertex(
    uint unit_vertex_id [[vertex_id]],
    uint quad_id [[instance_id]],
    constant float2 *unit_vertices [[buffer(GPUIQuadInputIndexVertices)]],
    constant GPUIQuad *quads [[buffer(GPUIQuadInputIndexQuads)]],
    constant GPUIUniforms *uniforms [[buffer(GPUIQuadInputIndexUniforms)]]
) {
    float2 unit_vertex = unit_vertices[unit_vertex_id];
    GPUIQuad quad = quads[quad_id];
    float2 position = unit_vertex * quad.size + quad.origin;
    float4 device_position = to_device_position_3d(position, *uniforms, quad.z);

    return QuadFragmentInput {
        device_position,
        position,
        float2(0., 0.),
        quad.origin,
        quad.size,
        coloru_to_colorf(quad.background_color),
        quad.border_top,
        quad.border_right,
        quad.border_bottom,
        quad.border_left,
        coloru_to_colorf(quad.border_color),
        quad.corner_radius,
        uniforms->opacity
    };
}

fragment float4 quad_fragment(
    QuadFragmentInput input [[stage_in]]
) {
    return quad_sdf(input);
}

struct SpriteFragmentInput {
    float4 position [[position]];
    float2 atlas_position;
    float4 color [[flat]];
    uchar compute_winding [[flat]];
    float opacity;
};

vertex SpriteFragmentInput sprite_vertex(
    uint unit_vertex_id [[vertex_id]],
    uint sprite_id [[instance_id]],
    constant float2 *unit_vertices [[buffer(GPUISpriteVertexInputIndexVertices)]],
    constant GPUISprite *sprites [[buffer(GPUISpriteVertexInputIndexSprites)]],
    constant GPUIUniforms *uniforms [[buffer(GPUISpriteVertexInputIndexUniforms)]],
    constant float2 *atlas_size [[buffer(GPUISpriteVertexInputIndexAtlasSize)]]
) {
    float2 unit_vertex = unit_vertices[unit_vertex_id];
    GPUISprite sprite = sprites[sprite_id];
    float2 position = unit_vertex * sprite.target_size + sprite.origin;
    float4 device_position = to_device_position_3d(position, *uniforms, sprite.z);
    float2 atlas_position = (unit_vertex * sprite.source_size + sprite.atlas_origin) / *atlas_size;

    return SpriteFragmentInput {
        device_position,
        atlas_position,
        coloru_to_colorf(sprite.color),
        sprite.compute_winding,
        uniforms->opacity
    };
}

fragment float4 sprite_fragment(
    SpriteFragmentInput input [[stage_in]],
    texture2d<float> atlas [[ texture(GPUISpriteFragmentInputIndexAtlas) ]]
) {
    constexpr sampler atlas_sampler(mag_filter::linear, min_filter::linear);
    float4 color = input.color;
    float4 sample = atlas.sample(atlas_sampler, input.atlas_position);
    float mask;
    if (input.compute_winding) {
        mask = 1. - abs(1. - fmod(sample.r, 2.));
    } else {
        mask = sample.a;
    }
    color.a *= mask * input.opacity;
    return color;
}
