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
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
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
    
    // Operates every cycle once
    pub fn tick(&mut self){
        // do the fetch here
        let opcode = self.fetch();
        // Decode this opcode
        // Execute the opcode giving ram and registers
        self.execute(opcode);
    }

    fn execute(&mut self,opcode: u16){
        let dig1 = (opcode & 0xF000) >> 12;
        let dig2 = (opcode & 0x0F00) >> 8;
        let dig2 = (opcode & 0x00F0) >> 4;
        let dig4 = (opcode & 0x000F);

        match (dig1,dig2,dig3,dig4){
            // Do nothing
            (0,0,0,0) => return, 

            // Set all the pixels back to false
            (0,0,0xE,0) =>  self.screen = [false; SCREEN_HEIGHT*SCREEN_WIDTH];

            // Return from subroutine
            (0,0,0xE,0xE) => {
                let ret_addr = self.pop();
                self.pc = ret_addr;
            },

            // Jump to this memory location
            (1,_,_,_) => {
                let nnn = opcode & 0x0FFF;
                self.pc = nnn;
            }
        
            
            // If opcode is unimplemented
            (_,_,_,_) => unimplemented!("Unimplemented Code {}",opcode),
        }
    }

    fn fetch(&mut self) -> u16{
        // here we are fetching the higher byte so lower byte is 0 extended here
        let high_bytes = self.ram[self.pc as usize] as u16;
        // Fetches the next instruction and takes lower byte of that instruction same as previous
        // it has 0 extension
        let lower_bytes = self.ram[(self.pc+1) as usize] as u16;
        let opcode = (high_bytes<<8) | lower_bytes; // This converts the little endian to big
        // endian and gets the 2 byte long opcode
        self.pc +=2;
        opcode
    }

    pub fn tick_timers(&mut self){
        if self.delay_timer >0{
            self.delay_timer -=1;
        }

        if self.sound_timer >0{
            if self.sound_timer == 1{
                // Sound here
            }
            self.sound_timer -= 1;
        }
    }


}
