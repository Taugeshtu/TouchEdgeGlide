use glam::Vec2;

pub struct GlideZone {
    pub glide_direction: Vec2,
    pub edge_start: f32,
    pub edge_end: f32,
    pub glide_speed: f32
}

impl GlideZone {
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