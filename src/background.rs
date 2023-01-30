use crate::camera::BackgroundCamera;
use bevy::prelude::*;
use bevy::render::extract_resource::ExtractResource;
use bevy::render::render_graph::Node;
use bevy::render::render_graph::{NodeRunError, RenderGraphContext, SlotInfo};
use bevy::render::render_resource::{
    AddressMode, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, BlendComponent, BlendState, Buffer,
    BufferAddress, BufferInitDescriptor, BufferUsages, ColorTargetState, ColorWrites, Extent3d,
    Face, FilterMode, FrontFace, ImageCopyTexture, ImageDataLayout, IndexFormat, LoadOp,
    MultisampleState, Operations, Origin3d, PipelineLayoutDescriptor, PolygonMode, PrimitiveState,
    PrimitiveTopology, RawFragmentState, RawRenderPipelineDescriptor, RawVertexBufferLayout,
    RawVertexState, RenderPassDescriptor, RenderPipeline, SamplerBindingType, SamplerDescriptor,
    ShaderModuleDescriptor, ShaderSource, ShaderStages, TextureAspect, TextureDescriptor,
    TextureDimension, TextureFormat, TextureSampleType, TextureUsages, TextureViewDescriptor,
    TextureViewDimension, VertexAttribute, VertexFormat, VertexStepMode,
};
use bevy::render::renderer::{RenderContext, RenderDevice, RenderQueue};
use bevy::render::texture::BevyDefault;
use bevy::render::view::{ExtractedView, ViewTarget};
use image::RgbaImage;
use std::num::NonZeroU32;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertex {
    fn desc<'a>() -> RawVertexBufferLayout<'a> {
        use std::mem;
        RawVertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float32x3,
                },
                VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as BufferAddress,
                    shader_location: 1,
                    format: VertexFormat::Float32x2,
                },
            ],
        }
    }
}

#[derive(Deref, DerefMut, Default, Resource, ExtractResource, Clone)]
pub struct BackgroundImage(pub RgbaImage);

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

const BACKGROUND_GRAPH: &str = "background_graph";
pub(crate) const BACKGROUND_NODE: &str = "background_node";

#[derive(Resource)]
pub struct BackgroundPipeline {
    render_pipeline: RenderPipeline,
}

