use bevy::prelude::*;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_resource::{
    BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType,
    BufferBindingType, BufferInitDescriptor, BufferUsages, CachedComputePipelineId,
    CommandEncoderDescriptor, ComputePassDescriptor, ComputePipelineDescriptor, PipelineCache,
    ShaderStages, StorageTextureAccess, TextureFormat, TextureViewDescriptor, TextureViewDimension,
};
use bevy::render::renderer::{RenderDevice, RenderQueue};
use bevy::render::texture::GpuImage;
use bevy::render::{Extract, ExtractSchedule, Render, RenderApp, RenderSystems};
use bevy_falling_sand::prelude::{ParticleMap, WorldColorTexture};

use super::{GolSpawnBuffer, GolTextures, SHADER_ASSET_PATH};

// ── Render-world resources ──────────────────────────────────────────────

/// Marker resource inserted when `GolTextures` exists in the main world.
/// Gates the dispatch systems so they don't run before setup.
#[derive(Resource)]
struct GolReady;

#[derive(Resource, Default)]
struct GpuGolState {
    spawn_data: Vec<u32>,
    tex_a: Option<Handle<Image>>,
    tex_b: Option<Handle<Image>>,
    current_is_a: bool,
    world_color_handle: Option<Handle<Image>>,
    width: u32,
    height: u32,
}

#[derive(Resource)]
struct GolUpdatePipeline {
    bind_group_layout_descriptor: BindGroupLayoutDescriptor,
    pipeline_id: CachedComputePipelineId,
}

#[derive(Resource)]
struct GolSpawnPipeline {
    bind_group_layout_descriptor: BindGroupLayoutDescriptor,
    pipeline_id: CachedComputePipelineId,
}

pub(super) fn build_render_app(app: &mut App) {
    let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
        return;
    };

    render_app
        .init_resource::<GpuGolState>()
        .add_systems(ExtractSchedule, extract_gol_data)
        .add_systems(
            Render,
            (dispatch_gol_spawn, dispatch_gol_update)
                .chain()
                .run_if(resource_exists::<GolReady>)
                .in_set(RenderSystems::Queue)
                .after(RenderSystems::PrepareAssets),
        );
}

pub(super) fn finish_render_app(app: &mut App) {
    let asset_server = app.world().resource::<AssetServer>();
    let shader: Handle<Shader> = asset_server.load(SHADER_ASSET_PATH);

    let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
        return;
    };

    // ── Update pipeline: gol_in(read) + gol_out(write) + world_color(read) + params(uniform)
    let update_entries = vec![
        BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::StorageTexture {
                access: StorageTextureAccess::ReadOnly,
                format: TextureFormat::Rgba8Unorm,
                view_dimension: TextureViewDimension::D2,
            },
            count: None,
        },
        BindGroupLayoutEntry {
            binding: 1,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::StorageTexture {
                access: StorageTextureAccess::WriteOnly,
                format: TextureFormat::Rgba8Unorm,
                view_dimension: TextureViewDimension::D2,
            },
            count: None,
        },
        BindGroupLayoutEntry {
            binding: 2,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::StorageTexture {
                access: StorageTextureAccess::ReadOnly,
                format: TextureFormat::Rgba8Unorm,
                view_dimension: TextureViewDimension::D2,
            },
            count: None,
        },
        BindGroupLayoutEntry {
            binding: 3,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        },
    ];

    let update_bgl = BindGroupLayoutDescriptor::new("gol_update_bgl", &update_entries);

    // ── Spawn pipeline: spawns(storage) + gol_tex(write) + spawn_params(uniform)
    let spawn_entries = vec![
        BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        },
        BindGroupLayoutEntry {
            binding: 1,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::StorageTexture {
                access: StorageTextureAccess::WriteOnly,
                format: TextureFormat::Rgba8Unorm,
                view_dimension: TextureViewDimension::D2,
            },
            count: None,
        },
        BindGroupLayoutEntry {
            binding: 2,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        },
    ];

    let spawn_bgl = BindGroupLayoutDescriptor::new("gol_spawn_bgl", &spawn_entries);

    let pipeline_cache = render_app.world().resource::<PipelineCache>();

    let update_pipeline_id = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
        label: Some("gol_update_pipeline".into()),
        layout: vec![update_bgl.clone()],
        shader: shader.clone(),
        shader_defs: vec![],
        entry_point: Some("update".into()),
        push_constant_ranges: vec![],
        zero_initialize_workgroup_memory: true,
    });

    let spawn_pipeline_id = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
        label: Some("gol_spawn_pipeline".into()),
        layout: vec![spawn_bgl.clone()],
        shader,
        shader_defs: vec![],
        entry_point: Some("spawn".into()),
        push_constant_ranges: vec![],
        zero_initialize_workgroup_memory: true,
    });

    render_app.insert_resource(GolUpdatePipeline {
        bind_group_layout_descriptor: update_bgl,
        pipeline_id: update_pipeline_id,
    });
    render_app.insert_resource(GolSpawnPipeline {
        bind_group_layout_descriptor: spawn_bgl,
        pipeline_id: spawn_pipeline_id,
    });
}

fn extract_gol_data(
    mut commands: Commands,
    mut gpu: ResMut<GpuGolState>,
    spawn_buf: Extract<Option<Res<GolSpawnBuffer>>>,
    textures: Extract<Option<Res<GolTextures>>>,
    color_tex: Extract<Option<Res<WorldColorTexture>>>,
    map: Extract<Option<Res<ParticleMap>>>,
) {
    let Some(tex) = textures.as_ref() else {
        commands.remove_resource::<GolReady>();
        return;
    };

    commands.insert_resource(GolReady);

    gpu.tex_a = Some(tex.a.clone());
    gpu.tex_b = Some(tex.b.clone());
    gpu.current_is_a = tex.current_is_a;

    if let Some(buf) = spawn_buf.as_ref() {
        gpu.spawn_data.clone_from(&buf.positions);
    }

    gpu.world_color_handle = color_tex.as_ref().map(|t| t.0.clone());

    if let Some(m) = map.as_deref() {
        gpu.width = m.width();
        gpu.height = m.height();
    }
}

