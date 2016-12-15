#![allow(dead_code)]

pub const VRAM_START: u32 = 0x00000000;
pub const VRAM_LENGTH: u32 = 0x00040000;
pub const VRAM_END: u32 = VRAM_START + VRAM_LENGTH - 1;

pub const LEFT_FRAME_BUFFER_0_START: u32 = 0x00000000;
pub const LEFT_FRAME_BUFFER_0_LENGTH: u32 = 0x00006000;
pub const LEFT_FRAME_BUFFER_0_END: u32 = LEFT_FRAME_BUFFER_0_START + LEFT_FRAME_BUFFER_0_LENGTH - 1;

pub const CHR_RAM_PATTERN_TABLE_0_START: u32 = 0x00006000;
pub const CHR_RAM_PATTERN_TABLE_0_LENGTH: u32 = 0x00002000;
pub const CHR_RAM_PATTERN_TABLE_0_END: u32 = CHR_RAM_PATTERN_TABLE_0_START + CHR_RAM_PATTERN_TABLE_0_LENGTH - 1;

pub const LEFT_FRAME_BUFFER_1_START: u32 = 0x00008000;
pub const LEFT_FRAME_BUFFER_1_LENGTH: u32 = 0x00006000;
pub const LEFT_FRAME_BUFFER_1_END: u32 = LEFT_FRAME_BUFFER_1_START + LEFT_FRAME_BUFFER_1_LENGTH - 1;

pub const CHR_RAM_PATTERN_TABLE_1_START: u32 = 0x0000e000;
pub const CHR_RAM_PATTERN_TABLE_1_LENGTH: u32 = 0x00002000;
pub const CHR_RAM_PATTERN_TABLE_1_END: u32 = CHR_RAM_PATTERN_TABLE_1_START + CHR_RAM_PATTERN_TABLE_1_LENGTH - 1;

pub const RIGHT_FRAME_BUFFER_0_START: u32 = 0x00010000;
pub const RIGHT_FRAME_BUFFER_0_LENGTH: u32 = 0x00006000;
pub const RIGHT_FRAME_BUFFER_0_END: u32 = RIGHT_FRAME_BUFFER_0_START + RIGHT_FRAME_BUFFER_0_LENGTH - 1;

pub const CHR_RAM_PATTERN_TABLE_2_START: u32 = 0x00016000;
pub const CHR_RAM_PATTERN_TABLE_2_LENGTH: u32 = 0x00002000;
pub const CHR_RAM_PATTERN_TABLE_2_END: u32 = CHR_RAM_PATTERN_TABLE_2_START + CHR_RAM_PATTERN_TABLE_2_LENGTH - 1;

pub const RIGHT_FRAME_BUFFER_1_START: u32 = 0x00018000;
pub const RIGHT_FRAME_BUFFER_1_LENGTH: u32 = 0x00006000;
pub const RIGHT_FRAME_BUFFER_1_END: u32 = RIGHT_FRAME_BUFFER_1_START + RIGHT_FRAME_BUFFER_1_LENGTH - 1;

pub const CHR_RAM_PATTERN_TABLE_3_START: u32 = 0x0001e000;
pub const CHR_RAM_PATTERN_TABLE_3_LENGTH: u32 = 0x00002000;
pub const CHR_RAM_PATTERN_TABLE_3_END: u32 = CHR_RAM_PATTERN_TABLE_3_START + CHR_RAM_PATTERN_TABLE_3_LENGTH - 1;

pub const BG_SEGMENTS_AND_WINDOW_PARAM_TABLE_START: u32 = 0x00020000;
pub const BG_SEGMENTS_AND_WINDOW_PARAM_TABLE_LENGTH: u32 = 0x0001d800;
pub const BG_SEGMENTS_AND_WINDOW_PARAM_TABLE_END: u32 = BG_SEGMENTS_AND_WINDOW_PARAM_TABLE_START + BG_SEGMENTS_AND_WINDOW_PARAM_TABLE_LENGTH - 1;

pub const WINDOW_ATTRIBS_START: u32 = 0x0003d800;
pub const WINDOW_ATTRIBS_LENGTH: u32 = 0x00000400;
pub const WINDOW_ATTRIBS_END: u32 = WINDOW_ATTRIBS_START + WINDOW_ATTRIBS_LENGTH - 1;

pub const COLUMN_TABLE_START: u32 = 0x0003dc00;
pub const COLUMN_TABLE_LENGTH: u32 = 0x00000400;
pub const COLUMN_TABLE_END: u32 = COLUMN_TABLE_START + COLUMN_TABLE_LENGTH - 1;

