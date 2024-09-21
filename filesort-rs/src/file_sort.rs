pub trait FileSort {
    fn read(&self);
    fn write(&self);
    fn sort(&self);
}