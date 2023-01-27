use super::{atlas::AtlasAllocator, image_cache::ImageCache, sprite_cache::SpriteCache};
use crate::{
    color::Color,
    geometry::{
        rect::RectF,
        vector::{vec2f, vec2i, Vector2F},
    },
    platform,
    scene::{Glyph, Icon, Quad, Scene},
};
use cocoa::{
    base::{NO, YES},
    foundation::NSUInteger,
    quartzcore::AutoresizingMask,
};
use foreign_types::ForeignTypeRef;
use media::core_video::{self, CVMetalTextureCache};
use metal::{CommandQueue, MTLPixelFormat, MTLResourceOptions, NSRange};
use objc::{self, msg_send, sel, sel_impl};
use shaders::ToFloat2 as _;
use std::{collections::HashMap, ffi::c_void, mem, sync::Arc};

const SHADERS_METALLIB: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/shaders.metallib"));
const INSTANCE_BUFFER_SIZE: usize = 8192 * 1024; // This is an arbitrary decision. There's probably a more optimal value.

pub struct Renderer {
    layer: metal::MetalLayer,
    command_queue: CommandQueue,
    sprite_cache: SpriteCache,
    image_cache: ImageCache,
    path_atlases: AtlasAllocator,
    quad_pipeline_state: metal::RenderPipelineState,
    shadow_pipeline_state: metal::RenderPipelineState,
    sprite_pipeline_state: metal::RenderPipelineState,
    image_pipeline_state: metal::RenderPipelineState,
    surface_pipeline_state: metal::RenderPipelineState,
    path_atlas_pipeline_state: metal::RenderPipelineState,
    underline_pipeline_state: metal::RenderPipelineState,
    unit_vertices: metal::Buffer,
    instances: metal::Buffer,
    cv_texture_cache: core_video::CVMetalTextureCache,
}

struct PathSprite {
    layer_id: usize,
    atlas_id: usize,
    shader_data: shaders::GPUISprite,
}

pub struct Surface {
    pub bounds: RectF,
    pub image_buffer: core_video::CVImageBuffer,
}

impl Renderer {
    pub fn new(is_opaque: bool, fonts: Arc<dyn platform::FontSystem>) -> Self {
        const PIXEL_FORMAT: MTLPixelFormat = MTLPixelFormat::BGRA8Unorm;

        let device: metal::Device = if let Some(device) = metal::Device::system_default() {
            device
        } else {
            log::error!("unable to access a compatible graphics device");
            std::process::exit(1);
        };

        let layer = metal::MetalLayer::new();
        layer.set_device(&device);
        layer.set_pixel_format(PIXEL_FORMAT);
        layer.set_presents_with_transaction(true);
        layer.set_opaque(is_opaque);
        unsafe {
            let _: () = msg_send![&*layer, setAllowsNextDrawableTimeout: NO];
            let _: () = msg_send![&*layer, setNeedsDisplayOnBoundsChange: YES];
            let _: () = msg_send![
                &*layer,
                setAutoresizingMask: AutoresizingMask::WIDTH_SIZABLE
                    | AutoresizingMask::HEIGHT_SIZABLE
            ];
        }

        let library = device
            .new_library_with_data(SHADERS_METALLIB)
            .expect("error building metal library");

        let unit_vertices = [
            (0., 0.).to_float2(),
            (1., 0.).to_float2(),
            (0., 1.).to_float2(),
            (0., 1.).to_float2(),
            (1., 0.).to_float2(),
            (1., 1.).to_float2(),
        ];
        let unit_vertices = device.new_buffer_with_data(
            unit_vertices.as_ptr() as *const c_void,
            (unit_vertices.len() * mem::size_of::<shaders::vector_float2>()) as u64,
            MTLResourceOptions::StorageModeManaged,
        );
        let instances = device.new_buffer(
            INSTANCE_BUFFER_SIZE as u64,
            MTLResourceOptions::StorageModeManaged,
        );

        let sprite_cache = SpriteCache::new(device.clone(), vec2i(1024, 768), 1., fonts.clone());
        let image_cache = ImageCache::new(device.clone(), vec2i(1024, 768), 1., fonts);
        let path_atlases =
            AtlasAllocator::new(device.clone(), build_path_atlas_texture_descriptor());
        let quad_pipeline_state = build_pipeline_state(
            &device,
            &library,
            "quad",
            "quad_vertex",
            "quad_fragment",
            PIXEL_FORMAT,
        );
        let shadow_pipeline_state = build_pipeline_state(
            &device,
            &library,
            "shadow",
            "shadow_vertex",
            "shadow_fragment",
            PIXEL_FORMAT,
        );
        let sprite_pipeline_state = build_pipeline_state(
            &device,
            &library,
            "sprite",
            "sprite_vertex",
            "sprite_fragment",
            PIXEL_FORMAT,
        );
        let image_pipeline_state = build_pipeline_state(
            &device,
            &library,
            "image",
            "image_vertex",
            "image_fragment",
            PIXEL_FORMAT,
        );
        let surface_pipeline_state = build_pipeline_state(
            &device,
            &library,
            "surface",
            "surface_vertex",
            "surface_fragment",
            PIXEL_FORMAT,
        );
        let path_atlas_pipeline_state = build_path_atlas_pipeline_state(
            &device,
            &library,
            "path_atlas",
            "path_atlas_vertex",
            "path_atlas_fragment",
            MTLPixelFormat::R16Float,
        );
        let underline_pipeline_state = build_pipeline_state(
            &device,
            &library,
            "underline",
            "underline_vertex",
            "underline_fragment",
            PIXEL_FORMAT,
        );
        let cv_texture_cache = CVMetalTextureCache::new(device.as_ptr()).unwrap();
        Self {
            layer,
            command_queue: device.new_command_queue(),
            sprite_cache,
            image_cache,
            path_atlases,
            quad_pipeline_state,
            shadow_pipeline_state,
            sprite_pipeline_state,
            image_pipeline_state,
            surface_pipeline_state,
            path_atlas_pipeline_state,
            underline_pipeline_state,
            unit_vertices,
            instances,
            cv_texture_cache,
        }
    }

