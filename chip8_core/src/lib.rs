use rand::random;

mod constants;
pub use constants::*;

pub struct Emulator{
    pc: u16,
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
    pub fn get_display(&self) -> &[bool]{
        &self.screen
    }

    pub fn keypress(&mut self,idx: usize,pressed: bool){
        self.keys[idx] = pressed;
    }

    pub fn load_game(&mut self,data: &[u8]){
        let start = START_ADDR as usize;
        let end = (START_ADDR as usize) + data.len();
        self.ram[start..end].copy_from_slice(data);
    }

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
        if self.stack.len() >16 { 
            panic!("Stack Overflow!");
        }
        self.stack[self.stack_ptr as usize] = val;
        self.stack_ptr +=1 ;
    }

    fn pop(&mut self) -> u16{
        self.stack_ptr -=1 ;
        if self.stack_ptr <0 {
            panic!("Stack Ran out!");
        }
        self.stack[self.stack_ptr as usize]
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
        let dig3 = (opcode & 0x00F0) >> 4;
        let dig4 = opcode & 0x000F;

        match (dig1,dig2,dig3,dig4){
            // Do nothing
            (0,0,0,0) => return, 

            // Set all the pixels back to false
            (0,0,0xE,0) =>  self.screen = [false; SCREEN_HEIGHT*SCREEN_WIDTH],

            // Return from subroutine
            (0,0,0xE,0xE) => {
                let ret_addr = self.pop();
                self.pc = ret_addr;
            },

            // Jump to this memory location
            (1,_,_,_) => {
                let nnn = opcode & 0x0FFF;
                self.pc = nnn;
            },
            
            // Add current pc to stack and jump to nnn
            (2,_,_,_) =>{
                let nnn = opcode & 0x0FFF;
                self.push(self.pc);
                self.pc = nnn;
            },
            

            // 3XNN : skip next if VX == NN
            (3,_,_,_) =>{
                let x = dig2 as usize;
                let nn = (opcode & 0x00FF) as u8;
                if self.v[x] == nn{
                    self.pc+=2;
                }
            },
    
            // 4XNN : skip next if VX!=NN
            (4,_,_,_) =>{
                let x = dig2 as usize;
                let nn = (opcode & 0x00FF) as u8;
                if self.v[x] != nn {
                    self.pc +=2;
                }
            },

            // 5XY0 : skip next if VX == VY
            (5,_,_,0) =>{
                let x = dig2 as usize;
                let y = dig3 as usize;
                if self.v[x] == self.v[y]{
                    self.pc +=2;
                }
            },
            
            // 6XNN : set V register specified by second dig to value given
            (6,_,_,_) =>{
                let x = dig2 as usize;
                let nn = (opcode & 0x00FF) as u8;
                self.v[x] = nn;
            },

            // 7XNN : adds given value to register VX
            (7,_,_,_) =>{
                let x = dig2 as usize;
                let nn = (opcode & 0x00FF) as u8;
                // This is done to prevent it crossing the contraint size
                self.v[x] = self.v[x].wrapping_add(nn);
            },
            
            // 8XY0 : sets VX = VY
            (8,_,_,0) =>{
                let x = dig2 as usize;
                let y = dig3 as usize;
                self.v[x] =  self.v[y];
            },

            // 8XY1 : bitwise operations
            (8,_,_,1) =>{
                let x = dig2 as usize;
                let y = dig3 as usize;
                self.v[x] |= self.v[y];
            },

            // 8XY2 : bitwise operations
            (8,_,_,2) =>{
                let x = dig2 as usize;
                let y = dig3 as usize;
                self.v[x] |= self.v[y];
            },

            // 8XY3 : bitwise operations
            (8,_,_,3) =>{
                let x = dig2 as usize;
                let y = dig3 as usize;
                self.v[x] |= self.v[y];
            },

            // 8XY4 : adding VX to VY
            (8,_,_,4) =>{
                let x = dig2 as usize;
                let y = dig3 as usize;
                let (final_vx,carry) = self.v[x].overflowing_add(self.v[y]);
                let final_vf = if carry {1} else {0};
                self.v[x] = final_vx;
                // 16 reg is used in storing the flag bit for overflow
                self.v[0xF] = final_vf;
            },

            // 8XY5 : subtraction VX -= VY
            // here the flag reg works in opposite sense
            (8,X,Y,5) =>{
                let x = dig2 as usize;
                let y = dig3 as usize;
                let final_vx = self.v[x].wrapping_sub(self.v[y]);
                let final_vf = if self.v[x] < self.v[y] {0} else {1};
                self.v[x] = final_vx;
                self.v[0xF] = final_vf;
                // overflowing_sub can also be used which offers inbuilt tuple output
            },
    

            // 8XY6 : right shifts the VX reg and stores the dropped bit into the VF reg
            (8,_,_,6) =>{
                let x = dig2 as usize;
                let y = dig3 as usize;
                let dropped_bit = self.v[x] & 1;
                self.v[x] >>= 1;
                self.v[0xF] = dropped_bit;
            },


            // 8XY7 : does VX = VY-VX
            (8,_,_,7) =>{
                let x = dig2 as usize;
                let y = dig3 as usize;

                let final_vx = self.v[y].wrapping_sub(self.v[x]);
                let final_vf = if self.v[y] < self.v[x] {0} else {1};
                self.v[x] = final_vx;
                self.v[0xF] = final_vf;
            },

            // 8XYE: sets VX as the left shifted one with bit overflow in vf register
            (8,_,_,0xE) =>{
                let x = dig2 as usize;
                let y = dig3 as usize;
                // Size these are max u8
                let dropped_bit = (self.v[x] >> 7) & 1;
                self.v[x] <<= 1;
                self.v[0xF] = dropped_bit;
            },
            
            // 9XY0 : skip iteration
            (9,X,Y,0) =>{
                let x = dig2 as usize;
                let y = dig3 as usize;
                if self.v[x] != self.v[y]{
                    self.pc+=2;
                }
            },

            // ANNN : Set the index register to this NNN
            (0xA,_,_,_) =>{
                let nnn = opcode & 0x0FFF;
                self.index_reg = nnn;
            },

            // BNNN : Jump to v0 + NNN
            (0xB,_,_,_) =>{
                let nnn = opcode & 0x0FFF;
                self.pc = (self.v[0] as u16) + nnn;
            },

            // CXNN : set VX = random() & NN
            (0xC,_,_,_) =>{
                let x = dig2 as usize;
                let nn = (opcode & 0x00FF) as u8;
                let rand_number : u8 = random();
                self.v[x] = rand_number & nn;
            },

            // EX9E : skip if key is pressed
            (0xE,_,9,0xE) =>{
                let x = dig2 as usize;
                let vx = self.v[x];
                let key = self.keys[vx as usize];
                if key  {self.pc+=2;}
            },

            // EXA1 : skip if key is not pressed
            (0xE,_,0xA,1) =>{
                let x = dig2 as usize;
                let vx = self.v[x];
                let key = self.keys[vx as usize];
                if !key {self.pc+=2;}
            },

            // DXYN : Draw Sprite
            (0xD,_,_,_) =>{
                // Getting the x and y coordinates from v registers
                let x_coordinate = self.v[dig2 as usize] as u16;
                let y_coordinate = self.v[dig3 as usize] as u16;
                
                // Last digit tells how many rows high sprite data is
                let height = dig4;

                // Tracks for flipped pixels
                let mut isflipped = false;

                // Iterating over each row of sprite data
                for y_line in 0..height{
                    // Finding which memory address row data is stored in
                    let address = self.index_reg + y_line as u16;
                    let pixels = self.ram[address as usize];
                    
                    // Each is a byte long
                    for x_line in 0..8{
                        if (pixels & (0b1000_0000 >> x_line)) != 0{
                            let x = (x_coordinate + x_line) as usize % SCREEN_WIDTH;
                            let y = (y_coordinate + y_line) as usize % SCREEN_HEIGHT;

                            // Fetch pixels index for 1D screen array
                            let idx = x + SCREEN_WIDTH  * y;

                            // Check if to flip pixel and set
                            isflipped |= self.screen[idx];
                            self.screen[idx] = true;

                        }
                    }
                }

                // Fill the VF register
                if isflipped {self.v[0xF] = 1} else {self.v[0xF] = 0};


            },

            // FX07 : set VX to delay_timer
            (0xF,_,0,7) =>{
                let x = dig2 as usize;
                self.v[x] = self.delay_timer;
            },

            // FX0A : wait for key press ( basically like a pause )
            (0xF,_,0,0xA) =>{
                let x = dig2 as usize;
                let mut pressed = false;
                for i in 0..self.keys.len(){
                    if self.keys[i]{
                        self.v[x] = i as u8;
                        pressed = true;
                        break;
                    }
                }

                if !pressed {self.pc+=2};
                
            },

            // FX15 : set delay_timer to VX
            (0xF,_,1,5) =>{
                let x = dig2 as usize;
                self.delay_timer = self.v[x];
            },
            
            // FX18 : set sound_timer to VX
            (0xF,_,1,8) =>{
                let x = dig2 as usize;
                self.sound_timer = self.v[x];
            },

            // FX1E : increment index_reg with itself and VX
            (0xF,_,1,0xE) =>{
                let x = dig2 as usize;
                let vx = self.v[x] as u16;
                self.index_reg = self.index_reg.wrapping_add(vx);
            },

            // FX29 : set I to font address
            // Each font sprite takes 5 bytes each
            (0xF,_,2,9)  =>{
                let x = dig2 as usize;
                self.index_reg = (self.v[x] as u16) * 5;
            },



            
            
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