impl FromWorld for BackgroundPipeline {
    fn from_world(world: &mut World) -> Self {
        let device = world.resource::<RenderDevice>();
        let samples = match world.get_resource::<Msaa>() {
            None => 4,
            Some(msaa) => msaa.samples,
        };
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Webcam Shader"),
            source: ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let texture_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("webcam_bind_group_layout"),
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            sample_type: TextureSampleType::Float { filterable: true },
                            view_dimension: TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Sampler(SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Webcam Render Pipeline Layout"),
            bind_group_layouts: &[&texture_bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&RawRenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: RawVertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(RawFragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: TextureFormat::bevy_default(),
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
                cull_mode: Some(Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: samples,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            // If the pipeline will be used with a multiview render pass, this
            // indicates how many array layers the attachments will have.
            multiview: None,
        });

        Self { render_pipeline }
    }
}

pub struct BackgroundPassDriverNode;

impl Node for BackgroundPassDriverNode {
    fn run(
        &self,
        graph: &mut RenderGraphContext,
        _render_context: &mut RenderContext,
        _world: &World,
    ) -> Result<(), NodeRunError> {
        graph.run_sub_graph(BACKGROUND_GRAPH, vec![])?;

        Ok(())
    }
}

pub struct BackgroundNode {
    query: QueryState<&'static ViewTarget, With<ExtractedView>>,
    vertex_buffer: Option<Buffer>,
    index_buffer: Option<Buffer>,
    diffuse_bind_group: Option<BindGroup>,
}

impl BackgroundNode {
    pub fn new(world: &mut World) -> Self {
        Self {
            query: QueryState::new(world),

            vertex_buffer: None,
            index_buffer: None,
            diffuse_bind_group: None,
        }
    }
}

impl Node for BackgroundNode {
    fn input(&self) -> Vec<SlotInfo> {
        vec![]
    }

    fn update(&mut self, world: &mut World) {
        self.query.update_archetypes(world);
        if let Some(img) = world.get_resource::<BackgroundImage>() {
            let device = world.get_resource::<RenderDevice>().unwrap();
            let queue = world.get_resource::<RenderQueue>().unwrap();

            if self.index_buffer.is_none() {
                let index_buffer = device.create_buffer_with_data(&BufferInitDescriptor {
                    label: Some("Index Buffer"),
                    contents: bytemuck::cast_slice(INDICES),
                    usage: BufferUsages::INDEX,
                });
                self.index_buffer = Some(index_buffer)
            }
            if self.vertex_buffer.is_none() {
                let vertex_buffer = device.create_buffer_with_data(&BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(VERTICES),
                    usage: BufferUsages::VERTEX,
                });
                self.vertex_buffer = Some(vertex_buffer)
            }

            let dimensions = img.0.dimensions();

            let size = Extent3d {
                width: dimensions.0,
                height: dimensions.1,
                depth_or_array_layers: 1,
            };
            let texture = device.create_texture(&TextureDescriptor {
                label: Some("webcam_img"),
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba8UnormSrgb,
                usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            });

            queue.write_texture(
                ImageCopyTexture {
                    aspect: TextureAspect::All,
                    texture: &texture,
                    mip_level: 0,
                    origin: Origin3d::ZERO,
                },
                &img.0,
                ImageDataLayout {
                    offset: 0,
                    bytes_per_row: NonZeroU32::new(4 * dimensions.0),
                    rows_per_image: NonZeroU32::new(dimensions.1),
                },
                size,
            );

            let view = texture.create_view(&TextureViewDescriptor::default());
            let sampler = device.create_sampler(&SamplerDescriptor {
                address_mode_u: AddressMode::ClampToEdge,
                address_mode_v: AddressMode::ClampToEdge,
                address_mode_w: AddressMode::ClampToEdge,
                mag_filter: FilterMode::Linear,
                min_filter: FilterMode::Nearest,
                mipmap_filter: FilterMode::Nearest,
                ..Default::default()
            });

            let texture_bind_group_layout =
                device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                    entries: &[
                        BindGroupLayoutEntry {
                            binding: 0,
                            visibility: ShaderStages::FRAGMENT,
                            ty: BindingType::Texture {
                                multisampled: false,
                                view_dimension: TextureViewDimension::D2,
                                sample_type: TextureSampleType::Float { filterable: true },
                            },
                            count: None,
                        },
                        BindGroupLayoutEntry {
                            binding: 1,
                            visibility: ShaderStages::FRAGMENT,
                            ty: BindingType::Sampler(SamplerBindingType::Filtering),
                            count: None,
                        },
                    ],
                    label: Some("texture_bind_group_layout"),
                });

            let diffuse_bind_group = device.create_bind_group(&BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: BindingResource::TextureView(&view),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: BindingResource::Sampler(&sampler),
                    },
                ],
                label: Some("diffuse_bind_group"),
            });

            self.diffuse_bind_group = Some(diffuse_bind_group);
        }
    }

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        for target in self.query.iter_manual(world) {
            let pipeline = world.get_resource::<BackgroundPipeline>().unwrap();
            let pass_descriptor = RenderPassDescriptor {
                label: Some("background_pass"),
                color_attachments: &[Some(target.get_color_attachment(Operations {
                    load: LoadOp::Load,
                    store: true,
                }))],
                depth_stencil_attachment: None,
            };

            if let (Some(vertex_buffer), Some(index_buffer)) =
                (&self.vertex_buffer, &self.index_buffer)
            {
                let mut render_pass = render_context
                    .command_encoder
                    .begin_render_pass(&pass_descriptor);

                render_pass.set_pipeline(&pipeline.render_pipeline);

                render_pass.set_bind_group(0, self.diffuse_bind_group.as_ref().unwrap(), &[]);
                render_pass.set_vertex_buffer(0, *vertex_buffer.slice(..));
                render_pass.set_index_buffer(*index_buffer.slice(..), IndexFormat::Uint16);

                render_pass.draw_indexed(0..(INDICES.len() as u32), 0, 0..1);
            }
        }

        Ok(())
    }
}

pub fn handle_background_image(
    cam_query: Query<&mut BackgroundCamera>,
    mut image: ResMut<BackgroundImage>,
) {
    for background_camera in cam_query.iter() {
        while let Some(img) = background_camera.image_rx.drain().last() {
            image.0 = img;
        }
    }
}
