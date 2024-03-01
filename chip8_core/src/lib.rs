const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 16;
const STACK_SIZE: usize= = 16;
const NUM_KEYS: usize = 16;
const START_ADDR: u16 = 0x200;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

pub struct Emulator{
    pc: hu16,
    ram: [u8; RAM_SIZE],
    screen: [bool; SCREEN_HEIGHT*SCREEN_WIDTH],
    v: [u8; NUM_REGS],
    index_reg: u16,
    stack: [u16; STACK_SIZE],
    sound_timer: u8,
    delay_timer: u8,
    keys: [bool; NUM_KEYS],
}
