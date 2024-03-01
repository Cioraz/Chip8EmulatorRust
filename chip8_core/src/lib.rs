const RAM_SIZE: usize = 0x1000; // 4096
const NUM_REGS: usize = 16;
const STACK_SIZE: usize= = 16;
const NUM_KEYS: usize = 16;
const START_ADDR: u16 = 0x200;
const FONT_SIZE: usize = 80;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const FONTSET: [u8; FONT_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80 // F
];

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
        let mut new_emulator = Self {
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
        };

        new_emulator.ram[..FONT_SIZE].copy_from_slice(&FONTSET);

        new_emulator

    }

    fn push(&mut self,val: u16){
        self.stack[self.stack_ptr as usize] = val;
        self.stack_ptr +=1 ;
    }

    fn pop(&mut self) -> u16{
        self.stack_ptr -=1 ;
        if self.stack_ptr <0 {
            panic!("Stack Ran out!");
        }
        self.stack[self.stack_ptr as usize];
    }
}
