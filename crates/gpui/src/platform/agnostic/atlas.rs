use crate::geometry::{
    rect::RectI,
    vector::{vec2i, Vector2I},
};
use etagere::BucketedAtlasAllocator;
use wgpu::{Device, Queue, Texture, TextureDescriptor, TextureFormat};

pub struct AtlasAllocator {
    device: Device,
    queue: Queue,
    texture_descriptor: TextureDescriptor<'static>,
    atlases: Vec<Atlas>,
    free_atlases: Vec<Atlas>,
}

#[derive(Copy, Clone)]
pub struct AllocId {
    pub atlas_id: usize,
    alloc_id: etagere::AllocId,
}

impl AtlasAllocator {
    pub fn new(device: Device, queue: Queue, texture_descriptor: TextureDescriptor<'static>) -> Self {
        let mut me = Self {
            device,
            queue,
            texture_descriptor,
            atlases: Vec::new(),
            free_atlases: Vec::new(),
        };
        let atlas = me.new_atlas(Vector2I::zero());
        me.atlases.push(atlas);
        me
    }

    pub fn default_atlas_size(&self) -> Vector2I {
        vec2i(
            self.texture_descriptor.size.width as i32,
            self.texture_descriptor.size.height as i32,
        )
    }

    pub fn allocate(&mut self, requested_size: Vector2I) -> (AllocId, Vector2I) {
        let (alloc_id, origin) = self
            .atlases
            .last_mut()
            .unwrap()
            .allocate(requested_size)
            .unwrap_or_else(|| {
                let mut atlas = self.new_atlas(requested_size);
                let (id, origin) = atlas.allocate(requested_size).unwrap();
                self.atlases.push(atlas);
                (id, origin)
            });

        let id = AllocId {
            atlas_id: self.atlases.len() - 1,
            alloc_id,
        };
        (id, origin)
    }

    pub fn upload(&mut self, size: Vector2I, bytes: &[u8]) -> (AllocId, RectI) {
        let (alloc_id, origin) = self.allocate(size);
        let bounds = RectI::new(origin, size);
        self.atlases[alloc_id.atlas_id].upload(&self.queue, bounds, bytes);
        (alloc_id, bounds)
    }

    pub fn deallocate(&mut self, id: AllocId) {
        if let Some(atlas) = self.atlases.get_mut(id.atlas_id) {
            atlas.deallocate(id.alloc_id);
            if atlas.is_empty() {
                self.free_atlases.push(self.atlases.remove(id.atlas_id));
            }
        }
    }

    pub fn clear(&mut self) {
        for atlas in &mut self.atlases {
            atlas.clear();
        }
        self.free_atlases.extend(self.atlases.drain(1..));
    }

    pub fn texture(&self, atlas_id: usize) -> Option<&Texture> {
        self.atlases.get(atlas_id).map(|a| &a.texture)
    }

    fn new_atlas(&mut self, required_size: Vector2I) -> Atlas {
        if let Some(i) = self.free_atlases.iter().rposition(|atlas| {
            atlas.size().x() >= required_size.x() && atlas.size().y() >= required_size.y()
        }) {
            self.free_atlases.remove(i)
        } else {
            let size = self.default_atlas_size().max(required_size);
            if size.x() as u32 > self.texture_descriptor.size.width
                || size.y() as u32 > self.texture_descriptor.size.height
            {
                self.texture_descriptor.size = wgpu::Extent3d {
                    width: size.x() as u32,
                    height: size.y() as u32,
                    depth_or_array_layers: 1,
                };
            }

            let texture = self.device.create_texture(&self.texture_descriptor);
            Atlas::new(&self.texture_descriptor, size, texture)
        }
    }
}

struct Atlas {
    allocator: BucketedAtlasAllocator,
    texture: Texture,
    texture_format: TextureFormat,
}

impl Atlas {
    fn new(texture_descriptor: &TextureDescriptor<'static>, size: Vector2I, texture: Texture) -> Self {
        Self {
            allocator: BucketedAtlasAllocator::new(etagere::Size::new(size.x(), size.y())),
            texture,
            texture_format: texture_descriptor.format,
        }
    }

    fn size(&self) -> Vector2I {
        let size = self.allocator.size();
        vec2i(size.width, size.height)
    }

    fn allocate(&mut self, size: Vector2I) -> Option<(etagere::AllocId, Vector2I)> {
        let alloc = self
            .allocator
            .allocate(etagere::Size::new(size.x(), size.y()))?;
        let origin = alloc.rectangle.min;
        Some((alloc.id, vec2i(origin.x, origin.y)))
    }

    fn upload(&mut self, queue: &Queue, bounds: RectI, data_to_upload: &[u8]) {
        let location_to_write_to = wgpu::ImageCopyTexture {
            texture: &self.texture,
            mip_level: 0,
            origin: wgpu::Origin3d {
                x: bounds.origin_x() as u32,
                y: bounds.origin_y() as u32,
                z: 0,
            },
            aspect: wgpu::TextureAspect::All,
        };

        let bytes_per_row = std::num::NonZeroU32::new(
            bounds.width() as u32 * self.bytes_per_pixel() as u32);
        let image_data_layout = wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row,
            rows_per_image: None,
        };

        let modified_size = wgpu::Extent3d {
            width: bounds.width() as u32,
            height: bounds.height() as u32,
            depth_or_array_layers: 1,
        };

        queue.write_texture(
            location_to_write_to,
            data_to_upload,
            image_data_layout,
            modified_size,
        );
    }

    fn bytes_per_pixel(&self) -> u8 {
        self.texture_format.describe().block_size
    }

    fn deallocate(&mut self, id: etagere::AllocId) {
        self.allocator.deallocate(id);
    }

    fn is_empty(&self) -> bool {
        self.allocator.is_empty()
    }

    fn clear(&mut self) {
        self.allocator.clear();
    }
}
