pub mod color;
pub mod material;
pub mod mesh;
pub mod texture;
pub mod vertex;

use image::{DynamicImage, ImageBuffer, ImageReader, Rgb, Rgba};
use rustc_hash::{FxBuildHasher, FxHashMap};
use std::{collections::HashMap, path::Path, sync::Arc};
use vulkano::{
  buffer::{
    Buffer, BufferContents, BufferCreateInfo, BufferUsage, Subbuffer,
    allocator::{SubbufferAllocator, SubbufferAllocatorCreateInfo},
  },
  command_buffer::{
    AutoCommandBufferBuilder, CommandBufferUsage, CopyBufferToImageInfo,
    PrimaryCommandBufferAbstract, allocator::StandardCommandBufferAllocator,
  },
  descriptor_set::{DescriptorSet, WriteDescriptorSet, allocator::DescriptorSetAllocator},
  device::{Device, Queue},
  image::{
    Image, ImageCreateInfo, ImageType, ImageUsage,
    sampler::{Filter, Sampler, SamplerAddressMode, SamplerCreateInfo},
    view::ImageView,
  },
  memory::allocator::{
    AllocationCreateInfo, FreeListAllocator, GenericMemoryAllocator, MemoryTypeFilter,
  },
  sync::GpuFuture,
};

pub use color::{MdrColorType, MdrRgb, MdrRgba};
pub use material::{
  MdrGpuMaterialHandle, MdrMaterial, MdrMeshMaterialCreateInfo, MdrMeshMaterialData,
};
pub use mesh::{MdrGpuMeshHandle, MdrMesh, MdrMeshData};
pub use texture::{MdrGpuTextureHandle, MdrTexture};
pub use vertex::{MdrVertex_norm, MdrVertex_pos, MdrVertex_uv};

use crate::graphics::pipeline::MdrEnginePipelines;

use self::{
  color::MdrColor,
  texture::{MdrSamplerMode, MdrTextureCreateInfo},
};

/// Manages resources on the GPU by storing meshes, textures, and materials into libraries which
/// can be accessed by key.
///
/// Objects in the scene only store these keys rather than maintaining references to the buffers
/// in which their data is stored.
pub struct MdrResourceManager {
  logical_device: Arc<Device>,
  memory_allocator: Arc<GenericMemoryAllocator<FreeListAllocator>>,
  command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
  descriptor_set_allocator: Arc<dyn DescriptorSetAllocator>,
  pipelines: MdrEnginePipelines,
  queue: Arc<Queue>,

  vertex_allocator: SubbufferAllocator,
  index_allocator: SubbufferAllocator,
  mesh_library: HashMap<String, MdrGpuMeshHandle, FxBuildHasher>,

  material_library: HashMap<String, MdrGpuMaterialHandle, FxBuildHasher>,
  material_allocator: SubbufferAllocator,

  texture_transfer_futures: Option<Box<dyn GpuFuture>>,
  sampler_palette: HashMap<MdrSamplerMode, Arc<Sampler>, FxBuildHasher>,
  texture_library: HashMap<String, MdrGpuTextureHandle, FxBuildHasher>,
}

