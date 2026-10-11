use crate::camera::BackgroundCamera;
use bevy::asset::RenderAssetUsages;
use bevy::image::TextureFormatPixelInfo;
use bevy::mesh::VertexBufferLayout;
use bevy::prelude::*;
use bevy::render::extract_resource::ExtractResource;
use bevy::render::render_resource::{
    binding_types::{sampler, texture_2d},
    AddressMode, BindGroup, BindGroupEntries, BindGroupLayoutDescriptor, BindGroupLayoutEntries,
    BlendComponent, BlendState, Buffer, BufferAddress, BufferInitDescriptor, BufferUsages,
    ColorTargetState, ColorWrites, Extent3d, FilterMode, FragmentState, FrontFace, IndexFormat,
    MipmapFilterMode, MultisampleState, PipelineCache, PolygonMode, PrimitiveState,
    PrimitiveTopology, RenderPassDescriptor, RenderPipelineDescriptor, SamplerBindingType,
    SamplerDescriptor, ShaderStages, SpecializedRenderPipeline, SpecializedRenderPipelines,
    TexelCopyBufferLayout, TextureDescriptor, TextureDimension, TextureFormat, TextureSampleType,
    TextureUsages, TextureViewDescriptor, VertexAttribute, VertexFormat, VertexState,
    VertexStepMode,
};
use bevy::render::renderer::{RenderContext, RenderDevice, RenderQueue, ViewQuery};
use bevy::render::view::{ExtractedView, Msaa, ViewTarget};
use bevy::shader::Shader;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertex {
    fn layout() -> VertexBufferLayout {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: vec![
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float32x3,
                },
                VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as BufferAddress,
                    shader_location: 1,
                    format: VertexFormat::Float32x2,
                },
            ],
        }
    }
}

#[derive(Deref, DerefMut, Default, Resource, ExtractResource, Clone)]
#[extract_app(bevy::render::RenderApp)]
pub struct BackgroundImage(pub Image);

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-1.0, -1.0, 0.0],
        tex_coords: [0.0, 1.0],
    }, // A
    Vertex {
        position: [1.0, -1.0, 0.0],
        tex_coords: [1.0, 1.0],
    }, // B
    Vertex {
        position: [-1.0, 1.0, 0.0],
        tex_coords: [0.0, 0.0],
    }, // C
    Vertex {
        position: [1.0, 1.0, 0.0],
        tex_coords: [1.0, 0.0],
    }, // d
];

const INDICES: &[u16] = &[0, 1, 2, 2, 1, 3];

/// Holds the shader, sampler, bind-group layout descriptor and vertex/index buffers used to
/// draw the webcam background. The render pipeline itself is specialized per view (because the
/// output format depends on the view's swapchain), see [`BackgroundPipelineKey`].
#[derive(Resource)]
pub struct BackgroundPipeline {
    pub shader: Handle<Shader>,
    pub bind_group_layout: BindGroupLayoutDescriptor,
    pub sampler: SamplerDescriptor<'static>,
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
}

impl FromWorld for BackgroundPipeline {
    fn from_world(world: &mut World) -> Self {
        let device = world.resource::<RenderDevice>();
        let asset_server = world.resource::<AssetServer>();

        let bind_group_layout = BindGroupLayoutDescriptor::new(
            "webcam_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    sampler(SamplerBindingType::Filtering),
                ),
            ),
        );

        let sampler = SamplerDescriptor {
            label: Some("webcam_sampler"),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Nearest,
            mipmap_filter: MipmapFilterMode::Nearest,
            ..Default::default()
        };

        let vertex_buffer = device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("Webcam Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("Webcam Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: BufferUsages::INDEX,
        });

        Self {
            shader: bevy::asset::load_embedded_asset!(asset_server, "shader.wgsl"),
            bind_group_layout,
            sampler,
            vertex_buffer,
            index_buffer,
        }
    }
}

/// Per-view specialization key. The background draws into the view's main color target, so the
/// pipeline must match the view's texture format ([`ExtractedView::target_format`]) **and** its
/// MSAA sample count — the render pass's color attachment is multisampled when MSAA is on.
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct BackgroundPipelineKey {
    format: TextureFormat,
    sample_count: u32,
}

