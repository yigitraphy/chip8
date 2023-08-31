pub struct Opcode(u16);

impl Opcode {
    // OPCODE üzerinde 0x0X00 değerini döner
    fn oxoo(&self) -> usize {
        ((self.0 & 0x0F00) >> 8) as usize 
    }

    // OPCODE üzerinde 0x00Y0 değerini döner
    fn ooyo(&self) -> usize {
        ((self.0 & 0x00F0) >> 4) as usize 
    }

    // OPCODE üzerinde 0x000N değerini döner 
    fn ooon(&self) -> u8 {
        (self.0 & 0x000F) -> as u8 
    }

    // OPCODE üzerinde 0x00NN değerini döner 
    fn oonn(&self) -> u8 {
        (self.0 & 0x00FF) -> as u8 
    }

    // OPCODE üzerinde 0x0NNN değerini döner
    fn onnn(&self) -> u16 {
        self.0 & 0x0FFF
    }
} 

impl From<u16> for Opcode {
    fn from(opcode: u16) -> Opcode {
        Opcode(opcode)
    }
}

pub type Address = u16;
pub type Register = usize;

pub enum Instruction {
    ClearDisplay,                                              //  
    Return,                                                    //
    Jump(Address),                                             //
    Call(Address),                                             //
    SkipIfEqualsByte(Register, u8),                            //
    SkipIfNotEqualsByte(Register, u8),                         //
    SkipIfEqual(Register, Register),                           //
    LoadByte(Register, u8),                                    //
    AddByte(Register, u8),                                     //
    Move(Register, Register),                                  //
    Or(Register, Register),                                    //
    And(Register, Register),                                   //
    Xor(Register, Register),                                   //
    Add(Register, Register),                                   //
    Sub(Register, Register),                                   //
    ShiftRight(Register),                                      //
    ReverseSub(Register, Register),                           //
    ShiftLeft(Register),                                      //
    SkipIfNotEqualsByte(Register, Register),                  //
    LoadI(Address),                                           //
    JumpPlusZero(Address),                                    //
    Random(Register, u8),                                     //
    Draw(Register, Register, u8),                             //
    SkipIfPressed(Register),                                 //
    SkipIfNotPressed(Register),                               //
    LoadDelayTimer(Register),                                 //
    WaitForKeyPress(Register),                                //
    SetDelayTimer(Register),                                  //
    SetSoundTimer(Register),                                  //
    AddToI(Register),                                         //
    LoadSprite(Register),                                     //
    BCDRepresentation(Register),                             // 
    StoreRegisters(Register),                                //
    LoadRegisters(Register),                                 //
} 