    pub fn layer(&self) -> &metal::MetalLayerRef {
        &*self.layer
    }

    pub fn render(&mut self, scene: &Scene) {
        let layer = self.layer.clone();
        let drawable_size = layer.drawable_size();
        let drawable = if let Some(drawable) = layer.next_drawable() {
            drawable
        } else {
            log::error!(
                "failed to retrieve next drawable, drawable size: {:?}",
                drawable_size
            );
            return;
        };
        let command_queue = self.command_queue.clone();
        let command_buffer = command_queue.new_command_buffer();

        self.sprite_cache.set_scale_factor(scene.scale_factor());
        self.image_cache.set_scale_factor(scene.scale_factor());

        let mut offset = 0;

        self.render_layers(
            scene,
            &mut offset,
            vec2f(drawable_size.width as f32, drawable_size.height as f32),
            command_buffer,
            drawable.texture(),
        );
        self.instances.did_modify_range(NSRange {
            location: 0,
            length: offset as NSUInteger,
        });
        self.image_cache.finish_frame();

        command_buffer.commit();
        command_buffer.wait_until_completed();
        drawable.present();
    }

    fn render_layers(
        &mut self,
        scene: &Scene,
        offset: &mut usize,
        drawable_size: Vector2F,
        command_buffer: &metal::CommandBufferRef,
        output: &metal::TextureRef,
    ) {
        let render_pass_descriptor = metal::RenderPassDescriptor::new();
        let color_attachment = render_pass_descriptor
            .color_attachments()
            .object_at(0)
            .unwrap();
        color_attachment.set_texture(Some(output));
        color_attachment.set_load_action(metal::MTLLoadAction::Clear);
        color_attachment.set_store_action(metal::MTLStoreAction::Store);
        let alpha = if self.layer.is_opaque() { 1. } else { 0. };
        color_attachment.set_clear_color(metal::MTLClearColor::new(0.7, 0.7, 0.7, alpha));
        let command_encoder = command_buffer.new_render_command_encoder(render_pass_descriptor);

        command_encoder.set_viewport(metal::MTLViewport {
            originX: 0.0,
            originY: 0.0,
            width: drawable_size.x() as f64,
            height: drawable_size.y() as f64,
            znear: 0.0,
            zfar: 1.0,
        });

        let scale_factor = scene.scale_factor();
        for (layer_id, layer) in scene.layers().enumerate() {
            let z_id = layer_id * 3;
            let z_quads = z_id as f32 * scene.layer_z_factor();
            let z_sprites = (z_id + 2) as f32 * scene.layer_z_factor();
            self.render_quads(
                z_quads,
                scene.scale,
                scene.rotate_x,
                scene.rotate_y,
                scene.rotate_z,
                scene.fov,
                layer.quads(),
                scale_factor,
                offset,
                drawable_size,
                command_encoder,
            );
            self.render_sprites(
                z_sprites,
                scene.scale,
                scene.rotate_x,
                scene.rotate_y,
                scene.rotate_z,
                scene.fov,
                layer.glyphs(),
                layer.icons(),
                scale_factor,
                offset,
                drawable_size,
                command_encoder,
            );
        }

        command_encoder.end_encoding();
    }

