use crate::config::math::Vec3;
use crate::memory::ProcessInfo;
use crate::offsets::player as player_offsets;

pub struct Aimbot<'a> {
    pub process_info: &'a ProcessInfo,
    pub entity_head: Vec3,
    pub player_head: Vec3,
}

impl<'a> Aimbot<'a> {
    pub fn new(process_info: &'a ProcessInfo, entity_head: Vec3, player_head: Vec3) -> Self {
        Self {
            process_info,
            entity_head,
            player_head,
        }
    }

    pub fn process(self) -> bool {
        let (yaw, pitch) = Self::calculate_angle(self.entity_head, self.player_head);

        self.process_info.process.write_mem::<f32>(
            self.process_info.get_player_ptr() + player_offsets::YAW,
            yaw,
        );
        self.process_info.process.write_mem::<f32>(
            self.process_info.get_player_ptr() + player_offsets::PITCH,
            pitch,
        );

        true
    }

    fn calculate_angle(entity_head: Vec3, player_head: Vec3) -> (f32, f32) {
        let delta = Vec3 {
            x: entity_head.x - player_head.x,
            y: entity_head.y - player_head.y,
            z: entity_head.z - player_head.z,
        };

        /*
        Calculates the yaw angle using the arctangent function.

        yaw = arctan(y/x).to_degrees();

        the to_degrees() method converts the result from radians to degrees.
        */
        let mut yaw = delta.y.atan2(delta.x).to_degrees();
        if yaw < 0.0 {
            yaw += 360.0;
        }

        /*
        Calculates the pitch angle using the arctangent function.

        pitch = arctan(z/hyp).to_degrees();

        the to_degrees() method converts the result from radians to degrees.
        */
        let hyp = (delta.x * delta.x + delta.y * delta.y).sqrt();
        let pitch = delta.z.atan2(hyp).to_degrees();

        (yaw + 90.0, pitch) // add 90 degrees to the yaw angle to compensate for the camera angle.
    }
}
