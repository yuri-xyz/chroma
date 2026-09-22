use std::io::Write;

use anyhow::Result;

use super::uniforms::ShaderUniforms;

pub struct ShaderPipeline {
  device: wgpu::Device,
  queue: wgpu::Queue,
  compute_pipeline: wgpu::ComputePipeline,
  bind_group_layout: wgpu::BindGroupLayout,
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
    ensure_non_empty_dimensions(width, height)?;

    let mut instance_descriptor = wgpu::InstanceDescriptor::new_without_display_handle();
    instance_descriptor.backends = wgpu::Backends::PRIMARY;
    let instance = wgpu::Instance::new(instance_descriptor);

    let adapter = instance
      .request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: None,
        force_fallback_adapter: false,
      })
      .await
      .map_err(|_| anyhow::anyhow!("Failed to find adapter"))?;

    let (device, queue) = adapter
      .request_device(&wgpu::DeviceDescriptor {
        label: Some("Shader Device"),
        required_features: wgpu::Features::empty(),
        required_limits: wgpu::Limits::default(),
        memory_hints: wgpu::MemoryHints::default(),
        ..Default::default()
      })
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
    // Without an error scope, wgpu reports invalid WGSL (e.g. a broken
    // --custom-shader) through its uncaptured-error handler, which panics.
    let validation_scope = device.push_error_scope(wgpu::ErrorFilter::Validation);
    let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
      label: Some("Shader Module"),
      source: wgpu::ShaderSource::Wgsl(shader_source.into()),
    });

    let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
      label: Some("Uniform Buffer"),
      size: std::mem::size_of::<ShaderUniforms>() as u64,
      usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
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

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: Some("Pipeline Layout"),
      bind_group_layouts: &[Some(&bind_group_layout)],
      immediate_size: 0,
    });

    writeln!(debug_log, "DEBUG: Creating compute pipeline...")?;
    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
      label: Some("Compute Pipeline"),
      layout: Some(&pipeline_layout),
      module: &shader_module,
      entry_point: Some("main"),
      compilation_options: Default::default(),
      cache: None,
    });

    if let Some(error) = validation_scope.pop().await {
      anyhow::bail!("Failed to compile shader: {error}");
    }
    writeln!(debug_log, "DEBUG: Compute pipeline created successfully")?;

    let (output_buffer, staging_buffer, bind_group) = Self::create_size_dependent_resources(
      &device,
      &bind_group_layout,
      &uniform_buffer,
      width,
      height,
    );

    Ok(Self {
      device,
      queue,
      compute_pipeline,
      bind_group_layout,
      bind_group,
      uniform_buffer,
      output_buffer,
      staging_buffer,
      width,
      height,
    })
  }

  /// Reallocate the output buffers for new dimensions, reusing the device and
  /// compiled pipeline.
  pub fn resize(&mut self, width: u32, height: u32) -> Result<()> {
    ensure_non_empty_dimensions(width, height)?;

    let (output_buffer, staging_buffer, bind_group) = Self::create_size_dependent_resources(
      &self.device,
      &self.bind_group_layout,
      &self.uniform_buffer,
      width,
      height,
    );

    self.output_buffer = output_buffer;
    self.staging_buffer = staging_buffer;
    self.bind_group = bind_group;
    self.width = width;
    self.height = height;

    Ok(())
  }

  fn create_size_dependent_resources(
    device: &wgpu::Device,
    bind_group_layout: &wgpu::BindGroupLayout,
    uniform_buffer: &wgpu::Buffer,
    width: u32,
    height: u32,
  ) -> (wgpu::Buffer, wgpu::Buffer, wgpu::BindGroup) {
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

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      label: Some("Bind Group"),
      layout: bind_group_layout,
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

    (output_buffer, staging_buffer, bind_group)
  }

  pub fn render(&self, uniforms: &ShaderUniforms) -> Result<Vec<u8>> {
    let mut rgba_data = Vec::new();
    self.render_into(uniforms, &mut rgba_data)?;
    Ok(rgba_data)
  }

  /// Render into `rgba_data`, replacing its contents, so the render loop can
  /// reuse one pixel buffer across frames.
  pub fn render_into(&self, uniforms: &ShaderUniforms, rgba_data: &mut Vec<u8>) -> Result<()> {
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

    self.device.poll(wgpu::PollType::wait_indefinitely())?;

    receiver.recv()??;

    let data = buffer_slice.get_mapped_range();
    let float_data: &[f32] = bytemuck::cast_slice(&data);
    rgba_data.clear();
    rgba_data.reserve(float_data.len());
    rgba_data.extend(float_data.as_chunks::<4>().0.iter().flat_map(
      |&[red, green, blue, _alpha]| {
        [
          float_channel_to_byte(red),
          float_channel_to_byte(green),
          float_channel_to_byte(blue),
          255,
        ]
      },
    ));

    drop(data);
    self.staging_buffer.unmap();

    Ok(())
  }

  pub fn width(&self) -> u32 {
    self.width
  }

  pub fn height(&self) -> u32 {
    self.height
  }
}

/// Truncating conversion; `as` saturates, so negative and NaN channels map to 0.
fn float_channel_to_byte(channel: f32) -> u8 {
  (channel * 255.0).min(255.0) as u8
}

fn ensure_non_empty_dimensions(width: u32, height: u32) -> Result<()> {
  if width == 0 || height == 0 {
    anyhow::bail!("Shader output dimensions must be non-zero, got {width}x{height}");
  }

  Ok(())
}
