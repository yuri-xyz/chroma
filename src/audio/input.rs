use anyhow::Result;
#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
#[cfg(feature = "audio")]
use cpal::{Device, Stream, StreamConfig};

pub struct AudioInput {
    #[cfg(feature = "audio")]
    _device: Device,
    #[cfg(feature = "audio")]
    _config: StreamConfig,
    #[cfg(feature = "audio")]
    _stream: Option<Stream>,
}

impl AudioInput {
    #[cfg(feature = "audio")]
    pub fn new() -> Result<Self> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or_else(|| anyhow::anyhow!("No input device available"))?;

        let config = device.default_input_config()?;

        Ok(Self {
            _device: device,
            _config: config.into(),
            _stream: None,
        })
    }

    #[cfg(not(feature = "audio"))]
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }

    #[cfg(feature = "audio")]
    pub fn start(&mut self) -> Result<()> {
        Ok(())
    }

    #[cfg(not(feature = "audio"))]
    pub fn start(&mut self) -> Result<()> {
        Err(anyhow::anyhow!("Audio feature not enabled"))
    }

    #[cfg(feature = "audio")]
    pub fn stop(&mut self) {}

    #[cfg(not(feature = "audio"))]
    pub fn stop(&mut self) {}
}
