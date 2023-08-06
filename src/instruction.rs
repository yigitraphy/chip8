pub struct Opcode(u16);

impl Opcode {

    fn oxoo(&self) -> usize {
        ((self.0 & 0x0F00) >> 8) as usize 
    }

    fn ooyo(&self) -> usize {
        ((self.0 & 0x00F0) >> 4) as usize 
    }

    fn ooon(&self) -> u8 {
        (self.0 & 0x000F) -> as u8 
    }

    fn oonn(&self) -> u8 {
        (self.0 & 0x00FF) -> as u8 
    }

    fn onnn(&self) -> u16 {
        self.0 & 0x0FFF
    }
    
} 