    fn render_quads(
        &mut self,
        z: f32,
        scale: f32,
        rotate_x: f32,
        rotate_y: f32,
        rotate_z: f32,
        fov: f32,
        quads: &[Quad],
        scale_factor: f32,
        offset: &mut usize,
        drawable_size: Vector2F,
        command_encoder: &metal::RenderCommandEncoderRef,
    ) {
        if quads.is_empty() {
            return;
        }
        align_offset(offset);
        let next_offset = *offset + quads.len() * mem::size_of::<shaders::GPUIQuad>();
        assert!(
            next_offset <= INSTANCE_BUFFER_SIZE,
            "instance buffer exhausted"
        );

        command_encoder.set_render_pipeline_state(&self.quad_pipeline_state);
        command_encoder.set_vertex_buffer(
            shaders::GPUIQuadInputIndex_GPUIQuadInputIndexVertices as u64,
            Some(&self.unit_vertices),
            0,
        );
        command_encoder.set_vertex_buffer(
            shaders::GPUIQuadInputIndex_GPUIQuadInputIndexQuads as u64,
            Some(&self.instances),
            *offset as u64,
        );
        command_encoder.set_vertex_bytes(
            shaders::GPUIQuadInputIndex_GPUIQuadInputIndexUniforms as u64,
            mem::size_of::<shaders::GPUIUniforms>() as u64,
            [shaders::GPUIUniforms {
                viewport_size: drawable_size.to_float2(),
                scale,
                rotate_x,
                rotate_y,
                rotate_z,
                fov,
            }]
            .as_ptr() as *const c_void,
        );

        let buffer_contents = unsafe {
            (self.instances.contents() as *mut u8).add(*offset) as *mut shaders::GPUIQuad
        };
        for (ix, quad) in quads.iter().enumerate() {
            let bounds = quad.bounds * scale_factor;
            let border_width = quad.border.width * scale_factor;
            let shader_quad = shaders::GPUIQuad {
                origin: bounds.origin().round().to_float2(),
                size: bounds.size().round().to_float2(),
                background_color: quad
                    .background
                    .unwrap_or_else(Color::transparent_black)
                    .to_uchar4(),
                border_top: border_width * (quad.border.top as usize as f32),
                border_right: border_width * (quad.border.right as usize as f32),
                border_bottom: border_width * (quad.border.bottom as usize as f32),
                border_left: border_width * (quad.border.left as usize as f32),
                border_color: quad.border.color.to_uchar4(),
                corner_radius: quad.corner_radius * scale_factor,
                z,
            };
            unsafe {
                *(buffer_contents.add(ix)) = shader_quad;
            }
        }

        command_encoder.draw_primitives_instanced(
            metal::MTLPrimitiveType::Triangle,
            0,
            6,
            quads.len() as u64,
        );
        *offset = next_offset;
    }

