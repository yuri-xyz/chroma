use anyhow::Result;
use std::io::Write;

use super::uniforms::ShaderUniforms;

pub struct ShaderPipeline {
  device: wgpu::Device,
  queue: wgpu::Queue,
  compute_pipeline: wgpu::ComputePipeline,
  bind_group: wgpu::BindGroup,
  uniform_buffer: wgpu::Buffer,
  output_buffer: wgpu::Buffer,
  staging_buffer: wgpu::Buffer,
  width: u32,
  height: u32,
}

impl ShaderPipeline {
  pub async fn new<W: Write>(
    width: u32,
    height: u32,
    custom_shader: Option<String>,
    debug_log: &mut W,
  ) -> Result<Self> {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
      backends: wgpu::Backends::PRIMARY,
      ..Default::default()
    });

    let adapter = instance
      .request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: None,
        force_fallback_adapter: false,
      })
      .await
      .ok_or_else(|| anyhow::anyhow!("Failed to find adapter"))?;

    let (device, queue) = adapter
      .request_device(
        &wgpu::DeviceDescriptor {
          label: Some("Shader Device"),
          required_features: wgpu::Features::empty(),
          required_limits: wgpu::Limits::default(),
          memory_hints: wgpu::MemoryHints::default(),
        },
        None,
      )
      .await?;

    // Load shader source - either custom or the compiled one from build.rs
    let shader_source: String = if let Some(custom) = custom_shader {
      writeln!(
        debug_log,
        "DEBUG: Using custom shader source ({} bytes)",
        custom.len()
      )?;
      custom
    } else {
      writeln!(debug_log, "DEBUG: Using built-in compiled shader")?;
      include_str!(concat!(env!("OUT_DIR"), "/compiled_shader.wgsl")).to_string()
    };

    writeln!(debug_log, "DEBUG: Creating shader module...")?;
    let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
      label: Some("Shader Module"),
      source: wgpu::ShaderSource::Wgsl(shader_source.into()),
    });
    writeln!(debug_log, "DEBUG: Shader module created successfully")?;

    let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
      label: Some("Uniform Buffer"),
      size: std::mem::size_of::<ShaderUniforms>() as u64,
      usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
      mapped_at_creation: false,
    });

    let buffer_size = (width * height * 4 * 4) as u64;

    let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
      label: Some("Output Buffer"),
      size: buffer_size,
      usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
      mapped_at_creation: false,
    });

    let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
      label: Some("Staging Buffer"),
      size: buffer_size,
      usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
      mapped_at_creation: false,
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
      label: Some("Bind Group Layout"),
      entries: &[
        wgpu::BindGroupLayoutEntry {
          binding: 0,
          visibility: wgpu::ShaderStages::COMPUTE,
          ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
          },
          count: None,
        },
        wgpu::BindGroupLayoutEntry {
          binding: 1,
          visibility: wgpu::ShaderStages::COMPUTE,
          ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Storage { read_only: false },
            has_dynamic_offset: false,
            min_binding_size: None,
          },
          count: None,
        },
      ],
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      label: Some("Bind Group"),
      layout: &bind_group_layout,
      entries: &[
        wgpu::BindGroupEntry {
          binding: 0,
          resource: uniform_buffer.as_entire_binding(),
        },
        wgpu::BindGroupEntry {
          binding: 1,
          resource: output_buffer.as_entire_binding(),
        },
      ],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: Some("Pipeline Layout"),
      bind_group_layouts: &[&bind_group_layout],
      push_constant_ranges: &[],
    });

    writeln!(debug_log, "DEBUG: Creating compute pipeline...")?;
    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
      label: Some("Compute Pipeline"),
      layout: Some(&pipeline_layout),
      module: &shader_module,
      entry_point: "main",
      compilation_options: Default::default(),
      cache: None,
    });
    writeln!(debug_log, "DEBUG: Compute pipeline created successfully")?;

    Ok(Self {
      device,
      queue,
      compute_pipeline,
      bind_group,
      uniform_buffer,
      output_buffer,
      staging_buffer,
      width,
      height,
    })
  }

  pub fn render(&self, uniforms: &ShaderUniforms) -> Result<Vec<u8>> {
    self
      .queue
      .write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(uniforms));

    let mut encoder = self
      .device
      .create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Compute Encoder"),
      });

    {
      let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
        label: Some("Compute Pass"),
        timestamp_writes: None,
      });

      compute_pass.set_pipeline(&self.compute_pipeline);
      compute_pass.set_bind_group(0, &self.bind_group, &[]);

      let workgroup_count_x = self.width.div_ceil(8);
      let workgroup_count_y = self.height.div_ceil(8);

      compute_pass.dispatch_workgroups(workgroup_count_x, workgroup_count_y, 1);
    }

    encoder.copy_buffer_to_buffer(
      &self.output_buffer,
      0,
      &self.staging_buffer,
      0,
      (self.width * self.height * 4 * 4) as u64,
    );

    self.queue.submit(Some(encoder.finish()));

    let buffer_slice = self.staging_buffer.slice(..);
    let (sender, receiver) = flume::unbounded();

    buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
      sender.send(result).ok();
    });

    self.device.poll(wgpu::Maintain::Wait);

    receiver.recv()??;

    let data = buffer_slice.get_mapped_range();
    let float_data: &[f32] = bytemuck::cast_slice(&data);
    let mut rgba_data = Vec::with_capacity((self.width * self.height * 4) as usize);

    for chunk in float_data.chunks(4) {
      rgba_data.push((chunk[0] * 255.0).min(255.0) as u8);
      rgba_data.push((chunk[1] * 255.0).min(255.0) as u8);
      rgba_data.push((chunk[2] * 255.0).min(255.0) as u8);
      rgba_data.push(255);
    }

    drop(data);
    self.staging_buffer.unmap();

    Ok(rgba_data)
  }

  pub fn width(&self) -> u32 {
    self.width
  }

  pub fn height(&self) -> u32 {
    self.height
  }
}
