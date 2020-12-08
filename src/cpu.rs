const FONTS: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // '0' at 0x00
    0x20, 0x60, 0x20, 0x20, 0x70, // '1' at 0x05
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // '2' at 0x0A
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // '3' at 0x0F
    0x90, 0x90, 0xF0, 0x10, 0x10, // '4' at 0x14
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // '5' at 0x19
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // '6' at 0x1E
    0xF0, 0x10, 0x20, 0x40, 0x40, // '7' at 0x23
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // '8' at 0x28
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // '9' at 0x2D
    0xF0, 0x90, 0xF0, 0x90, 0x90, // 'A' at 0x32
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // 'B' at 0x37
    0xF0, 0x80, 0x80, 0x80, 0xF0, // 'C' at 0x3C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // 'D' at 0x41
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // 'E' at 0x46
    0xF0, 0x80, 0xF0, 0x80, 0x80  // 'F' at 0x4B
];

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Cpu {
    v: [u8; 16], // 通用寄存器
    i: u16, // 地址寄存器
    pc: u16, // 程序计数器
    mem: [u8; 0x1000], // 内存
    stack: [u16; 16],
    pub keys: [bool; 16],
    sp: u8, // 栈指针
    delay: u8,
    sound: u8,
    pub gfx: [u8; 2048],
    pub step_num: usize,
}

impl Cpu {
    pub fn new(rom: Vec<u8>) -> Self {
        let mut mem = [0u8; 0x1000];
        Self::init_memory(&mut mem, rom);
        Self {
            v: [0; 16],
            mem,
            i: 0x0000,
            pc: 0x0200,
            keys: [false; 16],
            stack: [0; 16],
            sp: 0,
            delay: 0,
            sound: 0,
            gfx: [0; WIDTH * HEIGHT],
            step_num: 0,
        }
    }

