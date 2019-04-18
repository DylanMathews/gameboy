use super::convention::Term;

// The GameBoy has instructions & registers similar to the Intel 8080, Intel 8085, & Zilog Z80 microprocessors. It has
// eight 8-bit registers A,B,C,D,E,F,H,L and two 16-bit registers SP & PC
//
// -------------
// | A   Flags |  ---> Program Status Word
// | B       C |  ---> B
// | D       E |  ---> D
// | H       L |  ---> H
// |    SP     |  ---> Stack Pointer
// |    PC     |  ---> Program Counter
// -------------
#[derive(Clone, Default)]
pub struct Register {
    pub a: u8,
    pub f: u8, // The F register is indirectly accessible by the programer.
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

// Some instructions, however, allow you to use the registers A,B,C,D,E,H,L as 16-bit registers by pairing them up
// in the following manner: AF,BC,DE,HL.
impl Register {
    pub fn get_af(&self) -> u16 {
        (u16::from(self.a) << 8) | u16::from(self.f)
    }

    pub fn get_bc(&self) -> u16 {
        (u16::from(self.b) << 8) | u16::from(self.c)
    }

    pub fn get_de(&self) -> u16 {
        (u16::from(self.d) << 8) | u16::from(self.e)
    }

    pub fn get_hl(&self) -> u16 {
        (u16::from(self.h) << 8) | u16::from(self.l)
    }

    pub fn set_af(&mut self, v: u16) {
        self.a = (v >> 8) as u8;
        self.f = (v & 0x00F0) as u8;
    }

    pub fn set_bc(&mut self, v: u16) {
        self.b = (v >> 8) as u8;
        self.c = (v & 0x00FF) as u8;
    }

    pub fn set_de(&mut self, v: u16) {
        self.d = (v >> 8) as u8;
        self.e = (v & 0x00FF) as u8;
    }

    pub fn set_hl(&mut self, v: u16) {
        self.h = (v >> 8) as u8;
        self.l = (v & 0x00FF) as u8;
    }
}

// The Fleg Register consists of the following bits: Z, N, H, C, 0, 0, 0, 0.
pub enum Flag {
    // Zero Flag. This bit is set when the result of a math operationis zero or two values match when using the CP
    // instruction.
    Z = 0b1000_0000,
    // Subtract Flag. This bit is set if a subtraction was performed in the last math instruction.
    N = 0b0100_0000,
    // Half Carry Flag. This bit is set if a carry occurred from the lowernibble in the last math operation.
    H = 0b0010_0000,
    // Carry Flag. This bit is set if a carry occurred from the last math operation or if register A is the smaller
    // valuewhen executing the CP instruction.
    C = 0b0001_0000,
}

impl Flag {
    pub fn og(self) -> u8 {
        self as u8
    }

    pub fn bw(self) -> u8 {
        !self.og()
    }
}

impl Register {
    pub fn get_flag(&self, f: Flag) -> bool {
        (self.f & f as u8) != 0
    }

    pub fn set_flag(&mut self, f: Flag, v: bool) {
        if v {
            self.f |= f.og();
        } else {
            self.f &= f.bw();
        }
    }
}

impl Register {
    pub fn power_up(term: Term) -> Self {
        let mut r = Self::default();
        match term {
            Term::GB => {
                r.a = 0x01;
            }
            Term::GBP => {
                r.a = 0xff;
            }
            Term::GBC => {
                r.a = 0x11;
            }
            Term::SGB => {
                r.a = 0x01;
            }
        }
        r.f = 0xb0;
        r.b = 0x00;
        r.c = 0x13;
        r.d = 0x00;
        r.e = 0xd8;
        r.h = 0x01;
        r.l = 0x4d;
        // The GameBoy stack pointer is initialized to 0xfffe on power up but a programmer should not rely on this
        // setting and rather should explicitly set its value.
        r.sp = 0xfffe;
        // On power up, the GameBoy Program Counter is initialized to 0x0100 and the instruction found at this location
        // in ROM is executed. The Program Counter from this point on is controlled, indirectly, by the program
        // instructions themselves that were generated by the programmer of the ROM cart.
        r.pc = 0x0100;
        r
    }
}
