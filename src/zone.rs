use glam::Vec2;
use crate::config::ZoneConfig;

pub enum GlideDirection {
    Left,
    Right,
    Up,
    Down,
}

pub struct GlideZone {
    pub glide_direction: Vec2,
    pub edge_start: f32,
    pub edge_end: f32,
    pub glide_speed: f32
}

impl GlideZone {
    pub fn from_config(direction: GlideDirection, config: &ZoneConfig) -> Self {
        match direction {
            GlideDirection::Left => Self {
                glide_direction: Vec2::new(-1.0, 0.0),
                edge_start: config.zone_size,
                edge_end: config.full_speed_at,
                glide_speed: config.speed,
            },
            GlideDirection::Right => Self {
                glide_direction: Vec2::new(1.0, 0.0),
                edge_start: 1.0 - config.zone_size,
                edge_end: 1.0 - config.full_speed_at,
                glide_speed: config.speed,
            },
            GlideDirection::Up => Self {
                glide_direction: Vec2::new(0.0, -1.0),
                edge_start: config.zone_size,
                edge_end: config.full_speed_at,
                glide_speed: config.speed,
            },
            GlideDirection::Down => Self {
                glide_direction: Vec2::new(0.0, 1.0),
                edge_start: 1.0 - config.zone_size,
                edge_end: 1.0 - config.full_speed_at,
                glide_speed: config.speed,
            },
        }
    }

    pub fn compute_activation_factor(&self, normalized_touch :Vec2 ) -> f32 {
        let aligned_touch = self.glide_direction.dot( normalized_touch );
        
        let gradient_range = self.edge_end - self.edge_start;
        let aligned_grad_start = self.glide_direction.dot( Vec2::ONE *self.edge_start );
        let activation =
            if gradient_range == 0.0 {
                // threshold activation
                (aligned_touch - aligned_grad_start).signum().clamp(0.0, 1.0)
            }
            else {
                // inverse-lerp in the glide_direction-aligned space
                ((aligned_touch - aligned_grad_start) /gradient_range.abs()).clamp(0.0, 1.0)
            }
        ;
        return activation;
    }
    
    pub fn compute_glide(&self, normalized_touch :Vec2 ) -> Vec2 {
        return self.compute_activation_factor(normalized_touch) *self.glide_speed *self.glide_direction;
    }
}