use chroma::ascii::{AsciiConverter, AsciiPalette};
use chroma::params::ShaderParams;
use chroma::shader::ShaderUniforms;

#[test]
fn test_params_default() {
  let params = ShaderParams::default();

  assert_eq!(params.time, 0.0);
  assert_eq!(params.frequency, 10.0);
}

#[test]
fn test_uniforms_conversion() {
  let params = ShaderParams::default();
  let uniforms = ShaderUniforms::from_params(&params);

  assert_eq!(uniforms.time, 0.0);
  assert_eq!(uniforms.frequency, 10.0);
}

#[test]
fn test_ascii_conversion() {
  let converter = AsciiConverter::default();
  let pixels = vec![255u8; 16];
  let result = converter.convert_frame(&pixels, 2, 2);

  assert_eq!(result.len(), 2);
  assert_eq!(result[0].len(), 2);
}

#[test]
fn test_ascii_palette() {
  let palette = AsciiPalette::standard();

  assert_eq!(palette.get_character(0.0), ' ');
  assert_eq!(palette.get_character(1.0), '@');
}