fn dispatch_gol_spawn(
    gpu: Res<GpuGolState>,
    pipeline_res: Res<GolSpawnPipeline>,
    pipeline_cache: Res<PipelineCache>,
    gpu_images: Res<RenderAssets<GpuImage>>,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
) {
    if gpu.spawn_data.is_empty() {
        return;
    }

    // Spawn into the current INPUT texture so the update step sees them.
    let input_handle = if gpu.current_is_a {
        gpu.tex_a.as_ref()
    } else {
        gpu.tex_b.as_ref()
    };
    let Some(handle) = input_handle else { return };
    let Some(gpu_image) = gpu_images.get(handle) else {
        return;
    };
    let Some(pipeline) = pipeline_cache.get_compute_pipeline(pipeline_res.pipeline_id) else {
        return;
    };

    let tex_view = gpu_image.texture.create_view(&TextureViewDescriptor {
        format: Some(TextureFormat::Rgba8Unorm),
        ..default()
    });

    // Pack spawn positions into a storage buffer
    let mut data_bytes = Vec::with_capacity(gpu.spawn_data.len() * 4);
    for &packed in &gpu.spawn_data {
        data_bytes.extend_from_slice(&packed.to_le_bytes());
    }

    let storage_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("gol_spawn_buffer"),
        contents: &data_bytes,
        usage: BufferUsages::STORAGE,
    });

    let count = gpu.spawn_data.len() as u32;
    let mut param_bytes = [0u8; 16];
    param_bytes[0..4].copy_from_slice(&count.to_le_bytes());

    let uniform_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("gol_spawn_params"),
        contents: &param_bytes,
        usage: BufferUsages::UNIFORM,
    });

    let bind_group_layout =
        pipeline_cache.get_bind_group_layout(&pipeline_res.bind_group_layout_descriptor);

    let bind_group = render_device.create_bind_group(
        "gol_spawn_bind_group",
        &bind_group_layout,
        &[
            BindGroupEntry {
                binding: 0,
                resource: storage_buffer.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 1,
                resource: BindingResource::TextureView(&tex_view),
            },
            BindGroupEntry {
                binding: 2,
                resource: uniform_buffer.as_entire_binding(),
            },
        ],
    );

    let mut encoder = render_device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("gol_spawn_encoder"),
    });

    {
        let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some("gol_spawn_pass"),
            ..default()
        });
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.dispatch_workgroups(count.div_ceil(64), 1, 1);
    }

    render_queue.submit(std::iter::once(encoder.finish()));
}

fn dispatch_gol_update(
    gpu: Res<GpuGolState>,
    pipeline_res: Res<GolUpdatePipeline>,
    pipeline_cache: Res<PipelineCache>,
    gpu_images: Res<RenderAssets<GpuImage>>,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
) {
    let (Some(handle_a), Some(handle_b), Some(world_handle)) =
        (&gpu.tex_a, &gpu.tex_b, &gpu.world_color_handle)
    else {
        return;
    };

    let Some(pipeline) = pipeline_cache.get_compute_pipeline(pipeline_res.pipeline_id) else {
        return;
    };

    let Some(img_a) = gpu_images.get(handle_a) else {
        return;
    };
    let Some(img_b) = gpu_images.get(handle_b) else {
        return;
    };
    let Some(img_world) = gpu_images.get(world_handle) else {
        return;
    };

    let (input_img, output_img) = if gpu.current_is_a {
        (img_a, img_b)
    } else {
        (img_b, img_a)
    };

    let input_view = input_img.texture.create_view(&TextureViewDescriptor {
        format: Some(TextureFormat::Rgba8Unorm),
        ..default()
    });
    let output_view = output_img.texture.create_view(&TextureViewDescriptor {
        format: Some(TextureFormat::Rgba8Unorm),
        ..default()
    });
    let world_view = img_world.texture.create_view(&TextureViewDescriptor {
        format: Some(TextureFormat::Rgba8Unorm),
        ..default()
    });

    let mut param_bytes = [0u8; 8];
    param_bytes[0..4].copy_from_slice(&gpu.width.to_le_bytes());
    param_bytes[4..8].copy_from_slice(&gpu.height.to_le_bytes());

    let uniform_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("gol_update_params"),
        contents: &param_bytes,
        usage: BufferUsages::UNIFORM,
    });

    let bind_group_layout =
        pipeline_cache.get_bind_group_layout(&pipeline_res.bind_group_layout_descriptor);

    let bind_group = render_device.create_bind_group(
        "gol_update_bind_group",
        &bind_group_layout,
        &[
            BindGroupEntry {
                binding: 0,
                resource: BindingResource::TextureView(&input_view),
            },
            BindGroupEntry {
                binding: 1,
                resource: BindingResource::TextureView(&output_view),
            },
            BindGroupEntry {
                binding: 2,
                resource: BindingResource::TextureView(&world_view),
            },
            BindGroupEntry {
                binding: 3,
                resource: uniform_buffer.as_entire_binding(),
            },
        ],
    );

    let mut encoder = render_device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("gol_update_encoder"),
    });

    {
        let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some("gol_update_pass"),
            ..default()
        });
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        let wg_x = gpu.width.div_ceil(16);
        let wg_y = gpu.height.div_ceil(16);
        pass.dispatch_workgroups(wg_x, wg_y, 1);
    }

    render_queue.submit(std::iter::once(encoder.finish()));
}
