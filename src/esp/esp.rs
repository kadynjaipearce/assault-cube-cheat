use crate::config::math::Vec2;
use crate::memory::ProcessInfo;
use crate::memory::player::Entity;

pub struct ESP<'a> {
    pub process_info: &'a ProcessInfo,
    pub screen_size: Vec2,
}

#[derive(Debug, Clone)]
pub struct EspBox {
    pub top_left: Vec2,
    pub bottom_right: Vec2,
}

impl<'a> ESP<'a> {
    pub fn new(process_info: &'a ProcessInfo) -> Self {
        let screen = process_info.get_window_dimensions();
        Self {
            process_info,
            screen_size: screen,
        }
    }

    pub fn process(&self, entity: &Entity) -> bool {
        let view_matrix = self.process_info.get_view_matrix();
        let mut head_screen = Vec2 { x: 0.0, y: 0.0 };
        let mut feet_screen = Vec2 { x: 0.0, y: 0.0 };

        if !view_matrix.world_to_screen(entity.head_position, &mut head_screen, self.screen_size) {
            return false;
        }

        if !view_matrix.world_to_screen(entity.feet_position, &mut feet_screen, self.screen_size) {
            return false;
        }

        println!(
            "head_screen: {:?}, feet_screen: {:?}",
            head_screen, feet_screen
        );

        let height = (feet_screen.y - head_screen.y).abs();
        let width = height * 0.45;

        return true;
    }
}
