use std::{
    array,
    num::{NonZero, NonZeroU64},
    result,
};

// nxm * mxp = n*p
// 2x3* 3x2 = 2x2
use wgpu::util::DeviceExt;
#[tokio::main]
async fn main() {
    let a = [[3.0f32, 7., 9.], [8., 4.0, 1.0]];
    let b = [[46.0f32, 24.], [10., 7.], [9., 8.]];
    let mut c = [[0.0f32; 2]; 2];
    let c_gpu = c.clone();

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..3 {
                c[i][j] += a[i][k] * b[k][j]
            }
        }
    }

    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .unwrap();

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_defaults(),
                memory_hints: wgpu::MemoryHints::MemoryUsage,
            },
            None,
        )
        .await
        .unwrap();
    let module = device.create_shader_module(wgpu::include_wgsl!("../matmul.wgsl"));
    let array_a_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("array_a_buffer"),
        contents: bytemuck::cast_slice(&a),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let array_b_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("array_b_buffer"),
        contents: bytemuck::cast_slice(&b),
        usage: wgpu::BufferUsages::STORAGE,
    });

    let array_c_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("array_c_buffer"),
        contents: bytemuck::cast_slice(&c_gpu),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
    });
    let read_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("read_buffer"),
        size: array_c_buffer.size(),
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },

                    has_dynamic_offset: false,
                    min_binding_size: Some(NonZeroU64::new(array_a_buffer.size()).unwrap()),
                },

                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },

                    has_dynamic_offset: false,
                    min_binding_size: Some(NonZeroU64::new(array_b_buffer.size()).unwrap()),
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },

                    has_dynamic_offset: false,
                    min_binding_size: Some(NonZeroU64::new(array_c_buffer.size()).unwrap()),
                },
                count: None,
            },
        ],
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: array_a_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: array_b_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: array_c_buffer.as_entire_binding(),
            },
        ],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        module: &module,
        entry_point: Some("main"),
        compilation_options: wgpu::PipelineCompilationOptions::default(),
        cache: None,
    });

    // records commands sent to gpu
    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
        label: None,
        timestamp_writes: None,
    });

    compute_pass.set_pipeline(&pipeline);
    compute_pass.set_bind_group(0, &bind_group, &[]);
    compute_pass.dispatch_workgroups(1, 1, 1);
    drop(compute_pass);
    encoder.copy_buffer_to_buffer(&array_c_buffer, 0, &read_buffer, 0, array_c_buffer.size());

    let command_buffer = encoder.finish();
    queue.submit([command_buffer]);
    let buffer_slice = read_buffer.slice(..);
    buffer_slice.map_async(wgpu::MapMode::Read, |_| {});
    device.poll(wgpu::Maintain::Wait);
    let data = buffer_slice.get_mapped_range();
    let result: &[f32] = bytemuck::cast_slice(&data);
    println!("CPU RESULT: {c:?}");
    println!("GPU RESULT: {result:?}");
}
