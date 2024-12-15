use observer::{Observer, Publisher, PublisherErr};
use std::cell::RefCell;

#[derive(Clone, Copy, PartialEq, Debug)]
enum SomeType {
    Value1,
    Value2,
}

struct RealPublisher<'a, const N: usize> {
    observers: [Option<&'a mut dyn Observer<SomeType, RealPublisher<'a, N>>>; N],
    observer_count: usize,
    data: SomeType,
}

impl<'a, const N: usize> RealPublisher<'a, N> {
    fn new() -> Self {
        Self {
            observers: [const { None }; N],
            observer_count: 0,
            data: SomeType::Value1,
        }
    }

    fn change_value(&mut self, data: SomeType) {
        self.data = data;
        self.notify(data);
    }
}

impl<'a, const N: usize> Publisher<'a, SomeType> for RealPublisher<'a, N> {
    fn subscribe(
        &mut self,
        observer: &'a mut dyn Observer<SomeType, RealPublisher<'a, N>>,
    ) -> Result<(), PublisherErr> {
        if self.observer_count < N {
            self.observers[self.observer_count] = Some(observer);
            self.observer_count += 1;
            Ok(())
        } else {
            Err(PublisherErr::ObserverListFull)
        }
    }

    fn notify(&mut self, data: SomeType) {
        for observer in self.observers.iter_mut().flatten() {
            let mut observer_data = observer.update().borrow_mut();
            *observer_data = data;
        }
    }
}

struct RealObserver1 {
    data: RefCell<SomeType>,
}

impl RealObserver1 {
    fn new() -> Self {
        Self {
            data: RefCell::new(SomeType::Value1),
        }
    }
}

impl<'a, const N: usize> Observer<SomeType, RealPublisher<'a, N>> for RealObserver1 {
    fn update(&self) -> &RefCell<SomeType> {
        &self.data
    }
}

struct RealObserver2 {
    data: RefCell<SomeType>,
}

impl RealObserver2 {
    fn new() -> Self {
        Self {
            data: RefCell::new(SomeType::Value1),
        }
    }
}

impl<'a, const N: usize> Observer<SomeType, RealPublisher<'a, N>> for RealObserver2 {
    fn update(&self) -> &RefCell<SomeType> {
        &self.data
    }
}

fn main() {
    let mut real_publisher: RealPublisher<2> = RealPublisher::new();
    let mut real_observer_1 = RealObserver1::new();
    let mut real_observer_2 = RealObserver2::new();

    assert_eq!(real_publisher.data, SomeType::Value1);
    assert_eq!(real_publisher.observer_count, 0);

    assert_eq!(*real_observer_1.data.borrow(), SomeType::Value1);
    assert_eq!(*real_observer_2.data.borrow(), SomeType::Value1);

    match real_publisher.subscribe(&mut real_observer_1) {
        Err(PublisherErr::ObserverListFull) => debug_assert!(false),
        _ => (),
    }

    match real_publisher.subscribe(&mut real_observer_2) {
        Err(PublisherErr::ObserverListFull) => debug_assert!(false),
        _ => (),
    }

    assert_eq!(real_publisher.observer_count, 2);

    real_publisher.change_value(SomeType::Value2);

    assert_eq!(real_publisher.data, SomeType::Value2);

    assert_eq!(*real_observer_1.data.borrow(), SomeType::Value2);
    assert_eq!(*real_observer_2.data.borrow(), SomeType::Value2);
}
