use std::collections::BTreeMap;
use std::io;
use std::sync::{Arc, Mutex};
use std::task::{Poll, Waker};
use futures::future;

pub struct Source<Event> {
    wakers: Mutex<BTreeMap<Event, Vec<Waker>>>,
}

pub trait EventSource<Handle, Event> {
    fn poll(&self, events: &[(Handle, Event)]) -> Vec<Waker>;
}

pub struct EmptyReactor<Handle, Event, S> {
    es: S,
    events: Mutex<Vec<(Handle, Event)>>,
    sources: Mutex<BTreeMap<Handle, Arc<Source<Event>>>>,
}

impl<Event> Source<Event>
where
    Event: Ord + Copy,
{
    fn new() -> Source<Event> {
        Source {
            wakers: Mutex::new(BTreeMap::new())
        }
    }
    pub async fn wait(&self, event: Event) -> io::Result<()> {
        let mut polled = false;

        future::poll_fn(move |cx| {
            if polled {
                Poll::Ready(Ok(()))
            } else {
                let mut wakers_map = self.wakers
                    .lock()
                    .unwrap();
                let wakers = wakers_map
                    .entry(event)
                    .or_insert_with(Vec::new);

                if wakers.iter().all(|w| !w.will_wake(cx.waker())) {
                    wakers.push(cx.waker().clone());
                }

                polled = true;
                Poll::Pending
            }
        })
        .await
    }
}

impl<Handle, Event, S> EmptyReactor<Handle, Event, S>
where
    S: EventSource<Handle, Event>,
    Handle: Ord + Copy,
    Event: Ord + Copy,
{
    pub fn new(es: S) -> EmptyReactor<Handle, Event, S> {
        EmptyReactor {
            es,
            events: Mutex::new(Vec::new()),
            sources: Mutex::new(BTreeMap::new()),
        }
    }
    pub fn insert(&self, handle: Handle) -> Arc<Source<Event>> {
        let source = Arc::new(Source::new());
        self.sources.lock().unwrap().insert(handle, source.clone());
        source
    }
    pub fn remove(&self, handle: &Handle) {
        self.sources.lock().unwrap().remove(handle);
    }
    pub async fn run(&self) {
        loop {
            let events = self.events.lock().unwrap();
            let ready = self.es.poll(&events);
            drop(events);
            for waker in ready {
                waker.wake();
            }
        }
    }
}
