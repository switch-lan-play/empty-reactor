use empty_reactor::{EmptyReactor, EventSource};
use std::time::{Duration, Instant};
use std::task::{Waker, Poll, Context};
use std::collections::BinaryHeap;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};

enum Event {
    Ready,
}

struct TimerReactor {
    timers: BinaryHeap<Instant>
}

impl EventSource for TimerReactor {
    type Handle = usize;
    type Event = Event;
    fn wait(&self, events: &[(Self::Handle, Self::Event)]) -> Duration {
        Duration::from_secs(1)
    }
    fn poll(&self, _events: &[(Self::Handle, Self::Event)]) -> Vec<Waker> {
        Vec::new()
    }
}

impl TimerReactor {
    fn insert() {
        static ID_GENERATOR: AtomicUsize = AtomicUsize::new(1);
        let id = ID_GENERATOR.fetch_add(1, Ordering::Relaxed);

        
    }
}

struct Delay {}
fn delay(duration: Duration) {

}
impl std::future::Future for Delay {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        Poll::Ready(())
    }
}

fn main() {
    println!("hello world");
}