    fn render_sprites(
        &mut self,
        z: f32,
        scale: f32,
        rotate_x: f32,
        rotate_y: f32,
        rotate_z: f32,
        fov: f32,
        glyphs: &[Glyph],
        icons: &[Icon],
        scale_factor: f32,
        offset: &mut usize,
        drawable_size: Vector2F,
        command_encoder: &metal::RenderCommandEncoderRef,
    ) {
        if glyphs.is_empty() && icons.is_empty() {
            return;
        }

        let mut sprites_by_atlas = HashMap::new();

        for glyph in glyphs {
            if let Some(sprite) = self.sprite_cache.render_glyph(
                glyph.font_id,
                glyph.font_size,
                glyph.id,
                glyph.origin,
            ) {
                // Snap sprite to pixel grid.
                let origin = (glyph.origin * scale_factor).floor() + sprite.offset.to_f32();
                sprites_by_atlas
                    .entry(sprite.atlas_id)
                    .or_insert_with(Vec::new)
                    .push(shaders::GPUISprite {
                        origin: origin.to_float2(),
                        target_size: sprite.size.to_float2(),
                        source_size: sprite.size.to_float2(),
                        atlas_origin: sprite.atlas_origin.to_float2(),
                        color: glyph.color.to_uchar4(),
                        compute_winding: 0,
                        z,
                    });
            }
        }

        for icon in icons {
            // Snap sprite to pixel grid.
            let origin = (icon.bounds.origin() * scale_factor).floor();
            let target_size = (icon.bounds.size() * scale_factor).ceil();
            let source_size = (target_size * 2.).to_i32();

            let sprite =
                self.sprite_cache
                    .render_icon(source_size, icon.path.clone(), icon.svg.clone());
            if sprite.is_none() {
                continue;
            }
            let sprite = sprite.unwrap();

            sprites_by_atlas
                .entry(sprite.atlas_id)
                .or_insert_with(Vec::new)
                .push(shaders::GPUISprite {
                    origin: origin.to_float2(),
                    target_size: target_size.to_float2(),
                    source_size: sprite.size.to_float2(),
                    atlas_origin: sprite.atlas_origin.to_float2(),
                    color: icon.color.to_uchar4(),
                    compute_winding: 0,
                    z,
                });
        }

        command_encoder.set_render_pipeline_state(&self.sprite_pipeline_state);
        command_encoder.set_vertex_buffer(
            shaders::GPUISpriteVertexInputIndex_GPUISpriteVertexInputIndexVertices as u64,
            Some(&self.unit_vertices),
            0,
        );
        command_encoder.set_vertex_bytes(
            shaders::GPUISpriteVertexInputIndex_GPUISpriteVertexInputIndexUniforms as u64,
            mem::size_of::<shaders::GPUIUniforms>() as u64,
            [shaders::GPUIUniforms {
                viewport_size: drawable_size.to_float2(),
                scale,
                rotate_x,
                rotate_y,
                rotate_z,
                fov,
            }]
            .as_ptr() as *const c_void,
        );

        for (atlas_id, sprites) in sprites_by_atlas {
            align_offset(offset);
            let next_offset = *offset + sprites.len() * mem::size_of::<shaders::GPUISprite>();
            assert!(
                next_offset <= INSTANCE_BUFFER_SIZE,
                "instance buffer exhausted"
            );

            let texture = self.sprite_cache.atlas_texture(atlas_id).unwrap();
            command_encoder.set_vertex_buffer(
                shaders::GPUISpriteVertexInputIndex_GPUISpriteVertexInputIndexSprites as u64,
                Some(&self.instances),
                *offset as u64,
            );
            command_encoder.set_vertex_bytes(
                shaders::GPUISpriteVertexInputIndex_GPUISpriteVertexInputIndexAtlasSize as u64,
                mem::size_of::<shaders::vector_float2>() as u64,
                [vec2i(texture.width() as i32, texture.height() as i32).to_float2()].as_ptr()
                    as *const c_void,
            );

            command_encoder.set_fragment_texture(
                shaders::GPUISpriteFragmentInputIndex_GPUISpriteFragmentInputIndexAtlas as u64,
                Some(texture),
            );

            unsafe {
                let buffer_contents =
                    (self.instances.contents() as *mut u8).add(*offset) as *mut shaders::GPUISprite;
                std::ptr::copy_nonoverlapping(sprites.as_ptr(), buffer_contents, sprites.len());
            }

            command_encoder.draw_primitives_instanced(
                metal::MTLPrimitiveType::Triangle,
                0,
                6,
                sprites.len() as u64,
            );
            *offset = next_offset;
        }
    }
}

fn build_path_atlas_texture_descriptor() -> metal::TextureDescriptor {
    let texture_descriptor = metal::TextureDescriptor::new();
    texture_descriptor.set_width(2048);
    texture_descriptor.set_height(2048);
    texture_descriptor.set_pixel_format(MTLPixelFormat::R16Float);
    texture_descriptor
        .set_usage(metal::MTLTextureUsage::RenderTarget | metal::MTLTextureUsage::ShaderRead);
    texture_descriptor.set_storage_mode(metal::MTLStorageMode::Private);
    texture_descriptor
}

fn align_offset(offset: &mut usize) {
    let r = *offset % 256;
    if r > 0 {
        *offset += 256 - r; // Align to a multiple of 256 to make Metal happy
    }
}