pub const OAM_START: u32 = 0x0003e000;
pub const OAM_LENGTH: u32 = 0x00002000;
pub const OAM_END: u32 = OAM_START + OAM_LENGTH - 1;

pub const DISPLAY_CONTROL_READ_REG: u32 = 0x0005f820;
pub const DISPLAY_CONTROL_WRITE_REG: u32 = 0x0005f822;

pub const CHR_RAM_PATTERN_TABLE_0_MIRROR_START: u32 = 0x00078000;
pub const CHR_RAM_PATTERN_TABLE_0_MIRROR_LENGTH: u32 = CHR_RAM_PATTERN_TABLE_0_LENGTH;
pub const CHR_RAM_PATTERN_TABLE_0_MIRROR_END: u32 = CHR_RAM_PATTERN_TABLE_0_MIRROR_START + CHR_RAM_PATTERN_TABLE_0_MIRROR_LENGTH - 1;

pub const CHR_RAM_PATTERN_TABLE_1_MIRROR_START: u32 = 0x0007a000;
pub const CHR_RAM_PATTERN_TABLE_1_MIRROR_LENGTH: u32 = CHR_RAM_PATTERN_TABLE_1_LENGTH;
pub const CHR_RAM_PATTERN_TABLE_1_MIRROR_END: u32 = CHR_RAM_PATTERN_TABLE_1_MIRROR_START + CHR_RAM_PATTERN_TABLE_1_MIRROR_LENGTH - 1;

pub const CHR_RAM_PATTERN_TABLE_2_MIRROR_START: u32 = 0x0007c000;
pub const CHR_RAM_PATTERN_TABLE_2_MIRROR_LENGTH: u32 = CHR_RAM_PATTERN_TABLE_2_LENGTH;
pub const CHR_RAM_PATTERN_TABLE_2_MIRROR_END: u32 = CHR_RAM_PATTERN_TABLE_2_MIRROR_START + CHR_RAM_PATTERN_TABLE_2_MIRROR_LENGTH - 1;

pub const CHR_RAM_PATTERN_TABLE_3_MIRROR_START: u32 = 0x0007e000;
pub const CHR_RAM_PATTERN_TABLE_3_MIRROR_LENGTH: u32 = CHR_RAM_PATTERN_TABLE_3_LENGTH;
pub const CHR_RAM_PATTERN_TABLE_3_MIRROR_END: u32 = CHR_RAM_PATTERN_TABLE_3_MIRROR_START + CHR_RAM_PATTERN_TABLE_3_MIRROR_LENGTH - 1;

pub enum MappedAddress {
    Vram(u32),

    DisplayControlReadReg,
    DisplayControlWriteReg,
}

pub fn map_address(addr: u32) -> MappedAddress {
    let addr = addr & 0x0007ffff;
    match addr {
        VRAM_START ... VRAM_END => MappedAddress::Vram(addr - VRAM_START),

        DISPLAY_CONTROL_READ_REG => MappedAddress::DisplayControlReadReg,
        DISPLAY_CONTROL_WRITE_REG => MappedAddress::DisplayControlWriteReg,

        CHR_RAM_PATTERN_TABLE_0_MIRROR_START ... CHR_RAM_PATTERN_TABLE_0_MIRROR_END =>
            MappedAddress::Vram(addr - CHR_RAM_PATTERN_TABLE_0_MIRROR_START + CHR_RAM_PATTERN_TABLE_0_START),
        CHR_RAM_PATTERN_TABLE_1_MIRROR_START ... CHR_RAM_PATTERN_TABLE_1_MIRROR_END =>
            MappedAddress::Vram(addr - CHR_RAM_PATTERN_TABLE_1_MIRROR_START + CHR_RAM_PATTERN_TABLE_1_START),
        CHR_RAM_PATTERN_TABLE_2_MIRROR_START ... CHR_RAM_PATTERN_TABLE_2_MIRROR_END =>
            MappedAddress::Vram(addr - CHR_RAM_PATTERN_TABLE_2_MIRROR_START + CHR_RAM_PATTERN_TABLE_2_START),
        CHR_RAM_PATTERN_TABLE_3_MIRROR_START ... CHR_RAM_PATTERN_TABLE_3_MIRROR_END =>
            MappedAddress::Vram(addr - CHR_RAM_PATTERN_TABLE_3_MIRROR_START + CHR_RAM_PATTERN_TABLE_3_START),

        _ => panic!("Unrecognized VIP addr: 0x{:08x}", addr)
    }
}