// SPDX-License-Identifier: CC-BY-SA-4.0
//! SigmaOS-brand — Visual identity, themes, logos, color palettes,
//! and desktop persona assets for SigmaOS.
//!
//! Provides:
//! - Persona brand specifications (Omarchy, Mint, Arch, Sovereign)
//! - Color palette representations and hex/RGB parsing
//! - Icon and wallpaper asset resolution with fallbacks
//! - SVG brand logo generators
//!
//! Principles: fail-fast, no unsafe code, deterministic unit-testable logic.

#![forbid(unsafe_code)]

use std::collections::HashMap;
use std::fmt;

// ── Errors ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BrandError {
    InvalidHexColor(String),
    PaletteNotFound(String),
    AssetNotFound(String),
    PersonaNotFound(String),
    EmptyName,
}

impl fmt::Display for BrandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidHexColor(s) => write!(f, "invalid hex color: '{s}' (expected #RRGGBB)"),
            Self::PaletteNotFound(s) => write!(f, "palette '{s}' not registered"),
            Self::AssetNotFound(s) => write!(f, "brand asset '{s}' not found"),
            Self::PersonaNotFound(s) => write!(f, "persona '{s}' not found"),
            Self::EmptyName => write!(f, "name cannot be empty"),
        }
    }
}

impl std::error::Error for BrandError {}

// ── Color & Palette ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Parse #RRGGBB or #RGB hex string.
    pub fn from_hex(hex: &str) -> Result<Self, BrandError> {
        let s = hex.strip_prefix('#').unwrap_or(hex);
        if s.len() == 6 {
            let r = u8::from_str_radix(&s[0..2], 16).map_err(|_| BrandError::InvalidHexColor(hex.to_string()))?;
            let g = u8::from_str_radix(&s[2..4], 16).map_err(|_| BrandError::InvalidHexColor(hex.to_string()))?;
            let b = u8::from_str_radix(&s[4..6], 16).map_err(|_| BrandError::InvalidHexColor(hex.to_string()))?;
            Ok(Self { r, g, b })
        } else if s.len() == 3 {
            let r_ch = s.chars().nth(0).unwrap();
            let g_ch = s.chars().nth(1).unwrap();
            let b_ch = s.chars().nth(2).unwrap();
            let r = u8::from_str_radix(&format!("{r_ch}{r_ch}"), 16)
                .map_err(|_| BrandError::InvalidHexColor(hex.to_string()))?;
            let g = u8::from_str_radix(&format!("{g_ch}{g_ch}"), 16)
                .map_err(|_| BrandError::InvalidHexColor(hex.to_string()))?;
            let b = u8::from_str_radix(&format!("{b_ch}{b_ch}"), 16)
                .map_err(|_| BrandError::InvalidHexColor(hex.to_string()))?;
            Ok(Self { r, g, b })
        } else {
            Err(BrandError::InvalidHexColor(hex.to_string()))
        }
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColorPalette {
    pub name: String,
    pub background: Color,
    pub foreground: Color,
    pub accent: Color,
    pub secondary: Color,
    pub border: Color,
    pub error: Color,
    pub success: Color,
}

impl ColorPalette {
    pub fn catppuccin_mocha() -> Self {
        Self {
            name: "catppuccin-mocha".into(),
            background: Color::new(30, 30, 46),   // #1e1e2e
            foreground: Color::new(205, 214, 244), // #cdd6f4
            accent: Color::new(137, 180, 250),     // #89b4fa
            secondary: Color::new(245, 194, 231),  // #f5c2e7
            border: Color::new(49, 50, 68),        // #313244
            error: Color::new(243, 139, 168),      // #f38ba8
            success: Color::new(166, 227, 161),    // #a6e3a1
        }
    }

    pub fn sovereign_dark() -> Self {
        Self {
            name: "sovereign-dark".into(),
            background: Color::new(10, 10, 15),   // #0a0a0f
            foreground: Color::new(220, 220, 230), // #dcdce6
            accent: Color::new(99, 102, 241),      // #6366f1
            secondary: Color::new(168, 85, 247),   // #a855f7
            border: Color::new(35, 35, 50),        // #232332
            error: Color::new(239, 68, 68),        // #ef4444
            success: Color::new(34, 197, 94),      // #22c55e
        }
    }

    pub fn mint_y() -> Self {
        Self {
            name: "mint-y".into(),
            background: Color::new(47, 52, 63),   // #2f343f
            foreground: Color::new(211, 218, 227), // #d3dae3
            accent: Color::new(141, 198, 63),      // #8dc63f
            secondary: Color::new(43, 146, 203),   // #2b92cb
            border: Color::new(64, 69, 82),        // #404552
            error: Color::new(204, 34, 34),        // #cc2222
            success: Color::new(141, 198, 63),     // #8dc63f
        }
    }

    pub fn arch_minimal() -> Self {
        Self {
            name: "arch-minimal".into(),
            background: Color::new(24, 25, 26),   // #18191a
            foreground: Color::new(240, 240, 240), // #f0f0f0
            accent: Color::new(23, 147, 209),      // #1793d1
            secondary: Color::new(51, 162, 220),   // #33a2dc
            border: Color::new(50, 52, 54),        // #323436
            error: Color::new(224, 86, 86),        // #e05656
            success: Color::new(86, 224, 132),     // #56e084
        }
    }
}

// ── Persona Brand Spec ───────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersonaBrand {
    pub name: String,
    pub display_name: String,
    pub tagline: String,
    pub default_palette: String,
    pub icon_name: String,
    pub wallpaper_default: String,
}