impl SpecializedRenderPipeline for BackgroundPipeline {
    type Key = BackgroundPipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        RenderPipelineDescriptor {
            label: Some("Webcam Background Pipeline".into()),
            layout: vec![self.bind_group_layout.clone()],
            immediate_size: 0,
            vertex: VertexState {
                shader: self.shader.clone(),
                shader_defs: vec![],
                constants: default(),
                entry_point: Some("vs_main".into()),
                buffers: vec![Vertex::layout()],
            },
            fragment: Some(FragmentState {
                shader: self.shader.clone(),
                shader_defs: vec![],
                constants: default(),
                entry_point: Some("fs_main".into()),
                targets: vec![Some(ColorTargetState {
                    format: key.format,
                    blend: Some(BlendState {
                        color: BlendComponent::REPLACE,
                        alpha: BlendComponent::REPLACE,
                    }),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: key.sample_count,
                ..default()
            },
            zero_initialize_workgroup_memory: false,
        }
    }
}

/// The texture and bind group for the current webcam frame, rebuilt every frame by
/// [`prepare_background`] and consumed by [`render_background`].
#[derive(Resource, Default)]
pub struct BackgroundBindGroup {
    pub bind_group: Option<BindGroup>,
}

/// Uploads the latest webcam frame into a GPU texture and (re)builds the bind group for it.
pub fn prepare_background(
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    pipeline_cache: Res<PipelineCache>,
    background_pipeline: Res<BackgroundPipeline>,
    image: Res<BackgroundImage>,
    mut bind_group: ResMut<BackgroundBindGroup>,
) {
    let img = &image.0;
    let Some(data) = img.data.as_ref() else {
        bind_group.bind_group = None;
        return;
    };

    let size = Extent3d {
        width: img.width(),
        height: img.height(),
        depth_or_array_layers: 1,
    };
    let texture = render_device.create_texture(&TextureDescriptor {
        label: Some("webcam_img"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: TextureFormat::Rgba8UnormSrgb,
        usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
        view_formats: &[],
    });

    // Pixel size is a `Result` in 0.19; Rgba8Unorm is always 4 bytes.
    let format_size = img.texture_descriptor.format.pixel_size().unwrap_or(4) as u32;

    render_queue.write_texture(
        texture.as_image_copy(),
        data,
        TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(img.width() * format_size),
            rows_per_image: None,
        },
        img.texture_descriptor.size,
    );

    let view = texture.create_view(&TextureViewDescriptor::default());
    let sampler = render_device.create_sampler(&background_pipeline.sampler);

    let layout = pipeline_cache.get_bind_group_layout(&background_pipeline.bind_group_layout);
    let group = render_device.create_bind_group(
        Some("webcam_diffuse_bind_group"),
        &layout,
        &BindGroupEntries::sequential((&view, &sampler)),
    );

    bind_group.bind_group = Some(group);
}

/// Draws the webcam frame as a fullscreen background quad for the current view, before the
/// main pass renders the scene on top of it. The render pipeline is specialized on the view's
/// output format and fetched from the [`PipelineCache`] (which compiles asynchronously).
pub fn render_background(
    pipeline_cache: Res<PipelineCache>,
    mut pipelines: ResMut<SpecializedRenderPipelines<BackgroundPipeline>>,
    background_pipeline: Res<BackgroundPipeline>,
    bind_group: Res<BackgroundBindGroup>,
    view: ViewQuery<(&ViewTarget, &ExtractedView, &Msaa)>,
    mut ctx: RenderContext,
) {
    let Some(bind_group) = bind_group.bind_group.as_ref() else {
        return;
    };

    let (target, extracted_view, msaa) = view.into_inner();

    // The pipeline is specialized on the view's output format AND sample count, since the
    // main color attachment is multisampled when MSAA is enabled.
    let key = BackgroundPipelineKey {
        format: extracted_view.target_format,
        sample_count: msaa.samples(),
    };
    let pipeline_id = pipelines.specialize(&pipeline_cache, &background_pipeline, key);
    let Some(pipeline) = pipeline_cache.get_render_pipeline(pipeline_id) else {
        // Pipeline hasn't finished compiling yet; skip this frame.
        return;
    };

    let pass_descriptor = RenderPassDescriptor {
        label: Some("background_pass"),
        color_attachments: &[Some(target.get_color_attachment())],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    };

    let mut render_pass = ctx.command_encoder().begin_render_pass(&pass_descriptor);

    render_pass.set_pipeline(pipeline);
    render_pass.set_bind_group(0, bind_group, &[]);
    // Unwrap Bevy's BufferSlice for the raw wgpu render pass.
    render_pass.set_vertex_buffer(0, *background_pipeline.vertex_buffer.slice(..));
    render_pass.set_index_buffer(
        *background_pipeline.index_buffer.slice(..),
        IndexFormat::Uint16,
    );
    render_pass.draw_indexed(0..(INDICES.len() as u32), 0, 0..1);
}

pub fn handle_background_image(
    cam_query: Query<&mut BackgroundCamera>,
    mut image: ResMut<BackgroundImage>,
) {
    for background_camera in cam_query.iter() {
        while let Some(rgba_image) = background_camera.image_rx.drain().last() {
            let size = Extent3d {
                width: rgba_image.width(),
                height: rgba_image.height(),
                depth_or_array_layers: 1,
            };
            let dimensions = TextureDimension::D2;
            let format = TextureFormat::Rgba8Unorm;
            let asset_usage = RenderAssetUsages::default();
            image.0 = Image::new(size, dimensions, rgba_image.to_vec(), format, asset_usage);
        }
    }
}
