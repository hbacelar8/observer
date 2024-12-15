use std::cell::RefCell;

pub enum PublisherErr {
    ObserverListFull,
}

pub trait Observer<T, P> {
    fn update(&self) -> &RefCell<T>;
}

pub trait Publisher<'a, T> {
    fn subscribe(&mut self, observer: &'a mut dyn Observer<T, Self>) -> Result<(), PublisherErr>;
    fn notify(&mut self, data: T)
    where
        T: Copy;
}
