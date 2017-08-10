//! GPU resource managers

pub use resource::framebuffer_manager::{FramebufferManager, RenderTarget, OffscreenBuffers};
pub use resource::texture_manager::{Texture, TextureManager, TextureWrapping};
pub use resource::material::Material;
pub use resource::material_manager::MaterialManager;
pub use resource::mesh_manager::MeshManager;
pub use resource::shader::{Shader, ShaderAttribute, ShaderUniform};
pub use resource::gpu_vector::{GPUVec, BufferType, AllocationType};
pub use resource::gl_primitive::GLPrimitive;
pub use resource::mesh::Mesh;

mod framebuffer_manager;
mod texture_manager;
mod mesh_manager;
mod material_manager;
pub mod material;
mod gpu_vector;
mod gl_primitive;
mod mesh;
mod shader;
