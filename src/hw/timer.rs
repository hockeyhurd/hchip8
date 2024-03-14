pub struct Timer
{
    duration: std::time::Duration,
    start_time: std::time::Instant,
}

impl Timer
{
    #[allow(dead_code)]
    pub fn new(duration: std::time::Duration)-> Self
    {
        Self { duration, start_time: std::time::Instant::now() }
    }

    #[allow(dead_code)]
    fn elapsed_time(&self) -> std::time::Duration
    {
        let now = std::time::Instant::now();
        now - self.start_time
    }

    #[allow(dead_code)]
    pub fn reset(&mut self)
    {
        self.start_time = std::time::Instant::now();
    }

    #[allow(dead_code)]
    pub fn sleep_for_remaining(&self) -> Option<std::time::Instant>
    {
        let time_elapsed = self.elapsed_time();

        // Make sure we sleep for >= 0 amount of time
        // (NOTE: std::time::Duration doesn't support negatives, so we can't simply subtract).
        if time_elapsed < self.duration
        {
            let sleep_time = self.duration - time_elapsed;
            std::thread::sleep(sleep_time);

            return Some(std::time::Instant::now());
        }

        return None;
    }
}

#[cfg(test)]
mod tests
{
    use crate::hw::timer::Timer;

    #[test]
    fn sleep_remaining_time_valid()
    {
        let max_duration_ms = std::time::Duration::from_millis(100);
        let timer = Timer::new(max_duration_ms);
        std::thread::sleep(std::time::Duration::from_millis(10));
        let opt_end_time = timer.sleep_for_remaining();
        assert!(opt_end_time.is_some());

        let sleep_duration = opt_end_time.unwrap() - timer.start_time;
        assert!(sleep_duration.as_millis().abs_diff(max_duration_ms.as_millis()) <= max_duration_ms.as_millis());
    }

    #[test]
    fn sleep_remaining_time_invalid()
    {
        let max_duration_ms = std::time::Duration::from_millis(10);
        let timer = Timer::new(max_duration_ms);

        // Simulate elapsed time being larger than max duration.
        std::thread::sleep(std::time::Duration::from_millis(100));
        let opt_end_time = timer.sleep_for_remaining();
        assert!(opt_end_time.is_none());
    }
}

