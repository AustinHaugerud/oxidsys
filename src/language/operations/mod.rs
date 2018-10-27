
pub trait Operation {
    fn op_code(&self) -> u16;
    fn documentation(&self) -> &str;
}
