use amethyst_renderer::TextureHandle;
use specs::{Component, VecStorage};

/// A component with the texture to display in this entity's `UiTransform`
#[derive(Clone)]
pub struct UiImage {
    /// The texture to display
    pub texture: TextureHandle,
}

impl Component for UiImage {
    type Storage = VecStorage<Self>;
}