impl MdrResourceManager {
  pub fn new(
    logical_device: Arc<Device>,
    memory_allocator: Arc<GenericMemoryAllocator<FreeListAllocator>>,
    command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
    descriptor_set_allocator: Arc<dyn DescriptorSetAllocator>,
    pipelines: MdrEnginePipelines,
    queue: Arc<Queue>,
  ) -> Self {
    // Mesh memory handler initialization
    let vertex_allocator = SubbufferAllocator::new(
      memory_allocator.clone(),
      SubbufferAllocatorCreateInfo {
        buffer_usage: BufferUsage::VERTEX_BUFFER,
        memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
          | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
        ..Default::default()
      },
    );
    let index_allocator = SubbufferAllocator::new(
      memory_allocator.clone(),
      SubbufferAllocatorCreateInfo {
        buffer_usage: BufferUsage::INDEX_BUFFER,
        memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
          | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
        ..Default::default()
      },
    );

    let mesh_library = FxHashMap::<String, MdrGpuMeshHandle>::default();

    // Material memory handler initialization
    let material_library = FxHashMap::<String, MdrGpuMaterialHandle>::default();
    let material_allocator = SubbufferAllocator::new(
      memory_allocator.clone(),
      SubbufferAllocatorCreateInfo {
        buffer_usage: BufferUsage::UNIFORM_BUFFER,
        memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
          | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
        ..Default::default()
      },
    );

    let sampler_palette = FxHashMap::<MdrSamplerMode, Arc<Sampler>>::default();
    let texture_library = FxHashMap::<String, MdrGpuTextureHandle>::default();

    Self {
      logical_device,
      memory_allocator,
      command_buffer_allocator,
      descriptor_set_allocator,
      pipelines,
      queue,

      vertex_allocator,
      index_allocator,
      mesh_library,

      material_library,
      material_allocator,

      texture_transfer_futures: None,
      sampler_palette,
      texture_library,
    }
  }

  // /////////////
  // Mesh handling
  // /////////////

  /// Load a mesh from an .obj file into the mesh library with a given name.
  /// `path` specifies a path to the .obj file.
  /// `name` is the name given to the mesh in the mesh library.
  pub fn load_mesh_obj(&mut self, path: &Path, name: &str) -> Result<MdrMesh, MdrResourceError> {
    // Check that the mesh name isn't already in use
    if self.mesh_library.contains_key(name) {
      tracing::error!("Mesh library already contains name: {name}");
      return Err(MdrResourceError::DuplicateMeshName);
    }

    let Some(mesh_data) = mesh::open_obj(path) else {
      return Err(MdrResourceError::ObjLoadError);
    };
    tracing::debug!("Loaded obj file: {path:?}");

    let mesh_handle = self.upload_mesh_to_gpu(&mesh_data);
    self.mesh_library.insert(String::from(name), mesh_handle);
    tracing::debug!("Added {name} to mesh library");

    Ok(MdrMesh {
      name: String::from(name),
    })
  }

  /// Returns an `MdrMesh` specified by `name` from the mesh library. If no match is found for the
  /// key, it returns `MdrResourceError::MeshNotFound`.
  pub fn retrieve_mesh(&self, name: &str) -> Result<MdrMesh, MdrResourceError> {
    if !self.mesh_library.contains_key(name) {
      return Err(MdrResourceError::MeshNotFound);
    }

    Ok(MdrMesh {
      name: String::from(name),
    })
  }

  /// Removes the mesh specified by `name` from the mesh library and drops it, freeing it
  /// from GPU memory. Doing this will effectively invalidate any existing `MdrMesh` objects.
  pub fn unload_mesh(&mut self, name: &str) {
    if !self.mesh_library.contains_key(name) {
      tracing::warn!("Cannot unload mesh {name} because it is not in the library",);
      return;
    }

    self.mesh_library.remove(&String::from(name));
  }

  // ////////////////
  // Texture handling
  // ////////////////

  /// Loads the texture specified in the input `texture_create_info` and stores it
  /// in the texture library for later use.
  pub fn load_texture(
    &mut self,
    texture_create_info: MdrTextureCreateInfo,
    name: &str,
  ) -> Result<MdrTexture, MdrResourceError> {
    // Check that the texture name isn't already in use
    if self.texture_library.contains_key(name) {
      tracing::error!("Texture library already contains name: {name}");
      return Err(MdrResourceError::DuplicateTextureName);
    }

    // Load image data from disk
    let image = match ImageReader::open(texture_create_info.source) {
      Ok(reader) => reader.decode().unwrap(),
      Err(_) => return Err(MdrResourceError::ImageLoadError),
    };

    // Upload to GPU and catalogue texture in library
    let texture_handle = self.upload_image_to_gpu(&image, texture_create_info);
    self
      .texture_library
      .insert(String::from(name), texture_handle);
    tracing::debug!("Added {name} to texture library");

    Ok(MdrTexture {
      name: String::from(name),
    })
  }