// ── Brand Manager ────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct BrandManager {
    palettes: HashMap<String, ColorPalette>,
    personas: HashMap<String, PersonaBrand>,
    assets: HashMap<String, String>, // asset_name -> URI/content
}

impl Default for BrandManager {
    fn default() -> Self {
        let mut mgr = Self {
            palettes: HashMap::new(),
            personas: HashMap::new(),
            assets: HashMap::new(),
        };

        // Register default palettes
        mgr.register_palette(ColorPalette::catppuccin_mocha());
        mgr.register_palette(ColorPalette::sovereign_dark());
        mgr.register_palette(ColorPalette::mint_y());
        mgr.register_palette(ColorPalette::arch_minimal());

        // Register core personas
        mgr.register_persona(PersonaBrand {
            name: "omarchy".into(),
            display_name: "SigmaOS Omarchy".into(),
            tagline: "Aesthetic Wayland-First Power-User Persona".into(),
            default_palette: "catppuccin-mocha".into(),
            icon_name: "distributor-logo-omarchy".into(),
            wallpaper_default: "assets/wallpapers/omarchy-catppuccin.png".into(),
        });

        mgr.register_persona(PersonaBrand {
            name: "sovereign".into(),
            display_name: "SigmaOS Sovereign Desktop".into(),
            tagline: "Ultra-Lightweight Microkernel-Native Graphical OS".into(),
            default_palette: "sovereign-dark".into(),
            icon_name: "sigmaos".into(),
            wallpaper_default: "assets/wallpapers/sigmaos-sovereign.png".into(),
        });

        mgr.register_persona(PersonaBrand {
            name: "mint".into(),
            display_name: "SigmaOS Mint".into(),
            tagline: "Ergonomic, Familiar Desktop Computing".into(),
            default_palette: "mint-y".into(),
            icon_name: "distributor-logo-mint".into(),
            wallpaper_default: "assets/wallpapers/mint-clean.png".into(),
        });

        mgr.register_persona(PersonaBrand {
            name: "arch".into(),
            display_name: "SigmaOS Arch".into(),
            tagline: "Minimalist, Rolling-Release Powerhouse".into(),
            default_palette: "arch-minimal".into(),
            icon_name: "distributor-logo-arch".into(),
            wallpaper_default: "assets/wallpapers/arch-dark.png".into(),
        });

        mgr
    }
}

