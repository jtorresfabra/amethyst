//! ECS rendering bundle

use amethyst_assets::{AssetStorage, Handle, Loader};
use amethyst_core::bundle::{ECSBundle, Result, ResultExt};
use amethyst_core::orientation::Orientation;
use amethyst_core::transform::components::*;
use specs::{DispatcherBuilder, World};

use {AmbientColor, Camera, Light, Material, MaterialDefaults, Mesh, Rgba, ScreenDimensions,
     Texture, WindowMessages};
use config::DisplayConfig;
use pipe::{PipelineBuild, PolyPipeline};
use system::RenderSystem;

/// Rendering bundle
///
/// Will register all necessary components needed for rendering, along with any resources.
/// Will also register asset contexts with the asset `Loader`, and add systems for merging
/// `AssetFuture` into its related component.
///
pub struct RenderBundle<B, P>
where
    B: PipelineBuild<Pipeline = P>,
    P: PolyPipeline,
{
    pipe: B,
    config: Option<DisplayConfig>,
}

impl<B, P> RenderBundle<B, P>
where
    B: PipelineBuild<Pipeline = P>,
    P: PolyPipeline,
{
    /// Create a new render bundle
    pub fn new(pipe: B, config: Option<DisplayConfig>) -> Self {
        Self { pipe, config }
    }
}

impl<'a, 'b, B: PipelineBuild<Pipeline = P>, P: 'b + PolyPipeline> ECSBundle<'a, 'b>
    for RenderBundle<B, P>
{
    fn build(
        self,
        world: &mut World,
        builder: DispatcherBuilder<'a, 'b>,
    ) -> Result<DispatcherBuilder<'a, 'b>> {
        world.add_resource(AmbientColor(Rgba::from([0.01; 3])));
        world.add_resource(WindowMessages::new());
        world.add_resource(AssetStorage::<Mesh>::new());
        world.add_resource(AssetStorage::<Texture>::new());
        world.add_resource(Orientation::default());

        let mat = create_default_mat(world);
        world.add_resource(MaterialDefaults(mat));

        world.register::<Transform>();
        world.register::<Light>();
        world.register::<Material>();
        world.register::<Handle<Mesh>>();
        world.register::<Handle<Texture>>();
        world.register::<Camera>();
        let system = RenderSystem::build(self.pipe, self.config).chain_err(|| "Renderer error!")?;
        let (width, height) = system
            .window_size()
            .expect("Window closed during initialization!");
        world.add_resource(ScreenDimensions::new(width, height));
        Ok(builder.add_thread_local(system))
    }
}

fn create_default_mat(world: &World) -> Material {
    let loader = world.read_resource::<Loader>();

    let albedo = [0.5, 0.5, 0.5, 1.0].into();
    let emission = [0.0; 4].into();
    let normal = [0.5, 0.5, 1.0, 1.0].into();
    let metallic = [0.0; 4].into();
    let roughness = [0.5; 4].into();
    let ambient_occlusion = [1.0; 4].into();
    let caveat = [1.0; 4].into();

    let tex_storage = world.read_resource();

    let albedo = loader.load_from_data(albedo, (), &tex_storage);
    let emission = loader.load_from_data(emission, (), &tex_storage);
    let normal = loader.load_from_data(normal, (), &tex_storage);
    let metallic = loader.load_from_data(metallic, (), &tex_storage);
    let roughness = loader.load_from_data(roughness, (), &tex_storage);
    let ambient_occlusion = loader.load_from_data(ambient_occlusion, (), &tex_storage);
    let caveat = loader.load_from_data(caveat, (), &tex_storage);

    Material {
        albedo,
        emission,
        normal,
        metallic,
        roughness,
        ambient_occlusion,
        caveat,
    }
}
