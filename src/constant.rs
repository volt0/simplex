pub enum Constant {
    Int(i32),
}

impl Constant {
    pub fn new_int(value: i32) -> Self {
        Constant::Int(value)
    }
}
