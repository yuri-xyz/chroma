macro_rules! define_named_enum {
  (
    $(#[$enum_meta:meta])*
    $vis:vis enum $name:ident {
      $(
        $(#[$variant_meta:meta])*
        $variant:ident => {
          full: $full_name:literal,
          display: $display_name:literal,
          aliases: [$($alias:literal),* $(,)?]
        }
      ),+ $(,)?
    },
    error_label: $error_label:literal
  ) => {
    #[repr(u32)]
    $(#[$enum_meta])*
    $vis enum $name {
      $(
        $(#[$variant_meta])*
        $variant,
      )+
    }

    impl $name {
      pub const fn all() -> &'static [Self] {
        &[
          $(Self::$variant),+
        ]
      }

      pub fn full_name(self) -> &'static str {
        const FULL_NAMES: &[&str] = &[
          $($full_name),+
        ];

        FULL_NAMES[self as usize]
      }

      pub fn name(self) -> &'static str {
        const DISPLAY_NAMES: &[&str] = &[
          $($display_name),+
        ];

        DISPLAY_NAMES[self as usize]
      }

      pub fn to_u32(self) -> u32 {
        self as u32
      }

      pub fn next(self) -> Self {
        let all = Self::all();
        let next_index = (self as usize + 1) % all.len();

        all[next_index]
      }

      pub fn previous(self) -> Self {
        let all = Self::all();
        let previous_index = (self as usize + all.len() - 1) % all.len();

        all[previous_index]
      }
    }

    impl std::str::FromStr for $name {
      type Err = String;

      fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
          $(
            $full_name $(| $alias)* => Ok(Self::$variant),
          )+
          _ => Err(format!("Unknown {}: {}", $error_label, s)),
        }
      }
    }
  };
}

mod color_mode;
mod palette_type;
mod pattern_type;
mod randomizer;
mod shader_params;

pub use color_mode::ColorMode;
pub use palette_type::PaletteType;
pub use pattern_type::PatternType;
pub use shader_params::ShaderParams;
