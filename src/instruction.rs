pub type Address = u16;
pub type Register = usize;

pub struct Opcode(u16);

impl Opcode {
    // Returns 0x0X00 in OPCODE
    fn oxoo(&self) -> usize {
        ((self.0 & 0x0F00) >> 8) as usize 
    }

    // Returns 0x00Y0 in OPCODE
    fn ooyo(&self) -> usize {
        ((self.0 & 0x00F0) >> 4) as usize 
    }

    // Returns 0x000N in OPCODE
    fn ooon(&self) -> u8 {
        (self.0 & 0x000F) -> as u8 
    }

    // Returns 0x00NN in OPCODE
    fn oonn(&self) -> u8 {
        (self.0 & 0x00FF) -> as u8 
    }

    // Returns 0x0NNN in OPCODE
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
    ClearDisplay,                                              //  Clear the display.
    Return,                                                    //  Return from a subroutine
    Jump(Address),                                             //  Jump to the specified address
    Call(Address),                                             //  Call a subroutine at the specified address and store the program counter
    SkipIfEqualsByte(Register, u8),                            //  Skip the next instruction if the register is equal to the specified value
    SkipIfNotEqualsByte(Register, u8),                         //  Skip the next instruction if the register is not equal to the specified value
    SkipIfEqual(Register, Register),                           //  Skip the next instruction if two registers are equal
    LoadByte(Register, u8),                                    //  Load the specified value into the register
    AddByte(Register, u8),                                     //  Add the specified value to the register
    Move(Register, Register),                                  //  Copy the value from one register to another
    Or(Register, Register),                                    //  Perform a bitwise OR operation between two registers
    And(Register, Register),                                   //  Perform a bitwise AND operation between two registers
    Xor(Register, Register),                                   //  Perform a bitwise XOR operation between two registers
    Add(Register, Register),                                   //  Perform an addition operation between two registers
    Sub(Register, Register),                                   //  Perform a subtraction operation between two registers
    ShiftRight(Register),                                      //  Shift the value in the specified register one bit to the right
    ReverseSub(Register, Register),                            //  Subtract one register from another, and save the overflow status
    ShiftLeft(Register),                                       //  Shift the value in the specified register one bit to the left
    SkipIfNotEqualsByte(Register, Register),                   //  Skip the next instruction if two registers are not equal
    LoadI(Address),                                            //  Load the specified address into the I register
    JumpPlusZero(Address),                                     //  Jump to the specified address plus the value in V0
    Random(Register, u8),                                      //  Load a random value into the specified register
    Draw(Register, Register, u8),                              //  Draw a sprite on the screen
    SkipIfPressed(Register),                                   //  Skip the next instruction if the specified key is pressed
    SkipIfNotPressed(Register),                                //  Skip the next instruction if the specified key is not pressed
    LoadDelayTimer(Register),                                  //  Load the value of the delay timer into the specified register
    WaitForKeyPress(Register),                                 //  Wait until a key is pressed and store its value
    SetDelayTimer(Register),                                   //  Set the delay timer to the value in the specified register
    SetSoundTimer(Register),                                   //  Set the sound timer to the value in the specified register
    AddToI(Register),                                          //  Add the value in the specified register to the register
    LoadSprite(Register),                                      //  Load the address of the sprite for the specified digit into the register
    BCDRepresentation(Register),                               //  Convert the value in the specified register to BCD representation in memory
    StoreRegisters(Register),                                  //  Store registers V0 through Vx in memory starting at address 
    LoadRegisters(Register),                                   //  Load registers V0 through Vx from memory starting at address
} 

impl Instruction {
    pub fn new<T: Into<Opcode>>(opcode: T) -> Option<Instruction> {
        let opcode = opcode.into();
        match opcode.0 & 0xF000 {
            0x0000 => match opcode.ooon() {
                0x0000 => Some(Instruction::ClearDisplay),
                0x000E => Some(Instruction::Return),
                _ => None,
            },
            0x1000 => Some(Instruction::Jump(opcode.onnn())),
            0x2000 => Some(Instruction::Call(opcode.onnn())),
            0x3000 => Some(Instruction::SkipIfEqualsByte(opcode.oxoo(), opcode.oonn())),
            0x4000 => Some(Instruction::SkipIfNotEqualsByte(
                opcode.oxoo(),
                opcode.oonn(),
            )),
            0x5000 => Some(Instruction::SkipIfEqual(opcode.oxoo(), opcode.ooyo()),
            0x6000 => Some(Instruction::LoadByte(opcode.oxoo(), opcode.oonn())),
            0x7000 => Some(Instruction::AddByte(opcode.oxoo(), opcode.oonn())),
            0x8000 => match opcode.ooon() {
                0x0000 => Some(Instruction::Move(opcode.oxoo(), opcode.ooyo())),
                0x0001 => Some(Instruction::Or(opcode.oxoo(), opcode.oonn())),
                0x0002 => Some(Instruction::And(opcode.oxoo(), opcode.ooyo())),
                0x0003 => Some(Instruction::Xor(opcode.oxoo(), opcode.ooyo())),
                0x0004 => Some(Instruction::Add(opcode.oxoo(), opcode.ooyo())),
                0x0005 => Some(Instruction::Sub(opcode.oxoo(), opcode.ooyo())),
                0x0006 => Some(Instruction::ShiftRight(opcode.oxoo(), opcode.ooyo())),
                0x0007 => Some(Instruction::ReverseSub(opcode.oxoo(), opcode.ooyo())),
                0x000E => Some(Instruction::ShiftLeft(opcode.oxoo())),
                _ => None,
                0x0009 => Some(Instruction::SkipIfNotEqual(opcode.oxoo(), opcode.ooyo())),
                0x000A => Some(Instruction::LoadI(opcode.onnn())),
                0x000B => Some(Instruction::JumpPlusZero(opcode.onnn())),
                0x000C => Some(Instruction::Random(opcode.oxoo(), opcode.oonn())),
                0x000D => Some(Instruction::Draw(
                    opcode.oxoo(),
                    opcode.ooyo(),
                    opcode.ooon(),
                )),
                0xE000 => match opcode.oonn() {
                    0x009E => Some(Instruction::SkipIfPressed(opcode.oxoo())),
                    0x00A1 => Some(Instruction::SkipIfNotPressed(opcode.oxoo())),
                    _ => None,
                },
                0xF000 => match opcode.oonn() {
                    0x0007 => Some(Instruction::LoadDelayTime(opcode.oxoo())),
                    0x000A => Some(Instruction::WaitForKeyPress(opcode.oxoo())),
                    0x0015 => Some(Instruction::SetDelayTimer(opcode.oxoo())),
                    0x0018 => Some(Instruction::SetSoundTimer(opcode.oxoo())),
                    0x001E => Some(Instruction::AddToI(opcode.oxoo())),
                    0x0029 => Some(Instruction::LoadSprite(opcode.oxoo())),
                    0x0033 => Some(Instruction::BCDRepresentation(opcode.oxoo())),
                    0x0055 => Some(Instruction::StoreRegisters(opcode.oxoo())),
                    0x0065 => Some(Instruction::LoadRegisters(opcode.oxoo())),
                    _ => None, 
                },
                _ => None,
            }
            )
        }
    }
}
