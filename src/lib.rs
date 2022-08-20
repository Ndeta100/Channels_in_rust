use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
};

pub struct Sender<T> {
    inner: Arc<Inner<T>>,
}
pub struct Receiver<T> {
    inner: Arc<Inner<T>>,
}
struct Inner<T> {
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
}
impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Sender {
            inner: Arc::clone(&self.inner),
        }
    }
}
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        queue: Mutex::default(),
        available: Condvar::new(),
    };
    let inner = Arc::new(inner);
    (
        Sender {
            inner: inner.clone(),
        },
        Receiver {
            inner: inner.clone(),
        },
    )
}
impl<T> Sender<T> {
    pub fn send(&mut self, t: T) {
        let mut queue = self.inner.queue.lock().unwrap();
        queue.push_back(t);
        drop(queue);
        self.inner.available.notify_one();
    }
}
impl<T> Receiver<T> {
    pub fn recv(&mut self) -> T {
        loop {
            let mut queue = self.inner.queue.lock().unwrap();
            match queue.pop_front() {
                Some(t) => return t,
                None => queue = self.inner.available.wait(queue).unwrap(),
            }
        }
    }
}
#[cfg(test)]
mod test {
    use crate::channel;
    #[test]
    fn ping_pong() {
        let (mut tx, mut rx) = channel();
        tx.send(42);
        assert_eq!(rx.recv(), 42);
    }
}
