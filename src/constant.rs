pub enum Constant {
    Integer(i32),
}

impl Constant {
    pub fn new_integer(value: i32) -> Self {
        Constant::Integer(value)
    }
}