impl BrandManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_palette(&mut self, palette: ColorPalette) {
        self.palettes.insert(palette.name.clone(), palette);
    }

    pub fn register_persona(&mut self, persona: PersonaBrand) {
        self.personas.insert(persona.name.clone(), persona);
    }

    pub fn register_asset(&mut self, name: impl Into<String>, path: impl Into<String>) {
        self.assets.insert(name.into(), path.into());
    }

    pub fn get_asset(&self, name: &str) -> Option<&str> {
        self.assets.get(name).map(|s| s.as_str())
    }

    pub fn get_palette(&self, name: &str) -> Result<&ColorPalette, BrandError> {
        self.palettes.get(name).ok_or_else(|| BrandError::PaletteNotFound(name.to_string()))
    }

    pub fn get_persona(&self, name: &str) -> Result<&PersonaBrand, BrandError> {
        self.personas.get(name).ok_or_else(|| BrandError::PersonaNotFound(name.to_string()))
    }

    pub fn list_personas(&self) -> Vec<&str> {
        self.personas.keys().map(|s| s.as_str()).collect()
    }

    pub fn list_palettes(&self) -> Vec<&str> {
        self.palettes.keys().map(|s| s.as_str()).collect()
    }

    /// Generate an SVG logo badge with the brand accent color.
    pub fn generate_svg_badge(&self, persona_name: &str) -> Result<String, BrandError> {
        let persona = self.get_persona(persona_name)?;
        let palette = self.get_palette(&persona.default_palette)?;

        let bg_hex = palette.background.to_hex();
        let fg_hex = palette.foreground.to_hex();
        let acc_hex = palette.accent.to_hex();

        Ok(format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="240" height="60" viewBox="0 0 240 60">
  <rect width="240" height="60" rx="8" fill="{bg_hex}"/>
  <circle cx="30" cy="30" r="16" fill="{acc_hex}"/>
  <text x="30" y="36" font-family="monospace" font-size="18" font-weight="bold" fill="{bg_hex}" text-anchor="middle">Σ</text>
  <text x="60" y="28" font-family="sans-serif" font-size="14" font-weight="bold" fill="{fg_hex}">{name}</text>
  <text x="60" y="44" font-family="sans-serif" font-size="10" fill="{acc_hex}">{tagline}</text>
</svg>"#,
            bg_hex = bg_hex,
            fg_hex = fg_hex,
            acc_hex = acc_hex,
            name = persona.display_name,
            tagline = persona.tagline,
        ))
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_hex_round_trip() {
        let c = Color::from_hex("#89b4fa").unwrap();
        assert_eq!(c.r, 137);
        assert_eq!(c.g, 180);
        assert_eq!(c.b, 250);
        assert_eq!(c.to_hex(), "#89b4fa");
    }

    #[test]
    fn test_color_short_hex() {
        let c = Color::from_hex("#fff").unwrap();
        assert_eq!(c, Color::new(255, 255, 255));
    }

    #[test]
    fn test_invalid_hex() {
        assert!(Color::from_hex("invalid").is_err());
        assert!(Color::from_hex("#12").is_err());
        assert!(Color::from_hex("#xyzxyz").is_err());
    }

    #[test]
    fn test_brand_manager_defaults() {
        let mgr = BrandManager::new();
        assert!(mgr.list_personas().contains(&"omarchy"));
        assert!(mgr.list_personas().contains(&"sovereign"));
        assert!(mgr.list_palettes().contains(&"catppuccin-mocha"));
        assert!(mgr.list_palettes().contains(&"sovereign-dark"));
    }

    #[test]
    fn test_persona_palette_lookup() {
        let mgr = BrandManager::new();
        let p = mgr.get_persona("omarchy").unwrap();
        assert_eq!(p.default_palette, "catppuccin-mocha");

        let pal = mgr.get_palette(&p.default_palette).unwrap();
        assert_eq!(pal.accent.to_hex(), "#89b4fa");
    }

    #[test]
    fn test_generate_svg_badge() {
        let mgr = BrandManager::new();
        let svg = mgr.generate_svg_badge("omarchy").unwrap();
        assert!(svg.starts_with("<svg"));
        assert!(svg.contains("SigmaOS Omarchy"));
        assert!(svg.contains("Σ"));
        assert!(svg.ends_with("</svg>"));
    }

    #[test]
    fn test_unknown_persona_fails() {
        let mgr = BrandManager::new();
        assert!(matches!(
            mgr.get_persona("windows95"),
            Err(BrandError::PersonaNotFound(_))
        ));
    }
}
