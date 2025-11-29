use crate::config::math::Vec2;
use crate::config::math::Vec3;
use crate::offsets::player as player_offsets;

#[derive(Debug, Clone, Copy)]
pub struct Entity {
    pub name: [u8; 10],
    pub health: u32,
    pub position: Vec3,
    pub view_angles: Vec2,
    pub head_position: Vec3,
    pub feet_position: Vec3,
    pub team_id: u8,
}

impl Entity {
    pub fn from_bytes(buffer: &[u8]) -> Self {
        Self {
            name: Self::read_value::<[u8; 10]>(buffer, player_offsets::NAME),
            health: Self::read_value::<u32>(buffer, player_offsets::HEALTH),
            position: Vec3 {
                x: Self::read_value::<f32>(buffer, player_offsets::POS_X),
                y: Self::read_value::<f32>(buffer, player_offsets::POS_Y),
                z: Self::read_value::<f32>(buffer, player_offsets::POS_Z),
            },
            view_angles: Vec2 {
                x: Self::read_value::<f32>(buffer, player_offsets::YAW),
                y: Self::read_value::<f32>(buffer, player_offsets::PITCH),
            },
            head_position: Vec3 {
                x: Self::read_value::<f32>(buffer, player_offsets::HEAD_POS_X),
                y: Self::read_value::<f32>(buffer, player_offsets::HEAD_POS_Y),
                z: Self::read_value::<f32>(buffer, player_offsets::HEAD_POS_Z),
            },
            feet_position: Vec3 {
                x: Self::read_value::<f32>(buffer, player_offsets::FEET_POS_X),
                y: Self::read_value::<f32>(buffer, player_offsets::FEET_POS_Y),
                z: Self::read_value::<f32>(buffer, player_offsets::FEET_POS_Z),
            },
            team_id: Self::read_value::<u8>(buffer, player_offsets::TEAM_ID),
        }
    }

    pub fn read_value<T: Copy>(buffer: &[u8], offset: usize) -> T {
        unsafe {
            let ptr = buffer.as_ptr().add(offset) as *const T;

            ptr.read_unaligned()
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0 && self.health <= 100
    }
}