  /// Creates a single-pixel texture with the input `MdrColor` and stores it in the texture library
  /// for later use.
  pub fn create_solid_texture(
    &mut self,
    color: MdrColor,
    name: &str,
  ) -> Result<MdrTexture, MdrResourceError> {
    // Check that the texture name isn't already in use
    if self.texture_library.contains_key(name) {
      tracing::error!("Texture library already contains name: {name}");
      return Err(MdrResourceError::DuplicateTextureName);
    }

    let image = match color {
      MdrColor::RGB(rgb) => {
        let rgb_u8 = [
          (rgb.r * 255.0) as u8,
          (rgb.g * 255.0) as u8,
          (rgb.b * 255.0) as u8,
        ];
        let image_buffer = ImageBuffer::from_fn(1, 1, |_, _| Rgb(rgb_u8));
        DynamicImage::ImageRgb8(image_buffer)
      }
      MdrColor::RGBA(rgba) => {
        let rgba_u8 = [
          (rgba.r * 255.0) as u8,
          (rgba.g * 255.0) as u8,
          (rgba.b * 255.0) as u8,
          (rgba.a * 255.0) as u8,
        ];
        let image_buffer = ImageBuffer::from_fn(1, 1, |_, _| Rgba(rgba_u8));
        DynamicImage::ImageRgba8(image_buffer)
      }
    };

    // Upload to GPU and catalogue texture in library
    let texture_handle = self.upload_image_to_gpu(
      &image,
      MdrTextureCreateInfo {
        source: Path::new(""),
        color_type: MdrColorType::from(color),
        sampler_mode: MdrSamplerMode::ClampToEdge,
      },
    );
    self
      .texture_library
      .insert(String::from(name), texture_handle);
    tracing::debug!("Added {name} to texture library");

    Ok(MdrTexture {
      name: String::from(name),
    })
  }

  /// Returns an `MdrTexture` specified by `name` from the texture library. If no match is found for the
  /// key, it returns `MdrResourceError::TextureNotFound`.
  pub fn retrieve_texture(&self, name: &str) -> Result<MdrTexture, MdrResourceError> {
    if !self.texture_library.contains_key(name) {
      return Err(MdrResourceError::TextureNotFound);
    }

    Ok(MdrTexture {
      name: String::from(name),
    })
  }

  /// Removes the texture specified by `name` from the texture library and drops it, freeing it
  /// from GPU memory. Doing this will effectively invalidate any existing `MdrTexture` objects.
  pub fn unload_texture(&mut self, name: &str) {
    if !self.texture_library.contains_key(name) {
      tracing::warn!("Cannot unload texture {name} because it is not in the library",);
      return;
    }

    self.texture_library.remove(&String::from(name));
  }

  fn fetch_texture_by_name(&self, name: &str) -> Result<MdrGpuTextureHandle, MdrResourceError> {
    self.texture_library.get(name).map_or_else(
      || Err(MdrResourceError::TextureNotFound),
      |texture| Ok(texture.clone()),
    )
  }

  // /////////////////
  // Material handling
  // /////////////////

