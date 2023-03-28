pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
const FONTSET_SIZE: usize = 80;

const FONTSET: [u8; FONTSET_SIZE] = [ 
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
                                    0xF0, 0x80, 0x80, 0x80, 0xF0, //C 
                                    0xE0, 0x90, 0x90, 0x90, 0xE0, // D 
                                    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E 
                                    0xF0, 0x80, 0xF0, 0x80, 0x80 // F
                                    ];



const RAM_SIZE: usize = 4096;
const NUM_REG: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;


pub struct Emu {
    pc: u16, // program counter
    ram: [u8; RAM_SIZE],
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    v_reg: [u8; NUM_REG],
    i_reg: u16,
    sp: u16,
    stack: [u16; STACK_SIZE],
    keys: [bool; NUM_KEYS],
    dt: u8,
    st: u8,
}

const START_ADDR: u16 = 0x200;

impl Emu {
    pub fn new() -> Self {
        let mut new_emu = Self {
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH*SCREEN_HEIGHT],
            v_reg: [0; NUM_REG],
            i_reg: 0,
            sp:0,
            stack: [0;STACK_SIZE],
            keys: [false;NUM_KEYS],
            dt: 0,
            st: 0,
        };
        new_emu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
        new_emu
    }

    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1 ;
    }

    fn pop(&mut self) -> u16 {
        self.sp -= 1 ;
        self.stack[self.sp as usize]
    }
    pub fn reset(&mut self) {
        self.pc = START_ADDR;
        self.ram = [0; RAM_SIZE];
        self.screen = [false; SCREEN_WIDTH*SCREEN_HEIGHT];
        self.v_reg = [0; NUM_REG];
        self.i_reg = 0;
        self.sp = 0;
        self.stack = [0; STACK_SIZE];
        self.keys = [false;NUM_KEYS];
        self.dt = 0;
        self.st = 0;
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }
    pub fn tick(&mut self) {
        // Fetch 
        let op = self.fetch();
        
        // Decode
        self.execute(op);

        // Execute
    }
    fn fetch(&mut self) -> u16 {
        let higher_byte = self.ram[self.pc as usize] as u16;
        let lower_byte = self.ram[(self.pc + 1) as usize ] as u16;
        let op = (higher_byte << 8) | lower_byte;
        self.pc += 2;
        op
    }
    pub fn tick_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1 ;
        }
        if self.st > 0 {
            if self.st == 1 {
                // BEEP sound
            }
            self.st -= 1 ;
        }
    }
    fn execute(&mut self, op:u16) {
        let digit1 = (op & 0xF000) >> 12 ;
        let digit2 = (op & 0x0F00) >> 8 ;
        let digit3 = (op & 0x00F0) >> 4 ;
        let digit4 = (op & 0x000F) ;

        match(digit1, digit2, digit3, digit4) {
            (_, _, _, _) => unimplemented!("Unimplemented Opcode: {}", op),

            // CLS 
            (0,0,0xE,0) => {
                self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
            },
            (0,0,0xE,0xE) => {
                let ret_addr = self.pop();
                self.pc = ret_addr;
            }
            (1, _, _, _) => {
                let nnn = op & 0xFFF;
                self.pc = nnn;
            }
            (2, _, _, _) => {
                let nnn = op & 0xFFF;
                self.push(nnn);
                self.pc = nnn;
            }
            (3, _, _, _) => {
                let x = digit2 as usize ; 
                let nn = (op * 0xFF) as u8; 
                if self.v_reg[x] == nn {
                    self.pc +=2 ;
                }
            }
            (4, _, _, _) => {
                let x = digit2 as usize ; 
                let nn = (op * 0xFF) as u8; 
                if self.v_reg[x] != nn {
                    self.pc +=2 ;
                }
            }
            (5, _, _, 0) => {
                let x = digit2 as usize ; 
                let y = digit3 as usize;
                let nn = (op * 0xFF) as u8; 
                if self.v_reg[x] != self.v_reg[y] {
                    self.pc +=2 ;
                }
            }
            (6, _, _, _) => {
                let x = digit2 as usize ; 
                let nn = (op * 0xFF) as u8; 
                self.v_reg[x] = nn;
            }
            (7, _, _, _) => {
                let x = digit2 as usize ; 
                let nn = (op * 0xFF) as u8; 
                self.v_reg[x] = self.v_reg[x].wrapping_add(nn);
            }
            (8, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] = self.v_reg[y];
            }
            (8, _, _, 1) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] != self.v_reg[y];
            }
            (8, _, _, 4) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                
                let (new_vx, carry) = self.v_reg[x].overflowing_add(self.v_reg[y]);
                let new_vf = if carry {1} else {0};

                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            }
            (8, _, _, 5) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                
                let (new_vx, borrow) = self.v_reg[x].overflowing_sub(self.v_reg[y]);
                let new_vf = if borrow {0} else {1};

                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            }
            (8, _, _, 6) => {
                let x = digit2 as usize;
                let lab = self.v_reg[x] & 1; 
                self.v_reg[x] >>= 1 ;
                self.v_reg[0xF] = lab;
            }

            (8, _, _, 7) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                
                let (new_vx, borrow) = self.v_reg[y].overflowing_sub(self.v_reg[x]);
                let new_vf = if borrow {0} else {1};

                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            }

        }

    }
}

// Continue from 5. Opcode execution
