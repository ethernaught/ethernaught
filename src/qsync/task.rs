use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::time::{Duration, Instant};
use std::{ptr, thread};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use crate::qsync::delay::Delay;

#[derive(Clone)]
pub struct Task {
    tasks: Arc<Mutex<Vec<Pin<Box<dyn Future<Output = ()> + Send>>>>>,
    running: Arc<AtomicBool>
}

impl Task {

    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(Vec::new())),
            running: Arc::new(AtomicBool::new(false))
        }
    }

    pub fn delay_for(duration: Duration) -> Delay {
        Delay {
            when: Instant::now() + duration,
        }
    }

    pub fn spawn<F>(&self, fut: F)
    where
        F: Future<Output = ()> + Send + 'static
    {
        self.tasks.lock().as_mut().unwrap().push(Box::pin(fut));
        self.run();
    }

    fn noop_waker() -> Waker {
        const NOOP_VTABLE: &RawWakerVTable = &RawWakerVTable::new(
            |data: *const ()| RawWaker::new(data, NOOP_VTABLE),
            |data: *const ()| {},
            |data: *const ()| {},
            |data: *const ()| {},
        );

        fn raw_waker() -> RawWaker {
            RawWaker::new(ptr::null(), NOOP_VTABLE)
        }

        unsafe { Waker::from_raw(raw_waker()) }
    }

    fn run(&self) {
        if self.running.load(Ordering::Relaxed) {
            return;
        }

        self.running.store(true, Ordering::Relaxed);
        let tasks = self.tasks.clone();
        let running = self.running.clone();

        thread::spawn(move || {
            let waker = Self::noop_waker();
            let mut cx = Context::from_waker(&waker);

            let mut last_check = Instant::now();
            let check_interval = Duration::from_millis(1);

            while !tasks.lock().unwrap().is_empty() {
                let mut i = 0;
                while i < tasks.lock().as_ref().unwrap().len() {
                    let poll = tasks.lock().as_mut().unwrap().get_mut(i).unwrap().as_mut().poll(&mut cx);
                    match poll {
                        Poll::Ready(_) => {
                            tasks.lock().as_mut().unwrap().remove(i);
                        }
                        Poll::Pending => {
                            i += 1;
                        }
                    }
                }

                thread::sleep(check_interval);

                if last_check.elapsed() > check_interval {
                    last_check = Instant::now();
                }
            }

            running.store(false, Ordering::Relaxed);
        });
    }
}
