use bevy::prelude::*;

#[derive(Resource)]
pub struct CanvasState {
    pub zoom: f32,
    pub offset: Vec2,
    pub viewport_size: Vec2,
    pub stroke_counter: u64,
}

impl Default for CanvasState {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            offset: Vec2::ZERO,
            viewport_size: Vec2::new(1920.0, 1080.0),
            stroke_counter: 0,
        }
    }
}

impl CanvasState {
    pub fn world_to_screen(&self, world_pos: Vec2) -> Vec2 {
        (world_pos + self.offset) * self.zoom
    }

    pub fn screen_to_world(&self, screen_pos: Vec2) -> Vec2 {
        screen_pos / self.zoom - self.offset
    }

    pub fn get_visible_bounds(&self) -> (Vec2, Vec2) {
        let top_left = self.screen_to_world(Vec2::ZERO);
        let bottom_right = self.screen_to_world(self.viewport_size);
        (top_left, bottom_right)
    }
}
