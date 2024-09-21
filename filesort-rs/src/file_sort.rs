pub trait FileSort {
    fn read(&mut self);
    fn write(&mut self);
    fn sort(&mut self);
}