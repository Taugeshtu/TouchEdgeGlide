use serde::{Deserialize, Serialize};
use std::fs;
use directories::ProjectDirs;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ZoneConfig {
    pub speed: f32,
    pub zone_size: f32,
    pub full_speed_at: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EdgesConfig {
    pub left: ZoneConfig,
    pub right: ZoneConfig,
    pub top: ZoneConfig,
    pub bottom: ZoneConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FullConfig {
    pub generic: Option<ZoneConfig>,
    pub edges: Option<EdgesConfig>,
}

pub fn load_config() -> FullConfig {
    let proj_dirs = ProjectDirs::from("", "", "touch-edge-glide")
        .expect("Could not determine config directory");
    let config_dir = proj_dirs.config_dir();
    let config_path = config_dir.join("config.toml");

    if !config_path.exists() {
        fs::create_dir_all(config_dir).expect("Could not create config directory");
        let default_config = FullConfig {
            generic: Some(ZoneConfig {
                speed: 5.0,
                zone_size: 0.15,
                full_speed_at: 0.05,
            }),
            edges: None,
        };
        
        let toml_string = r#"# TouchEdgeGlide Configuration

# OPTION A: Simple (Applied to all edges)
[generic]
speed = 5.0
zone_size = 0.15      # Starts gliding when within 15% of any edge
full_speed_at = 0.05  # Reaches max speed when within 5% of any edge

# OPTION B: Advanced (Overrides generic if present)
# If you use this section, [generic] is ignored.
# [edges]
# left   = { speed = 5.0, zone_size = 0.15, full_speed_at = 0.05 }
# right  = { speed = 5.0, zone_size = 0.15, full_speed_at = 0.05 }
# top    = { speed = 5.0, zone_size = 0.15, full_speed_at = 0.05 }
# bottom = { speed = 5.0, zone_size = 0.15, full_speed_at = 0.05 }
"#;
        fs::write(&config_path, toml_string).expect("Could not write default config");
        println!("TouchEdgeGlide: Created default config at {:?}", config_path);
        return default_config;
    }

    let content = fs::read_to_string(config_path).expect("Could not read config file");
    toml::from_str(&content).expect("Could not parse config file")
}
