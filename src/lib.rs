use std::time::{Duration, Instant};
use std::thread::sleep;

pub trait TimeIter: Sized {
    fn timed(self, dur: Duration) -> TimedIterator<Self>;
}

impl<I> TimeIter for I
    where I: Iterator
{
    fn timed(self, dur: Duration) -> TimedIterator<Self> {
        TimedIterator::from_iter(self, dur)
    }
}

pub struct TimedIterator<I> {
    inner: I,
    last_call: Instant,
    duration: Duration,
}

impl<I> Iterator for TimedIterator<I>
    where I: Iterator
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let remaining = self.duration.checked_sub(self.last_call.elapsed());
        if let Some( dur ) = remaining {
            sleep(dur);
        }
        
        self.last_call = Instant::now();
        self.inner.next()
    }
}

impl<I> TimedIterator<I>
    where I: Iterator
{
    fn from_iter(iter: I, dur: Duration) -> Self {
        Self {
            inner: iter,
            last_call: Instant::now(),
            duration: dur,
        }
    }
}



#[cfg(test)]
mod tests {
    use super::TimeIter;
    use std::time::{Duration, Instant};
    use std::thread::sleep;

    #[test]
    fn sleep_works() {
        let now = Instant::now();
        for _ in (0..3).timed(Duration::from_millis(100)) 
        {
        }
        assert!(now.elapsed() >= Duration::from_millis(300));
    }

    #[test]
    fn not_sleeping_longer() {
        let now = Instant::now();
        for _ in (0..3).timed(Duration::from_millis(100)) 
        {
            sleep(Duration::from_millis(90));
        }
        assert!(now.elapsed() <= Duration::from_millis(401));
    }
}
