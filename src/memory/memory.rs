use proc_mem::{ProcMemError, Process};

use crate::config::math::Vec2;
use crate::config::math::ViewMatrix;
use crate::memory::player::Entity;
use crate::offsets::globals;

#[derive(Debug)]
pub struct ProcessInfo {
    pub base_address: usize,
    pub process: Process,
}

impl ProcessInfo {
    pub fn attach_to_process(name: &str) -> Result<ProcessInfo, ProcMemError> {
        let process = match Process::with_name(name) {
            Ok(process) => process,
            Err(_) => panic!("Failed to attach to process: {}", name),
        };
        let module = match process.module("ac_client.exe") {
            Ok(module) => module,
            Err(_) => panic!("Failed to get module: {}", name),
        };

        let base_address: usize = module.base_address();

        Ok(ProcessInfo {
            base_address: base_address,
            process: process,
        })
    }

    pub fn get_player_ptr(&self) -> usize {
        self.process
            .read_mem::<usize>(self.base_address + globals::PLAYER_PTR)
            .expect("Failed to read player pointer")
    }

    pub fn get_number_of_entities(&self) -> usize {
        self.process
            .read_mem::<usize>(self.base_address + globals::N_ENT)
            .expect("Failed to read number of entities")
            - 1
    }

    pub fn get_entity_list(&self) -> usize {
        self.process
            .read_mem::<usize>(self.base_address + globals::ENT_LIST_PTR)
            .expect("Failed to read entity list")
    }

    pub fn get_player(&self, player_ptr: usize) -> Option<Entity> {
        let mut buffer: Vec<u8> = vec![0; 512];

        if !self
            .process
            .read_bytes(player_ptr, buffer.as_mut_ptr() as *mut u8, 512)
        {
            return None;
        }

        Some(Entity::from_bytes(&buffer))
    }

    pub fn get_entity(&self, entity_list: usize, index: usize) -> Option<Entity> {
        let entity_ptr = self
            .process
            .read_mem::<usize>(entity_list + index * 4)
            .expect("Failed to read entity");

        self.get_player(entity_ptr)
    }

    pub fn get_window_dimensions(&self) -> Vec2 {
        let width = self
            .process
            .read_mem::<u32>(self.base_address + globals::WINDOW_WIDTH)
            .expect("Failed to read window width");
        let height = self
            .process
            .read_mem::<u32>(self.base_address + globals::WINDOW_HEIGHT)
            .expect("Failed to read window height");

        Vec2 {
            x: width as f32,
            y: height as f32,
        }
    }

    pub fn get_view_matrix(&self) -> ViewMatrix {
        let mut buffer: Vec<u8> = vec![0; 16 * 4];
        self.process
            .read_bytes(globals::VIEW_MATRIX, buffer.as_mut_ptr() as *mut u8, 16 * 4);
        ViewMatrix::from_bytes(&buffer)
    }
}
