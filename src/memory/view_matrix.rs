use crate::config::math::Vec2;
use crate::config::math::Vec3;
use crate::config::math::ViewMatrix;

impl ViewMatrix {
    pub fn from_bytes(buffer: &[u8]) -> Self {
        let mut matrix = [[0.0f32; 4]; 4];

        for row in 0..4 {
            for col in 0..4 {
                let offset = (col * 4 + row) * std::mem::size_of::<f32>();
                matrix[row][col] = Self::read_value::<f32>(buffer, offset);
            }
        }

        Self { m: matrix }
    }

    fn read_value<T: Copy>(buffer: &[u8], offset: usize) -> T {
        unsafe {
            let ptr = buffer.as_ptr().add(offset) as *const T;
            ptr.read_unaligned()
        }
    }

    pub fn world_to_screen(&self, position: Vec3, screen: &mut Vec2, screen_size: Vec2) -> bool {
        let clip_x = position.x * self.m[0][0]
            + position.y * self.m[0][1]
            + position.z * self.m[0][2]
            + self.m[0][3];
        let clip_y = position.x * self.m[1][0]
            + position.y * self.m[1][1]
            + position.z * self.m[1][2]
            + self.m[1][3];
        let clip_w = position.x * self.m[3][0]
            + position.y * self.m[3][1]
            + position.z * self.m[3][2]
            + self.m[3][3];

        if clip_w < 0.0001 {
            return false;
        }

        let ndc_x = clip_x / clip_w;
        let ndc_y = clip_y / clip_w;

        screen.x = (screen_size.x / 2.0) + (ndc_x * screen_size.x / 2.0);
        screen.y = (screen_size.y / 2.0) - (ndc_y * screen_size.y / 2.0);

        return true;
    }
}
