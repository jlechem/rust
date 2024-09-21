pub trait Filesort {
    fn read(&self);
    fn write(&self);
    fn sort(&self);
}