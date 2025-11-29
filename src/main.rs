mod aimbot;
mod config;
mod esp;
mod memory;
mod offsets;
mod overlay;

use aimbot::Aimbot;
use esp::{ESP, EspBox};
use memory::player::Entity;
use std::thread::sleep;
use std::time::Duration;
use winapi::um::winuser::{GetAsyncKeyState, VK_RBUTTON};

fn main() {
    let process_info = memory::ProcessInfo::attach_to_process("ac_client.exe").unwrap();

    println!("Attached to process: ac_client.exe");

    let esp = ESP::new(&process_info);

    sleep(Duration::from_secs(1));

    loop {
        let player_ptr = process_info.get_player_ptr();
        let number_of_entities = process_info.get_number_of_entities();
        let entity_list = process_info.get_entity_list();
        let player = process_info.get_player(player_ptr);

        if let Some(player) = player {
            let mut closest_distance = f32::MAX;
            let mut closest_entity: Option<Entity> = None;
            // let mut esp_boxes: Vec<EspBox> = Vec::new();

            for i in 1..number_of_entities + 1 {
                let entity_ptr = process_info.get_entity(entity_list, i);

                if let Some(entity) = entity_ptr {
                    // Skip dead entities
                    if !entity.is_alive() {
                        continue;
                    }

                    esp.process(&entity);

                    // Calculate distance using head positions for more accurate aiming
                    let delta_x = entity.position.x - player.position.x;
                    let delta_y = entity.position.y - player.position.y;
                    let delta_z = entity.position.z - player.position.z;

                    let distance =
                        ((delta_x * delta_x) + (delta_y * delta_y) + (delta_z * delta_z)).sqrt();

                    if distance < closest_distance {
                        closest_distance = distance;
                        closest_entity = Some(entity);
                    }
                }
            }

            // Only aim if we have a valid closest entity
            if let Some(entity) = closest_entity {
                let key = unsafe { GetAsyncKeyState(VK_RBUTTON) } as u16;
                if key & 0x8000 != 0 {
                    let aimbot =
                        Aimbot::new(&process_info, entity.head_position, player.head_position);
                    aimbot.process();
                }
            }
        }
    }
}
