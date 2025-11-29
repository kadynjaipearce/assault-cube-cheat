pub mod globals {
    pub const WINDOW_WIDTH: usize = 0x191EE0;
    pub const WINDOW_HEIGHT: usize = 0x191EDC;
    pub const N_ENT: usize = 0x18AC0C;
    pub const ENT_LIST_PTR: usize = 0x18AC04;
    pub const PLAYER_PTR: usize = 0x17E0A8;
    pub const VIEW_MATRIX: usize = 0x57DFD0; // 0x57E010
    pub const FOV: usize = 0x18A7CC;
}

pub mod player {
    pub const NAME: usize = 0x205;
    pub const HEALTH: usize = 0xEC;

    pub const YAW: usize = 0x34; // x rotation
    pub const PITCH: usize = 0x38; // y rotation

    pub const POS_X: usize = 0x2C;
    pub const POS_Y: usize = 0x30;
    pub const POS_Z: usize = 0x28;

    pub const HEAD_POS_X: usize = 0x4;
    pub const HEAD_POS_Y: usize = 0x8;
    pub const HEAD_POS_Z: usize = 0xC;

    pub const FEET_POS_X: usize = 0x28;
    pub const FEET_POS_Y: usize = 0x2C;
    pub const FEET_POS_Z: usize = 0x30;

    pub const TEAM_ID: usize = 0x30C;
}
