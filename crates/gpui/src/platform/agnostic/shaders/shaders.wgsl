let PI = 3.141592653589793;
let GPUI_QUAD_INPUT_INDEX_VERTICES = 0;
let GPUI_QUAD_INPUT_INDEX_QUADS = 1;
let GPUI_QUAD_INPUT_INDEX_UNIFORMS = 2;

fn colori_to_colorf(colori: vec4<u32>) -> vec4<f32> {
    return vec4<f32>(colori) / vec4<f32>(0xff, 0xff, 0xff, 0xff);
}

fn to_device_position(pixel_position: vec2<f32>, viewport_size: vec2<f32>) -> vec4<f32> {
    return vec4<f32>(
        pixel_position / viewport_size * vec2<f32>(2.0, -2.0) + vec2<f32>(-1.0, 1.0),
        0.0, 1.0);
}

// A standard gaussian function, used for weighting samples
fn gausian(x: f32, sigma: f32) -> f32 {
    return exp(-(x * x) / (2.0 * sigma * sigma)) / (sqrt(2.0 * PI) * sigma);
}

// This approximates the error function, needed for the gaussian integral
fn erf(x: vec<f32>) -> vec2<f32> {
    let s = sign(x);
    let a = abs(x);
    x = 1.0 + (0.278393 + (0.230389 + 0.078108 * (a * a)) * a) * a;
    x *= x;
    return s - s / (x * x);
}

struct QuadFragmentInput {
    [[builtin(position)]] position: vec4<f32>,
    atlas_position: vec2<f32>, // only used in the image shader
    origin: vec2<f32>,
    size: vec2<f32>,
    background_color: vec4<f32>,
    border_top: f32,
    border_right: f32,
    border_bottom: f32,
    border_left: f32,
    border_color: vec4<f32>,
    corner_radius: f32,
}

fn quad_sdf(QuadFragmentInput input) -> vec4<f32> {
    let half_size = input.size / 2.0;
    let center = input.origin + half_size;
    let center_to_point = input.position.xy - center;
    let rounded_edge_to_point = abs(center_to_point) - half_size + input.corner_radius;
    let distance = length(max(0.0, rounded_edge_to_point)) + min(0.0, max(rounded_edge_to_point.x, rounded_edge_to_point.y)) - input.corner_radius;

    let vertical_border = center_to_point.x <= 0.0 ? input.border_left : input.border_right;
    let horizontal_border = center_to_point.y <= 0.0 ? input.border_top : input.border_bottom;
    let inset_size = half_size - input.corner_radius - float2(vertical_border, horizontal_border);
    let point_to_inset_corner = abs(center_to_point) - inset_size;
    let border_width;
    if (point_to_inset_corner.x < 0.0 && point_to_inset_corner.y < 0.0) {
        border_width = 0.0;
    } else if (point_to_inset_corner.y > point_to_inset_corner.x) {
        border_width = horizontal_border;
    } else {
        border_width = vertical_border;
    }

    let color;
    if (border_width == 0.0) {
        color = input.background_color;
    } else {
        let inset_distance = distance + border_width;
        let border_color = vec4<f32>(
            mix(input.background_color.rgb, input.border_color.rgb, input.border_color.a),
            saturate(input.background_color.a + input.border_color.a)
        );
        color = mix(border_color, input.background_color, saturate(0.5 - inset_distance));
    }

    return color * vec4<f32>(1.0, 1.0, 1.0, saturate(0.5 - distance));
}
