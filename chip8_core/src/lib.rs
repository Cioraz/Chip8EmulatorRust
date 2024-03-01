const RAM_SIZE: usize = 0x1000; // 4096
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
    stack_ptr: u16,
    sound_timer: u8,
    delay_timer: u8,
    keys: [bool; NUM_KEYS],
}

impl Emulator{
    pub fn new() -> Self{
        pc: START_ADDR,
        ram: [0; RAM_SIZE],
        screen: [false; SCREEN_WIDTH*SCREEN_HEIGHT],
        v: [0; NUM_REGS],
        index_reg: 0,
        stack_ptr: 0,
        stack: [0; STACK_SIZE],
        keys: [false; NUM_KEYS],
        delay_timer: 0,
        sound_timer: 0,
    }
}