fn build_pipeline_state(
    device: &metal::DeviceRef,
    library: &metal::LibraryRef,
    label: &str,
    vertex_fn_name: &str,
    fragment_fn_name: &str,
    pixel_format: metal::MTLPixelFormat,
) -> metal::RenderPipelineState {
    let vertex_fn = library
        .get_function(vertex_fn_name, None)
        .expect("error locating vertex function");
    let fragment_fn = library
        .get_function(fragment_fn_name, None)
        .expect("error locating fragment function");

    let descriptor = metal::RenderPipelineDescriptor::new();
    descriptor.set_label(label);
    descriptor.set_vertex_function(Some(vertex_fn.as_ref()));
    descriptor.set_fragment_function(Some(fragment_fn.as_ref()));
    let color_attachment = descriptor.color_attachments().object_at(0).unwrap();
    color_attachment.set_pixel_format(pixel_format);
    color_attachment.set_blending_enabled(true);
    color_attachment.set_rgb_blend_operation(metal::MTLBlendOperation::Add);
    color_attachment.set_alpha_blend_operation(metal::MTLBlendOperation::Add);
    color_attachment.set_source_rgb_blend_factor(metal::MTLBlendFactor::SourceAlpha);
    color_attachment.set_source_alpha_blend_factor(metal::MTLBlendFactor::One);
    color_attachment.set_destination_rgb_blend_factor(metal::MTLBlendFactor::OneMinusSourceAlpha);
    color_attachment.set_destination_alpha_blend_factor(metal::MTLBlendFactor::One);

    device
        .new_render_pipeline_state(&descriptor)
        .expect("could not create render pipeline state")
}

fn build_path_atlas_pipeline_state(
    device: &metal::DeviceRef,
    library: &metal::LibraryRef,
    label: &str,
    vertex_fn_name: &str,
    fragment_fn_name: &str,
    pixel_format: metal::MTLPixelFormat,
) -> metal::RenderPipelineState {
    let vertex_fn = library
        .get_function(vertex_fn_name, None)
        .expect("error locating vertex function");
    let fragment_fn = library
        .get_function(fragment_fn_name, None)
        .expect("error locating fragment function");

    let descriptor = metal::RenderPipelineDescriptor::new();
    descriptor.set_label(label);
    descriptor.set_vertex_function(Some(vertex_fn.as_ref()));
    descriptor.set_fragment_function(Some(fragment_fn.as_ref()));
    let color_attachment = descriptor.color_attachments().object_at(0).unwrap();
    color_attachment.set_pixel_format(pixel_format);
    color_attachment.set_blending_enabled(true);
    color_attachment.set_rgb_blend_operation(metal::MTLBlendOperation::Add);
    color_attachment.set_alpha_blend_operation(metal::MTLBlendOperation::Add);
    color_attachment.set_source_rgb_blend_factor(metal::MTLBlendFactor::One);
    color_attachment.set_source_alpha_blend_factor(metal::MTLBlendFactor::One);
    color_attachment.set_destination_rgb_blend_factor(metal::MTLBlendFactor::One);
    color_attachment.set_destination_alpha_blend_factor(metal::MTLBlendFactor::One);

    device
        .new_render_pipeline_state(&descriptor)
        .expect("could not create render pipeline state")
}

mod shaders {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    use crate::{
        color::Color,
        geometry::vector::{Vector2F, Vector2I},
    };
    use std::mem;

    include!(concat!(env!("OUT_DIR"), "/shaders.rs"));

    pub trait ToFloat2 {
        fn to_float2(&self) -> vector_float2;
    }

    impl ToFloat2 for (f32, f32) {
        fn to_float2(&self) -> vector_float2 {
            unsafe {
                let mut output = mem::transmute::<_, u32>(self.1.to_bits()) as vector_float2;
                output <<= 32;
                output |= mem::transmute::<_, u32>(self.0.to_bits()) as vector_float2;
                output
            }
        }
    }

    impl ToFloat2 for Vector2F {
        fn to_float2(&self) -> vector_float2 {
            unsafe {
                let mut output = mem::transmute::<_, u32>(self.y().to_bits()) as vector_float2;
                output <<= 32;
                output |= mem::transmute::<_, u32>(self.x().to_bits()) as vector_float2;
                output
            }
        }
    }

    impl ToFloat2 for Vector2I {
        fn to_float2(&self) -> vector_float2 {
            self.to_f32().to_float2()
        }
    }

    impl Color {
        pub fn to_uchar4(&self) -> vector_uchar4 {
            let mut vec = self.a as vector_uchar4;
            vec <<= 8;
            vec |= self.b as vector_uchar4;
            vec <<= 8;
            vec |= self.g as vector_uchar4;
            vec <<= 8;
            vec |= self.r as vector_uchar4;
            vec
        }
    }
}
