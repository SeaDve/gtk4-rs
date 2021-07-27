// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

#[cfg(any(feature = "broadway", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "broadway")))]
mod broadway_renderer;
#[cfg(any(feature = "broadway", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "broadway")))]
pub use self::broadway_renderer::BroadwayRenderer;

mod cairo_renderer;
pub use self::cairo_renderer::CairoRenderer;

mod gl_renderer;
pub use self::gl_renderer::GLRenderer;

mod gl_shader;
pub use self::gl_shader::GLShader;
pub use self::gl_shader::GLShaderBuilder;

#[cfg(any(feature = "v4_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v4_2")))]
mod ngl_renderer;
#[cfg(any(feature = "v4_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v4_2")))]
pub use self::ngl_renderer::NglRenderer;

mod renderer;
pub use self::renderer::{Renderer, NONE_RENDERER};

mod shader_args_builder;
pub use self::shader_args_builder::ShaderArgsBuilder;

mod transform;
pub use self::transform::Transform;

mod enums;
pub use self::enums::BlendMode;
pub use self::enums::Corner;
pub use self::enums::GLUniformType;
pub use self::enums::RenderNodeType;
pub use self::enums::ScalingFilter;
pub use self::enums::SerializationError;
pub use self::enums::TransformCategory;

#[doc(hidden)]
pub mod traits {
    pub use super::renderer::RendererExt;
}