  /// Creates a material wih the input `create_info` and stores it in the material
  /// library under the key `name` for future use.
  pub fn create_material(
    &mut self,
    create_info: &MdrMeshMaterialCreateInfo,
    name: &str,
  ) -> Result<MdrMaterial, MdrResourceError> {
    // Check that the material name isn't already in use
    if self.material_library.contains_key(name) {
      tracing::error!("Material library already contains name: {name}");
      return Err(MdrResourceError::DuplicateMaterialName);
    }

    // Helper function and state for assigning texture indices and building
    // a texture image view/sampler list to upload to the GPU.
    // TODO - Remove unwraps
    let mut textures = Vec::new();
    let mut get_texture_idx = |texture: &MdrTexture| -> i32 {
      let idx = i32::try_from(textures.len()).unwrap();
      let texture_handle = self.fetch_texture_by_name(&texture.name).unwrap();
      textures.push((texture_handle.image_view.clone(), texture_handle.sampler));
      idx
    };

    // Generate material uniform buffer contents from create info
    let material = MdrMeshMaterialData {
      base_color_factor: create_info.base_color.into(),
      roughness_factor: create_info.base_roughness,
      metallic_factor: create_info.base_metallic,
      diffuse_texture_set: create_info
        .diffuse
        .as_ref()
        .map_or(-1, &mut get_texture_idx),
      metallic_roughness_texture_set: create_info
        .metallic_roughness
        .as_ref()
        .map_or(-1, &mut get_texture_idx),
      normal_texture_set: create_info.normal.as_ref().map_or(-1, &mut get_texture_idx),
      occlusion_texture_set: create_info
        .occlusion
        .as_ref()
        .map_or(-1, &mut get_texture_idx),
      emissive_texture_set: create_info
        .emissive
        .as_ref()
        .map_or(-1, &mut get_texture_idx),
    };

    // Push material to GPU and store in library
    let material_handle = self.upload_material_to_gpu(material, textures);
    self
      .material_library
      .insert(String::from(name), material_handle);
    tracing::debug!("Added {name} to material library");

    Ok(MdrMaterial {
      name: String::from(name),
    })
  }

  /// Returns an [`MdrMaterial`] specified by `name` from the material library. If no match is found for the
  /// key, it returns `MdrResourceError::MaterialNotFound`.
  pub fn retrieve_material(&self, name: &str) -> Result<MdrMaterial, MdrResourceError> {
    if !self.material_library.contains_key(name) {
      return Err(MdrResourceError::MaterialNotFound);
    }

    Ok(MdrMaterial {
      name: String::from(name),
    })
  }

  /// Removes the material specified by `name` from the material library and drops it, freeing it
  /// from GPU memory. Doing this will effectively invalidate any existing [`MdrMaterial`] objects.
  pub fn unload_material(&mut self, name: &str) {
    if !self.material_library.contains_key(name) {
      tracing::warn!("Cannot unload material {name} because it is not in the library",);
      return;
    }

    self.material_library.remove(&String::from(name));
  }

  // //////////////////
  // Internal functions
  // //////////////////

  pub(crate) fn take_upload_futures(&mut self) -> Option<Box<dyn GpuFuture>> {
    self.texture_transfer_futures.take()
  }

  /// Gets a reference to the `MdrGpuMeshHandle` that corresponds to the input `MdrMesh`.
  /// This is called when building the render command buffer to bind the underlying buffers.
  pub(crate) fn get_mesh_handle(&self, mesh: &MdrMesh) -> &MdrGpuMeshHandle {
    match self.mesh_library.get_key_value(&mesh.name) {
      Some((_, handle)) => handle,
      None => {
        panic!("Could not find mesh {} in mesh library", mesh.name);
      }
    }
  }

  /// Gets a reference to the `MdrGpuMaterialHandle` that corresponds to the input `MdrMaterial`.
  /// This is called when building the render command buffer to bind the underlying buffers.
  pub(crate) fn get_material_handle(&self, mat: &MdrMaterial) -> &MdrGpuMaterialHandle {
    match self.material_library.get_key_value(&mat.name) {
      Some((_, handle)) => handle,
      None => {
        panic!("Could not find material {} in mat library", mat.name);
      }
    }
  }

