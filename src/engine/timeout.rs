use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use crate::error::{HypeError, Result};

#[derive(Debug, Clone)]
pub enum TimeoutError {
    TimeoutExpired(Duration),
    Interrupted,
    AlreadyStopped,
    NotStarted,
}

impl std::fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TimeoutError::TimeoutExpired(duration) => {
                write!(f, "Execution timed out after {:?}", duration)
            }
            TimeoutError::Interrupted => write!(f, "Execution was interrupted"),
            TimeoutError::AlreadyStopped => write!(f, "Timeout has already been stopped"),
            TimeoutError::NotStarted => write!(f, "Timeout has not been started"),
        }
    }
}

impl std::error::Error for TimeoutError {}

#[derive(Debug)]
pub struct TimeoutHandle {
    id: usize,
    start_time: Instant,
    duration: Duration,
    stopped: Arc<Mutex<bool>>,
    thread_handle: Option<JoinHandle<()>>,
}

impl Clone for TimeoutHandle {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            start_time: self.start_time,
            duration: self.duration,
            stopped: self.stopped.clone(),
            thread_handle: None, // Don't clone the thread handle
        }
    }
}

impl TimeoutHandle {
    fn new(id: usize, duration: Duration) -> Self {
        Self {
            id,
            start_time: Instant::now(),
            duration,
            stopped: Arc::new(Mutex::new(false)),
            thread_handle: None,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn start_time(&self) -> Instant {
        self.start_time
    }

    pub fn duration(&self) -> Duration {
        self.duration
    }

    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    pub fn remaining(&self) -> Option<Duration> {
        if self.elapsed() >= self.duration {
            None
        } else {
            Some(self.duration - self.elapsed())
        }
    }

    pub fn is_expired(&self) -> bool {
        self.elapsed() >= self.duration
    }

    pub fn is_stopped(&self) -> bool {
        *self.stopped.lock().unwrap()
    }

    fn stop(&self) -> Result<()> {
        let mut stopped = self.stopped.lock().unwrap();
        if *stopped {
            return Err(HypeError::Execution(
                TimeoutError::AlreadyStopped.to_string(),
            ));
        }
        *stopped = true;
        Ok(())
    }
}

impl Drop for TimeoutHandle {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

pub struct TimeoutManager {
    next_id: Arc<Mutex<usize>>,
    active_timeouts: Arc<Mutex<Vec<TimeoutHandle>>>,
}

impl TimeoutManager {
    pub fn new(default_timeout: Option<Duration>) -> Self {
        Self {
            next_id: Arc::new(Mutex::new(1)),
            active_timeouts: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn start_timeout(&self, duration: Duration) -> Result<TimeoutHandle> {
        let id = {
            let mut next_id = self.next_id.lock().unwrap();
            let id = *next_id;
            *next_id += 1;
            id
        };

        let mut handle = TimeoutHandle::new(id, duration);
        let stopped_clone = handle.stopped.clone();

        // Spawn a thread to monitor the timeout
        let thread_handle = thread::spawn(move || {
            let start = Instant::now();

            while start.elapsed() < duration {
                thread::sleep(Duration::from_millis(100));

                // Check if stopped
                if *stopped_clone.lock().unwrap() {
                    return;
                }
            }

            // Timeout expired
            if !*stopped_clone.lock().unwrap() {
                // In a real implementation, this would interrupt the Lua execution
                // For now, we just mark it as expired
                eprintln!("Script execution timed out after {:?}", duration);
            }
        });

        handle.thread_handle = Some(thread_handle);

        // Add to active timeouts
        {
            let mut active = self.active_timeouts.lock().unwrap();
            active.push(handle.clone());
        }

        Ok(handle)
    }

    pub fn stop_timeout(&self, handle: TimeoutHandle) -> Result<()> {
        handle.stop()?;

        // Remove from active timeouts
        {
            let mut active = self.active_timeouts.lock().unwrap();
            active.retain(|h| h.id() != handle.id());
        }

        Ok(())
    }

    pub fn interrupt(&self) -> Result<()> {
        // Stop all active timeouts
        let mut active = self.active_timeouts.lock().unwrap();

        for handle in active.iter() {
            let _ = handle.stop();
        }

        active.clear();

        Ok(())
    }

    pub fn force_stop(&self) -> Result<()> {
        self.interrupt()
    }

    pub fn get_active_count(&self) -> usize {
        self.active_timeouts.lock().unwrap().len()
    }

    pub fn get_active_timeouts(&self) -> Vec<TimeoutHandle> {
        self.active_timeouts.lock().unwrap().clone()
    }

    pub fn cleanup_expired(&self) -> Result<usize> {
        let mut active = self.active_timeouts.lock().unwrap();
        let initial_count = active.len();

        active.retain(|handle| !handle.is_expired() && !handle.is_stopped());

        Ok(initial_count - active.len())
    }

    pub fn wait_for_timeout(&self, handle: &TimeoutHandle) -> Result<()> {
        while !handle.is_expired() && !handle.is_stopped() {
            thread::sleep(Duration::from_millis(10));
        }

        if handle.is_expired() {
            Err(HypeError::Execution(
                TimeoutError::TimeoutExpired(handle.duration()).to_string(),
            ))
        } else {
            Ok(())
        }
    }

    pub fn check_timeout(&self, handle: &TimeoutHandle) -> Result<()> {
        if handle.is_expired() {
            Err(HypeError::Execution(
                TimeoutError::TimeoutExpired(handle.duration()).to_string(),
            ))
        } else {
            Ok(())
        }
    }
}

impl Default for TimeoutManager {
    fn default() -> Self {
        Self::new(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_timeout_manager_creation() {
        let manager = TimeoutManager::new(None);
        assert_eq!(manager.get_active_count(), 0);
    }

    #[test]
    fn test_start_timeout() -> Result<()> {
        let manager = TimeoutManager::new(None);
        let handle = manager.start_timeout(Duration::from_millis(100))?;

        assert_eq!(manager.get_active_count(), 1);
        assert!(!handle.is_expired());
        assert!(!handle.is_stopped());

        // Stop the timeout
        manager.stop_timeout(handle)?;
        assert_eq!(manager.get_active_count(), 0);

        Ok(())
    }

    #[test]
    fn test_timeout_expiration() -> Result<()> {
        let manager = TimeoutManager::new(None);
        let handle = manager.start_timeout(Duration::from_millis(10))?;

        // Wait for timeout to expire
        thread::sleep(Duration::from_millis(50));

        assert!(handle.is_expired());

        Ok(())
    }

    #[test]
    fn test_interrupt_all_timeouts() -> Result<()> {
        let manager = TimeoutManager::new(None);

        let handle1 = manager.start_timeout(Duration::from_secs(1))?;
        let handle2 = manager.start_timeout(Duration::from_secs(2))?;

        assert_eq!(manager.get_active_count(), 2);

        manager.interrupt()?;

        assert_eq!(manager.get_active_count(), 0);
        assert!(handle1.is_stopped());
        assert!(handle2.is_stopped());

        Ok(())
    }

    #[test]
    fn test_cleanup_expired() -> Result<()> {
        let manager = TimeoutManager::new(None);

        let _handle1 = manager.start_timeout(Duration::from_millis(10))?;
        let handle2 = manager.start_timeout(Duration::from_secs(1))?;

        // Wait for first timeout to expire
        thread::sleep(Duration::from_millis(50));

        let cleaned = manager.cleanup_expired()?;
        assert_eq!(cleaned, 1);
        assert_eq!(manager.get_active_count(), 1);

        // Stop the remaining timeout
        manager.stop_timeout(handle2)?;

        Ok(())
    }

    #[test]
    fn test_timeout_handle_properties() -> Result<()> {
        let manager = TimeoutManager::new(None);
        let handle = manager.start_timeout(Duration::from_millis(100))?;

        assert!(handle.id() > 0);
        assert_eq!(handle.duration(), Duration::from_millis(100));
        assert!(handle.elapsed() < Duration::from_millis(100));
        assert!(handle.remaining().is_some());

        manager.stop_timeout(handle)?;

        Ok(())
    }
}
