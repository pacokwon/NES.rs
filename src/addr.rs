pub enum AddrMode {
    Acc,
    Imp,    // Implied
    Imm,    // Immediate
    Zero,
    ZeroX,
    ZeroY,
    Abs,
    AbsX,
    AbsXPlus,
    AbsY,
    AbsYPlus,
    Ind,
    IndX,
    IndY,
    IndYPlus,
    Rel,    // Relative
}

pub trait Addr6502 {
    fn acc(&mut self);
    fn imp(&mut self);
    fn imm(&mut self);
    fn zero(&mut self);
    fn zero_x(&mut self);
    fn zero_y(&mut self);
    fn abs(&mut self);
    fn abs_x(&mut self);
    fn abs_x_plus(&mut self);
    fn abs_y(&mut self);
    fn abs_y_plus(&mut self);
    fn ind(&mut self);
    fn ind_x(&mut self);
    fn ind_y(&mut self);
    fn ind_y_plus(&mut self);
    fn rel(&mut self);
}