  /// Uploads input `MdrMeshdata` to the GPU and returns an `MdrGpuMeshHandle` containing the
  /// vertex buffer, index buffer, and index count for the input data.
  fn upload_mesh_to_gpu(&self, mesh: &MdrMeshData) -> MdrGpuMeshHandle {
    // Upload vertex data
    let positions_buffer = self.upload_vertex_data(&mesh.positions);
    let normals_buffer = self.upload_vertex_data(&mesh.normals);
    let uvs_buffer = self.upload_vertex_data(&mesh.uvs);
    let tangents_buffer = self.upload_vertex_data(&mesh.tangents);

    // Upload index data
    let index_buffer = self.upload_index_data(&mesh.indices);
    let index_count = mesh.indices.len() as u32;

    MdrGpuMeshHandle {
      positions_buffer,
      normals_buffer,
      uvs_buffer,
      tangents_buffer,

      index_buffer,
      index_count,
    }
  }

  fn upload_vertex_data<T: Copy + BufferContents>(&self, data: &[T]) -> Subbuffer<[T]> {
    let vertex_buffer = self
      .vertex_allocator
      .allocate_slice(data.len() as u64)
      .unwrap();
    vertex_buffer.write().unwrap().copy_from_slice(data);
    vertex_buffer
  }

  fn upload_index_data(&self, indices: &[u32]) -> Subbuffer<[u32]> {
    let index_buffer = self
      .index_allocator
      .allocate_slice(indices.len() as u64)
      .unwrap();
    index_buffer.write().unwrap().copy_from_slice(indices);
    index_buffer
  }