    pub fn new_wasm() -> Self {
        Self {
            v: [0; 16],
            mem: [0u8; 0x1000],
            i: 0x0000,
            pc: 0x0200,
            keys: [false; 16],
            stack: [0; 16],
            sp: 0,
            delay: 0,
            sound: 0,
            gfx: [0; WIDTH * HEIGHT],
            step_num: 0,
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        Self::init_memory(&mut self.mem, rom);
    }

    fn init_memory(mem: &mut [u8; 0x1000], rom: Vec<u8>) {
        for i in 0..rom.len() {
            mem[i + 0x200] = rom[i];
        }
        for i in 0..FONTS.len() {
            mem[i] = FONTS[i];
        }
    }

    pub fn step(&mut self) {
        let pc = self.pc as usize;
        let ins = ((self.mem[pc] as u16) << 8 | self.mem[pc + 1] as u16) as usize;
        let (x, y, nn) = ((ins & 0x0F00) >> 8, (ins & 0x00F0) >> 4, ins & 0x00FF);
        self.pc += 2;
        match ins {
            // 00E0 - CLS
            0x00E0 => {
                for i in 0..self.gfx.len() {
                    self.gfx[i] = 0;
                }
            },
            // 00EE - RET
            0x00EE => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize] + 2;
            }
            _ => {
                match ins >> 12 {
                    // 0NNN - SYS addr
                    0x0 => {
                        // self.pc += 2;
                    }
                    // 1NNN - JMP addr
                    0x1 => {
                        let addr = ins & 0x0FFF;
                        self.pc = addr as u16;
                    }
                    // 2NNN - CALL addr
                    0x2 => {
                        let addr = ins & 0x0FFF;
                        self.stack[self.sp as usize] = self.pc - 2;
                        self.sp += 1;
                        self.pc = addr as u16;
                    }
                    // 3XNN - SE Vx, byte
                    0x3 => {
                        if self.v[x] == nn as u8 {
                            self.pc += 2;
                        }
                    }
                    // 4XNN - SNE VX, BYTE
                    0x4 => {
                        if self.v[x] != nn as u8 {
                            self.pc += 2;
                        }
                    }
                    // 5XY0 - 解释器将寄存器Vx与寄存器Vy进行比较，如果相等，则将程序计数器加2。
                    0x5 => {
                        if ins & 0x000F == 0x0 {
                            if self.v[x] == self.v[y] {
                                self.pc += 2;
                            }
                        }
                    }
                    // 6XNN - 解释器将值NN放入寄存器Vx中。
                    0x6 => {
                        self.v[x] = nn as u8;
                    }
                    // 7XNN - 将值NN加到寄存器Vx的值，然后将结果存储在Vx中。
                    0x7 => {
                        // println!("vx = {}, nn = {}", self.v[x], nn);
                        let sum = self.v[x] as usize + nn;
                        self.v[x] = sum as u8;
                    }
                    0x8 => {
                        match ins & 0x000F {
                            // 8XY0 - 将寄存器Vy的值存储在寄存器Vx中。
                            0x0 => {
                                self.v[x] = self.v[y];
                            }
                            // 8XY1 - 将寄存器VX设置为VX | VY
                            0x1 => {
                                self.v[x] |= self.v[y];
                            }
                            // 8XY2 - 将寄存器VX设置为VX & VY
                            0x2 => {
                                self.v[x] &= self.v[y];
                            }
                            // 8XY3 - 将寄存器VX设置为VX ^ VY
                            0x3 => {
                                self.v[x] ^= self.v[y];
                            }
                            // 8XY4 - 将寄存器VY的值加到寄存器VX 如果发生进位，则将VF设置为01 如果未发生进位，则将VF设置为00
                            0x4 => {
                                let sum = self.v[x] as usize + self.v[y] as usize;
                                self.v[0xF] = if sum > 0xFF {
                                    1
                                } else {
                                    0
                                };
                                self.v[x] = sum as u8;
                            }
                            // 8XY5 - 从寄存器VX减去寄存器VY的值 如果发生借阅，请将VF设置为00 如果不发生借阅，请将VF设置为01
                            0x5 => {
                                let diff = self.v[x] as isize - self.v[y] as isize;
                                self.v[0xF] = if diff > 0 {
                                    1
                                } else {
                                    0
                                };
                                self.v[x] = diff as u8;
                            }
                            // 8XY6 - 将寄存器VY的值右移一位存储在寄存器VX¹中 在移位之前将寄存器VF设置为最低有效位 VY不变
                            0x6 => {
                                // self.v[0xF] = self.v[y] & 0x1;
                                // self.v[x] = self.v[y] >> 1;
                                self.v[0xF] = self.v[x] & 0x1;
                                self.v[x] >>= 1;
                            }
                            // 8XY7 - 将寄存器VX设置为VY减去VX的值 如果发生借阅，请将VF设置为00 如果不发生借阅，请将VF设置为01
                            0x7 => {
                                let diff = self.v[y] as isize - self.v[x] as isize;
                                self.v[0xF] = if diff > 0 {
                                    1
                                } else {
                                    0
                                };
                                self.v[x] = diff as u8;
                            }
                            // 8XYE - 如果Vx的最高有效位为1，则VF设置为1，否则设置为0。然后Vx乘以2。
                            0xE => {
                                // self.v[0xF] = self.v[y] >> 7;
                                // self.v[x] = self.v[y] << 1;
                                self.v[0xF] = self.v[x] >> 7;
                                self.v[x] <<= 1;
                            }
                            _ => {
                                println!("无效指令");
                            }
                        }
                    }
                    0x9 => {
                        // 9XY0 - 如果寄存器VX的值不等于寄存器VY的值，则跳过以下指令
                        if (ins & 0x000F) == 0x0 {
                            if self.v[x] != self.v[y] {
                                self.pc += 2;
                            }
                        }
                    }
                    // ANNN - 将存储器地址NNN存储在寄存器I中
                    0xA => {
                        let addr = ins & 0x0FFF;
                        self.i = addr as u16;
                    }
                    // BNNN - 跳转到地址NNN + V0
                    0xB => {
                        let addr = ins & 0x0FFF;
                        self.pc = self.v[0] as u16 + addr as u16;
                    }
                    // CXNN - 将VX设置为带有NN掩码的随机数
                    0xC => {
                        self.v[x] = rand::random::<u8>() & nn as u8;
                    }
                    // DXYN - 显示从（Vx，Vy）的内存位置I开始的n字节精灵，设置VF =冲突。
                    //
                    // 解释器从内存中读取n个字节（从I中存储的地址开始）。然后，这些字节在屏幕上的坐标（Vx，Vy）上显示为精灵。
                    // 将子画面异或到现有屏幕上。 如果这导致任何像素被擦除，则将VF设置为1，否则将其设置为0。
                    // 如果将子画面定位为使其一部分不在显示坐标之内，则它将环绕屏幕的另一侧。
                    // 有关XOR的更多信息，请参见指令8xy3；有关Chip-8屏幕和子画面的更多信息，请参见第2.4节“显示”。
                    0xD => {
                        let n = ins & 0x000F;
                        let (x, y) = (self.v[x] as usize, self.v[y] as usize);
                        self.v[0xF] = 0;
                        let i = self.i as usize;
                        for yl in 0..n {
                            let pixel = self.mem[i + yl];
                            for xl in 0..8 {
                                if (pixel & (0x80 >> xl)) != 0 {
                                    let x = x + xl; //if x + xl > 63 {127 - (x + xl)} else {x + xl};
                                    let y = y + yl;// if y + yl > 31 {63 - (y + yl)} else {y + yl};
                                    // println!("Drawing at ({}, {})", x, y);
                                    let x = if x >= WIDTH {
                                        x - WIDTH
                                    } else { x };
                                    let y = if y >= HEIGHT {
                                        y - HEIGHT
                                    } else { y };
                                    if self.gfx[x + y * WIDTH] == 1 {
                                        self.v[0xF] = 1;
                                    }
                                    self.gfx[x + y * WIDTH] ^= 1;
                                }
                            }
                        }
                        // println!("Drawing at ({}, {}) with {:#08b} bytes", x, y, n);
                    }
                    0xE => {
                        // EX9E - 如果按下与当前存储在寄存器VX中的十六进制值相对应的键，则跳过以下指令
                        if ins & 0x00FF == 0x9E {
                            if self.keys[self.v[x] as usize] {
                                self.pc += 2;
                            }
                        }
                        // EXA1 - 如果未按下与当前存储在寄存器VX中的十六进制值相对应的键，请跳过以下指令
                        if ins & 0x00FF == 0xA1 {
                            if !self.keys[self.v[x] as usize] {
                                self.pc += 2;
                            }
                        }
                    }
                    0xF => {
                        match ins & 0x00FF {
                            // FX07 - 将延迟定时器的当前值存储在寄存器VX中
                            0x07 => {
                                self.v[x] = self.delay;
                            },
                            // FX0A - 等待按键并将结果存储在寄存器VX中
                            0x0A => {
                                self.pc -= 2;
                                for i in 0..self.keys.len() {
                                    if self.keys[i] {
                                        self.v[x] = i as u8;
                                        self.pc += 2;
                                        break;
                                    }
                                }
                            }
                            // FX15 - 将延迟定时器设置为寄存器VX的值
                            0x15 => {
                                self.delay = self.v[x];
                            }
                            // FX18 - 将声音计时器设置为寄存器VX的值
                            0x18 => {
                                self.sound = self.v[x];
                            }
                            // FX1E - 将存储在寄存器VX中的值加到寄存器I
                            0x1E => {
                                let sum = self.i + self.v[x] as u16;
                                // if sum > 0xFFF {
                                //     self.v[0xF] = 1;
                                // } else {
                                //     self.v[0xF] = 0;
                                // }
                                self.i = sum;
                            }
                            // FX29 - 将I设置为与寄存器VX中存储的十六进制数字相对应的Sprite数据的存储地址
                            0x29 => {
                                self.i = self.v[x] as u16 * 0x5;
                            }
                            // FX33	- 在地址I，I + 1和I + 2处存储与寄存器VX中存储的值等效的二进制编码十进制数
                            0x33 => {
                                let i = self.i as usize;
                                let mut v = self.v[x];
                                self.mem[i + 2] = v % 10;
                                v /= 10;
                                self.mem[i + 1] = v % 10;
                                v /= 10;
                                self.mem[i] = v % 10;
                            }
                            // FX55 - 从地址I开始将存储器中的寄存器V0至VX的值包括在内 手术后我被设置为I + X +1
                            0x55 => {
                                for i in 0..x + 1{
                                    self.mem[i + self.i as usize] = self.v[i];
                                    // self.v[i] = self.mem[self.i as usize + i];
                                }
                                // self.i += (x + 1) as u16;
                            }
                            // FX65 - 用地址I中存储的值填充寄存器V0至VX 手术后我被设置为I + X +1
                            0x65 => {
                                for i in 0..x + 1{
                                    self.v[i] = self.mem[self.i as usize + i];
                                }
                                // self.i += (x + 1) as u16;
                            }
                            _ => {
                                println!("无效指令");
                            }
                        }
                    }
                    _ => {
                        println!("无效指令");
                    }
                };
            }
        };
    }

    pub fn update_timers(&mut self) {
        if self.delay > 0 {
            self.delay -= 1;
        }
        if self.sound > 0 {
            if self.sound == 1 {
                // Todo: Audio
            }
            self.sound -= 1;
        }
    }
}