use crate::{
    geometry::{rect::RectI, vector::Vector2I},
    ImageData,
};
use wgpu::{Device, Queue, Texture, TextureDescriptor, TextureFormat};
use std::{collections::HashMap, mem};

pub struct ImageCache {
    prev_frame: HashMap<usize, (AllocId, RectI)>,
    curr_frame: HashMap<usize, (AllocId, RectI)>,
    atlases: AtlasAllocator,
}

impl ImageCache {
    pub fn new(device: Device, queue: Queue, size: Vector2I) -> Self {
        let descriptor = TextureDescriptor {
            size: wgpu::Extent3d {
                width: size.x,
                height: size.y,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: TextureFormat::Bgra8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: None,
        };
        Self {
            prev_frame: Default::default(),
            curr_frame: Default::default(),
            atlases: AtlasAllocator::new(device, queue, descriptor),
        }
    } 

    pub fn render(&mut self, image: &ImageData) -> (AllocId, RectI) {
        let (alloc_id, atlas_bounds) = self
            .prev_frame
            .remove(&image.id)
            .or_else(|| self.curr_frame.get(&image.id).copied())
            .unwrap_or_else(|| self.atlases.upload(image.size(), image.as_bytes()));
        self.curr_frame.insert(image.id, (alloc_id, atlas_bounds));
        (alloc_id, atlas_bounds)
    }

    pub fn finish_frame(&mut self) {
        mem::swap(&mut self.prev_frame, &mut self.curr_frame);
        for (_, (id, _)) in self.curr_frame.drain() {
            self.atlases.deallocate(id);
        }
    }

    pub fn atlas_texture(&self, atlas_id: usize) -> Option<&Texture> {
        self.atlases.texture(atlas_id)
    }
}
