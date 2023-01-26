#include <metal_stdlib>
#include "shaders.h"

using namespace metal;

float4 coloru_to_colorf(uchar4 coloru) {
    return float4(coloru) / float4(0xff, 0xff, 0xff, 0xff);
}


float deg_to_rad(float deg)
{
    return deg * (3.14 / 180);
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

float4 to_device_position(float2 pixel_position_, float2 viewport_size) {
    float4 pixel_position = float4(pixel_position_ / viewport_size * float2(2., -2.) + float2(-1., 1.), 0., 1.);
    
    float4x4 model_matrix = rotation(float3(0., 1., 0.), -deg_to_rad(30.));
    float4x4 view_matrix = identity();
    view_matrix.columns[3].z = -2;
    
    float near = 0.1;
    float far = 100.;
    float aspect = viewport_size.x / viewport_size.y;
    float4x4 projection_matrix = perspective_projection(aspect, deg_to_rad(75.), near, far);

    float4x4 model_view = view_matrix * model_matrix;
    float4x4 model_view_proj = projection_matrix * model_view;
    return model_view_proj * pixel_position;
}

float4 to_device_position_3d(float2 pixel_position_, float2 viewport_size, float z) {
    float4 pixel_position = float4(pixel_position_ / viewport_size * float2(2., -2.) + float2(-1., 1.), z, 1.);
    
    float4x4 model_matrix = rotation(float3(1., 0., 0.), deg_to_rad(50.)) * rotation(float3(0., 1., 0.), -deg_to_rad(30.)) * rotation(float3(0., 0., 1.), -deg_to_rad(80.));
    float4x4 view_matrix = identity();
    view_matrix.columns[3].z = -2.5;
    
    float near = 0.1;
    float far = 100.;
    float aspect = viewport_size.x / viewport_size.y;
    float4x4 projection_matrix = perspective_projection(aspect, deg_to_rad(75.), near, far);

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
    uchar grayscale; // only used in image shader
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

    return color * float4(1., 1., 1., saturate(0.5 - distance));
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
    float4 device_position = to_device_position_3d(position, uniforms->viewport_size, quad.z);

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
        0,
    };
}

fragment float4 quad_fragment(
    QuadFragmentInput input [[stage_in]]
) {
    return quad_sdf(input);
}

struct ShadowFragmentInput {
    float4 position [[position]];
    vector_float2 origin;
    vector_float2 size;
    float corner_radius;
    float sigma;
    vector_uchar4 color;
};

vertex ShadowFragmentInput shadow_vertex(
    uint unit_vertex_id [[vertex_id]],
    uint shadow_id [[instance_id]],
    constant float2 *unit_vertices [[buffer(GPUIShadowInputIndexVertices)]],
    constant GPUIShadow *shadows [[buffer(GPUIShadowInputIndexShadows)]],
    constant GPUIUniforms *uniforms [[buffer(GPUIShadowInputIndexUniforms)]]
) {
    float2 unit_vertex = unit_vertices[unit_vertex_id];
    GPUIShadow shadow = shadows[shadow_id];

    float margin = 3. * shadow.sigma;
    float2 position = unit_vertex * (shadow.size + 2. * margin) + shadow.origin - margin;
    float4 device_position = to_device_position(position, uniforms->viewport_size);

    return ShadowFragmentInput {
        device_position,
        shadow.origin,
        shadow.size,
        shadow.corner_radius,
        shadow.sigma,
        shadow.color,
    };
}

fragment float4 shadow_fragment(
    ShadowFragmentInput input [[stage_in]]
) {
    float sigma = input.sigma;
    float corner_radius = input.corner_radius;
    float2 half_size = input.size / 2.;
    float2 center = input.origin + half_size;
    float2 point = input.position.xy - center;

    // The signal is only non-zero in a limited range, so don't waste samples
    float low = point.y - half_size.y;
    float high = point.y + half_size.y;
    float start = clamp(-3. * sigma, low, high);
    float end = clamp(3. * sigma, low, high);

    // Accumulate samples (we can get away with surprisingly few samples)
    float step = (end - start) / 4.;
    float y = start + step * 0.5;
    float alpha = 0.;
    for (int i = 0; i < 4; i++) {
        alpha += blur_along_x(point.x, point.y - y, sigma, corner_radius, half_size) * gaussian(y, sigma) * step;
        y += step;
    }

    return float4(1., 1., 1., alpha) * coloru_to_colorf(input.color);
}

struct SpriteFragmentInput {
    float4 position [[position]];
    float2 atlas_position;
    float4 color [[flat]];
    uchar compute_winding [[flat]];
};