  /// Uploads an input [`image::DynamicImage`] to the GPU  with settings defined by the `texture_create_info`.
  /// Returns an `MdrGpuTextureHandle` containing the resulting image view and sampler.
  fn upload_image_to_gpu(
    &mut self,
    image: &DynamicImage,
    create_info: MdrTextureCreateInfo,
  ) -> MdrGpuTextureHandle {
    // Get command buffer for upload
    // TODO - Is there another way to do this? Seems unnecessarily synchronous.
    let mut command_buffer_builder = AutoCommandBufferBuilder::primary(
      self.command_buffer_allocator.clone(),
      self.queue.queue_family_index(),
      CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();

    let extent = [image.width(), image.height(), 1];
    let upload_buffer = Buffer::new_slice(
      self.memory_allocator.clone(),
      BufferCreateInfo {
        usage: BufferUsage::TRANSFER_SRC,
        ..Default::default()
      },
      AllocationCreateInfo {
        memory_type_filter: MemoryTypeFilter::PREFER_HOST | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
        ..Default::default()
      },
      u64::from(extent[0] * extent[1] * create_info.color_type.component_count()),
    )
    .unwrap();

    // Upload image to buffer
    let mut write_guard = upload_buffer.write().unwrap();
    match create_info.color_type {
      MdrColorType::SRGBA | MdrColorType::NonColorData => {
        // SRGBA images are in standard (gamma-corrected) color space.
        //
        // NonColorData images are in linear color space, and their values are read as data, not rgb.
        // They are used for images that inform shading algorithms (normal maps, roughness maps, etc.)
        // TODO: Currently we're somewhat wasteful of memory because we use RGBA even when there isn't
        // a meaningful alpha channel.
        write_guard.copy_from_slice(&image.to_rgba8());
      }
      // SRGB images are in gamma-corrected color space, too, but with just the R, G, and B channels.
      MdrColorType::SRGB => write_guard.copy_from_slice(&image.to_rgb8()),
    }
    drop(write_guard);

    let image = Image::new(
      self.memory_allocator.clone(),
      ImageCreateInfo {
        image_type: ImageType::Dim2d,
        format: create_info.color_type.into(),
        extent,
        array_layers: 1,
        mip_levels: 1,
        usage: ImageUsage::TRANSFER_DST | ImageUsage::SAMPLED,
        ..Default::default()
      },
      AllocationCreateInfo::default(),
    )
    .unwrap();

    // Create command buffer containing command to transfer buffer to new image
    command_buffer_builder
      .copy_buffer_to_image(CopyBufferToImageInfo::buffer_image(
        upload_buffer,
        image.clone(),
      ))
      .unwrap();

    // Create ImageView and Sampler for the image
    let image_view = ImageView::new_default(image).unwrap();
    let sampler = self.get_sampler(create_info.sampler_mode);

    // Execute the transfer command
    let future = command_buffer_builder
      .build()
      .unwrap()
      .execute(self.queue.clone())
      .unwrap();
    // Store the future, optionally chaining it onto the last transfer future, if any
    self.texture_transfer_futures = match self.texture_transfer_futures.take() {
      Some(last) => Some(last.join(future).boxed()),
      None => Some(future.boxed()),
    };

    MdrGpuTextureHandle {
      image_view,
      sampler,
    }
  }

  /// Uploads an input [`MdrMeshMaterialData`] to the GPU .
  /// Returns an `MdrGpuMaterialHandle` containing the resulting buffer.
  fn upload_material_to_gpu(
    &self,
    material_uniforms: MdrMeshMaterialData,
    texture_elements: Vec<(Arc<ImageView>, Arc<Sampler>)>,
  ) -> MdrGpuMaterialHandle {
    tracing::trace!("Uploading material data to GPU: {material_uniforms:?}");

    let material_data = [material_uniforms];
    let material_buffer = self
      .material_allocator
      .allocate_slice(material_data.len() as u64)
      .unwrap();
    material_buffer
      .write()
      .unwrap()
      .copy_from_slice(&material_data);

    // Upload material data
    let descriptor_set = DescriptorSet::new_variable(
      self.descriptor_set_allocator.clone(),
      self.pipelines.mesh.descriptor_set_layout(),
      texture_elements.len() as u32,
      [
        // Material uniform data
        WriteDescriptorSet::buffer(0, material_buffer),
        // All image samplers for textures
        WriteDescriptorSet::image_view_sampler_array(1, 0, texture_elements),
      ],
      [],
    )
    .unwrap();

    MdrGpuMaterialHandle { descriptor_set }
  }

  /// Gets a sampler with the input `MdrSamplerMode` by either grabbing a reference off the
  /// sampler palette or, if none is available, creating a new one.
  fn get_sampler(&mut self, sampler_mode: MdrSamplerMode) -> Arc<Sampler> {
    // If we've already got that sampler, return it
    if let Some((_, sampler)) = self.sampler_palette.get_key_value(&sampler_mode) {
      return sampler.clone();
    }

    // If not, we need to create one
    // TODO We should probably put this in its own resource
    let sampler = Sampler::new(
      self.logical_device.clone(),
      SamplerCreateInfo {
        mag_filter: Filter::Linear,
        min_filter: Filter::Linear,
        address_mode: match sampler_mode {
          MdrSamplerMode::Repeat => [SamplerAddressMode::Repeat; 3],
          MdrSamplerMode::ClampToEdge => [SamplerAddressMode::ClampToEdge; 3],
        },
        ..Default::default()
      },
    )
    .unwrap();

    // Map the new sampler and return it
    self.sampler_palette.insert(sampler_mode, sampler.clone());
    sampler
  }
}

#[derive(Debug)]
/// Error emitted by [`MdrResourceManager`].
pub enum MdrResourceError {
  /// Emitted when the resource manager fails to load an .obj file.
  ObjLoadError,
  /// Emitted when the resource manager fails to load an image file.
  ImageLoadError,

  /// Emitted when the resource manager cannot find a mesh with a given name in its
  /// mesh library.
  MeshNotFound,
  /// Emitted when the resource manager attempts to add a mesh with a name that is
  /// already present in the mesh library.
  DuplicateMeshName,

  /// Emitted when the resource manager cannot find a material with a given name in its
  /// material library.
  MaterialNotFound,
  /// Emitted when the resource manager attempts to add a material with a name that is
  /// already present in the material library.
  DuplicateMaterialName,

  /// Emitted when the resource manager cannot find a texture with a given name in its
  /// texture library.
  TextureNotFound,
  /// Emitted when the resource manager attempts to add a texture with a name that is
  /// already present in the texture library.
  DuplicateTextureName,
}
