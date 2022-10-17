let PI = 3.141592653589793;
let GPUI_QUAD_INPUT_INDEX_VERTICES = 0;
let GPUI_QUAD_INPUT_INDEX_QUADS = 1;
let GPUI_QUAD_INPUT_INDEX_UNIFORMS = 2;

fn colori_to_colorf(colori: vec4<u32>) -> vec4<f32> {
    return vec4<f32>(colori) / vec4<f32>(255.0, 255.0, 255.0, 255.0);
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
fn erf(x: vec2<f32>) -> vec2<f32> {
    var z = x;
    let s = sign(x);
    let a = abs(x);
    z = 1.0 + (0.278393 + (0.230389 + 0.078108 * (a * a)) * a) * a;
    z = z * z;
    return s - s / (z * z);
}

struct Uniforms {
    viewport_size: vec2<f32>,
} 

struct Quad {
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

@binding(0) @group(0) var<uniform> uniforms: Uniforms;
@binding(1) @group(0) var<storage> unit_vertices: array<vec2<f32>>;
@binding(2) @group(0) var<storage> quads: array<Quad>;

struct QuadFragmentInput {
    @builtin(position) position: vec4<f32>,
    @location(0) atlas_position: vec2<f32>, // only used in the image shader
    @location(1) origin: vec2<f32>,
    @location(2) size: vec2<f32>,
    @location(3) background_color: vec4<f32>,
    @location(4) border_top: f32,
    @location(5) border_right: f32,
    @location(6) border_bottom: f32,
    @location(7) border_left: f32,
    @location(8) border_color: vec4<f32>,
    @location(9) corner_radius: f32,
}

@vertex
fn quad_vertex(
    @builtin(vertex_index) unit_vertex_id: u32,
    @builtin(instance_index) quad_id: u32,
) -> QuadFragmentInput { 
    var unit_vertex = unit_vertices[unit_vertex_id];
    var quad = quads[quad_id];
    var position = unit_vertex * quad.size + quad.origin;
    var device_position = to_device_position(position, uniforms.viewport_size);

    var out: QuadFragmentInput;
    out.position = device_position;
    out.atlas_position = vec2<f32>(0.0, 0.0);
    out.origin = quad.origin;
    out.size = quad.size;
    out.background_color = quad.background_color;
    out.border_top = quad.border_top;
    out.border_right = quad.border_right;
    out.border_bottom = quad.border_bottom;
    out.border_left = quad.border_left;
    out.border_color = quad.border_color;
    out.corner_radius = quad.corner_radius;
    return out;
}

@fragment
fn quad_fragment(input: QuadFragmentInput) -> @location(0) vec4<f32> {
    let half_size = input.size / 2.0;
    let center = input.origin + half_size;
    let center_to_point = input.position.xy - center;
    let rounded_edge_to_point = abs(center_to_point) - half_size + input.corner_radius;
    let distance = length(max(vec2<f32>(0.0, 0.0), rounded_edge_to_point)) + 
        min(0.0, max(rounded_edge_to_point.x, rounded_edge_to_point.y)) - 
        input.corner_radius;
    
    var vertical_border: f32;
    if (center_to_point.x <= 0.0) {
        vertical_border = input.border_left;
    } else {
        vertical_border = input.border_right;
    }
    
    var horizontal_border: f32;
    if (center_to_point.x <= 0.0) {
        horizontal_border = input.border_left;
    } else {
        horizontal_border = input.border_right;
    }
    
    let inset_size = half_size - input.corner_radius - vec2<f32>(vertical_border, horizontal_border);
    let point_to_inset_corner = abs(center_to_point) - inset_size;

    var border_width: f32;
    if (point_to_inset_corner.x < 0.0 && point_to_inset_corner.y < 0.0) {
        border_width = 0.0;
    } else if (point_to_inset_corner.y > point_to_inset_corner.x) {
        border_width = horizontal_border;
    } else {
        border_width = vertical_border;
    }

    var color: vec4<f32>;
    if (border_width == 0.0) {
        color = input.background_color;
    } else {
        let inset_distance = distance + border_width;
        let border_color = vec4<f32>(
            mix(input.background_color.rgb, input.border_color.rgb, input.border_color.a),
            clamp(input.background_color.a + input.border_color.a, 0.0, 1.0)
        );
        color = mix(border_color, input.background_color, clamp(0.5 - inset_distance, 0.0, 1.0));
    }

    return color * vec4<f32>(1.0, 1.0, 1.0, clamp(0.5 - distance, 0.0, 1.0));
}