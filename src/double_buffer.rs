pub struct DoubleBuffer<T> {
    read: T,
    write: T,
}

impl<T> DoubleBuffer<T> {
    pub fn new(read: T, write: T) -> Self {
        Self { read, write }
    }

    pub fn from_value(value: T) -> DoubleBuffer<T>
    where
        T: Clone,
    {
        Self::new(value.clone(), value)
    }

    pub fn split(&mut self) -> (&T, &mut T) {
        (&self.read, &mut self.write)
    }

    pub fn swap(&mut self) {
        std::mem::swap(&mut self.read, &mut self.write)
    }

    pub fn writer(&mut self) -> &mut T {
        &mut self.write
    }

    pub fn reader(&self) -> &T {
        &self.read
    }
}