vertex SpriteFragmentInput sprite_vertex(
    uint unit_vertex_id [[vertex_id]],
    uint sprite_id [[instance_id]],
    constant float2 *unit_vertices [[buffer(GPUISpriteVertexInputIndexVertices)]],
    constant GPUISprite *sprites [[buffer(GPUISpriteVertexInputIndexSprites)]],
    constant float2 *viewport_size [[buffer(GPUISpriteVertexInputIndexViewportSize)]],
    constant float2 *atlas_size [[buffer(GPUISpriteVertexInputIndexAtlasSize)]]
) {
    float2 unit_vertex = unit_vertices[unit_vertex_id];
    GPUISprite sprite = sprites[sprite_id];
    float2 position = unit_vertex * sprite.target_size + sprite.origin;
    float4 device_position = to_device_position_3d(position, *viewport_size, sprite.z);
    float2 atlas_position = (unit_vertex * sprite.source_size + sprite.atlas_origin) / *atlas_size;

    return SpriteFragmentInput {
        device_position,
        atlas_position,
        coloru_to_colorf(sprite.color),
        sprite.compute_winding
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
    color.a *= mask;
    return color;
}

vertex QuadFragmentInput image_vertex(
    uint unit_vertex_id [[vertex_id]],
    uint image_id [[instance_id]],
    constant float2 *unit_vertices [[buffer(GPUIImageVertexInputIndexVertices)]],
    constant GPUIImage *images [[buffer(GPUIImageVertexInputIndexImages)]],
    constant float2 *viewport_size [[buffer(GPUIImageVertexInputIndexViewportSize)]],
    constant float2 *atlas_size [[buffer(GPUIImageVertexInputIndexAtlasSize)]]
) {
    float2 unit_vertex = unit_vertices[unit_vertex_id];
    GPUIImage image = images[image_id];
    float2 position = unit_vertex * image.target_size + image.origin;
    float4 device_position = to_device_position(position, *viewport_size);
    float2 atlas_position = (unit_vertex * image.source_size + image.atlas_origin) / *atlas_size;

    return QuadFragmentInput {
        device_position,
        position,
        atlas_position,
        image.origin,
        image.target_size,
        float4(0.),
        image.border_top,
        image.border_right,
        image.border_bottom,
        image.border_left,
        coloru_to_colorf(image.border_color),
        image.corner_radius,
        image.grayscale,
    };
}

fragment float4 image_fragment(
    QuadFragmentInput input [[stage_in]],
    texture2d<float> atlas [[ texture(GPUIImageFragmentInputIndexAtlas) ]]
) {
    constexpr sampler atlas_sampler(mag_filter::linear, min_filter::linear);
    input.background_color = atlas.sample(atlas_sampler, input.atlas_position);
    if (input.grayscale) {
        float grayscale =
            0.2126 * input.background_color.r +
            0.7152 * input.background_color.g + 
            0.0722 * input.background_color.b;
        input.background_color = float4(grayscale, grayscale, grayscale, input.background_color.a);
    }
    return quad_sdf(input);
}

vertex QuadFragmentInput surface_vertex(
    uint unit_vertex_id [[vertex_id]],
    uint image_id [[instance_id]],
    constant float2 *unit_vertices [[buffer(GPUISurfaceVertexInputIndexVertices)]],
    constant GPUISurface *images [[buffer(GPUISurfaceVertexInputIndexSurfaces)]],
    constant float2 *viewport_size [[buffer(GPUISurfaceVertexInputIndexViewportSize)]],
    constant float2 *atlas_size [[buffer(GPUISurfaceVertexInputIndexAtlasSize)]]
) {
    float2 unit_vertex = unit_vertices[unit_vertex_id];
    GPUISurface image = images[image_id];
    float2 position = unit_vertex * image.target_size + image.origin;
    float4 device_position = to_device_position(position, *viewport_size);
    float2 atlas_position = (unit_vertex * image.source_size) / *atlas_size;

    return QuadFragmentInput {
        device_position,
        position,
        atlas_position,
        image.origin,
        image.target_size,
        float4(0.),
        0.,
        0.,
        0.,
        0.,
        float4(0.),
        0.,
        0,
    };
}

fragment float4 surface_fragment(
    QuadFragmentInput input [[stage_in]],
    texture2d<float> y_atlas [[ texture(GPUISurfaceFragmentInputIndexYAtlas) ]],
    texture2d<float> cb_cr_atlas [[ texture(GPUISurfaceFragmentInputIndexCbCrAtlas) ]]
) {
    constexpr sampler atlas_sampler(mag_filter::linear, min_filter::linear);
    const float4x4 ycbcrToRGBTransform = float4x4(
        float4(+1.0000f, +1.0000f, +1.0000f, +0.0000f),
        float4(+0.0000f, -0.3441f, +1.7720f, +0.0000f),
        float4(+1.4020f, -0.7141f, +0.0000f, +0.0000f),
        float4(-0.7010f, +0.5291f, -0.8860f, +1.0000f)
    );
    float4 ycbcr = float4(y_atlas.sample(atlas_sampler, input.atlas_position).r,
                          cb_cr_atlas.sample(atlas_sampler, input.atlas_position).rg, 1.0);

    input.background_color = ycbcrToRGBTransform * ycbcr;
    return quad_sdf(input);
}

struct PathAtlasVertexOutput {
    float4 position [[position]];
    float2 st_position;
    float clip_rect_distance [[clip_distance]] [4];
};

struct PathAtlasFragmentInput {
    float4 position [[position]];
    float2 st_position;
};

vertex PathAtlasVertexOutput path_atlas_vertex(
    uint vertex_id [[vertex_id]],
    constant GPUIPathVertex *vertices [[buffer(GPUIPathAtlasVertexInputIndexVertices)]],
    constant float2 *atlas_size [[buffer(GPUIPathAtlasVertexInputIndexAtlasSize)]]
) {
    GPUIPathVertex v = vertices[vertex_id];
    float4 device_position = to_device_position(v.xy_position, *atlas_size);
    return PathAtlasVertexOutput {
        device_position,
        v.st_position,
        {
            v.xy_position.x - v.clip_rect_origin.x,
            v.clip_rect_origin.x + v.clip_rect_size.x - v.xy_position.x,
            v.xy_position.y - v.clip_rect_origin.y,
            v.clip_rect_origin.y + v.clip_rect_size.y - v.xy_position.y
        }
    };
}

fragment float4 path_atlas_fragment(
    PathAtlasFragmentInput input [[stage_in]]
) {
    float2 dx = dfdx(input.st_position);
    float2 dy = dfdy(input.st_position);
    float2 gradient = float2(
        (2. * input.st_position.x) * dx.x - dx.y,
        (2. * input.st_position.x) * dy.x - dy.y
    );
    float f = (input.st_position.x * input.st_position.x) - input.st_position.y;
    float distance = f / length(gradient);
    float alpha = saturate(0.5 - distance);
    return float4(alpha, 0., 0., 1.);
}

struct UnderlineFragmentInput {
    float4 position [[position]];
    float2 origin;
    float2 size;
    float thickness;
    float4 color;
    bool squiggly;
};

vertex UnderlineFragmentInput underline_vertex(
    uint unit_vertex_id [[vertex_id]],
    uint underline_id [[instance_id]],
    constant float2 *unit_vertices [[buffer(GPUIUnderlineInputIndexVertices)]],
    constant GPUIUnderline *underlines [[buffer(GPUIUnderlineInputIndexUnderlines)]],
    constant GPUIUniforms *uniforms [[buffer(GPUIUnderlineInputIndexUniforms)]]
) {
    float2 unit_vertex = unit_vertices[unit_vertex_id];
    GPUIUnderline underline = underlines[underline_id];
    float2 position = unit_vertex * underline.size + underline.origin;
    float4 device_position = to_device_position(position, uniforms->viewport_size);

    return UnderlineFragmentInput {
        device_position,
        underline.origin,
        underline.size,
        underline.thickness,
        coloru_to_colorf(underline.color),
        underline.squiggly != 0,
    };
}

fragment float4 underline_fragment(
    UnderlineFragmentInput input [[stage_in]]
) {
    if (input.squiggly) {
        float half_thickness = input.thickness * 0.5;
        float2 st = ((input.position.xy - input.origin) / input.size.y) - float2(0., 0.5);
        float frequency = (M_PI_F * (3. * input.thickness)) / 8.;
        float amplitude = 1. / (2. * input.thickness);
        float sine = sin(st.x * frequency) * amplitude;
        float dSine = cos(st.x * frequency) * amplitude * frequency;
        float distance = (st.y - sine) / sqrt(1. + dSine * dSine);
        float distance_in_pixels = distance * input.size.y;
        float distance_from_top_border = distance_in_pixels - half_thickness;
        float distance_from_bottom_border = distance_in_pixels + half_thickness;
        float alpha = saturate(0.5 - max(-distance_from_bottom_border, distance_from_top_border));
        return input.color * float4(1., 1., 1., alpha);
    } else {
        return input.color;
    }
}
