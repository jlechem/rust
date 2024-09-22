use std::io::Error;

pub trait FileSort {
    fn read(&mut self) -> Result<bool,Error>;
    fn write(&mut self) -> Result<bool,Error>;
    fn sort(&mut self);
}