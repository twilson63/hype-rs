use super::{LuaStateConfig, LuaStateManager, LuaStateMetrics};
use crate::error::{HypeError, Result};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct LuaStateHandle {
    pub id: usize,
    pub state: Arc<Mutex<LuaStateManager>>,
    pub created_at: Instant,
    pub last_used: Arc<Mutex<Instant>>,
    pub usage_count: Arc<Mutex<usize>>,
    pub config: LuaStateConfig,
    pub is_persistent: bool,
}

impl LuaStateHandle {
    pub fn new(
        id: usize,
        state: LuaStateManager,
        config: LuaStateConfig,
        is_persistent: bool,
    ) -> Self {
        let now = Instant::now();
        Self {
            id,
            state: Arc::new(Mutex::new(state)),
            created_at: now,
            last_used: Arc::new(Mutex::new(now)),
            usage_count: Arc::new(Mutex::new(0)),
            config,
            is_persistent,
        }
    }

    pub fn touch(&self) {
        if let Ok(mut last_used) = self.last_used.lock() {
            *last_used = Instant::now();
        }
        if let Ok(mut count) = self.usage_count.lock() {
            *count += 1;
        }
    }

    pub fn get_usage_count(&self) -> usize {
        self.usage_count.lock().unwrap().clone()
    }

    pub fn get_idle_time(&self) -> Duration {
        self.last_used
            .lock()
            .map(|last_used| last_used.elapsed())
            .unwrap_or(Duration::ZERO)
    }

    pub fn get_age(&self) -> Duration {
        self.created_at.elapsed()
    }

    pub fn get_metrics(&self) -> LuaStateMetrics {
        self.state.lock().unwrap().get_metrics()
    }

    pub fn force_gc(&self) -> Result<()> {
        self.state.lock().unwrap().force_gc()
    }
}

impl Drop for LuaStateHandle {
    fn drop(&mut self) {
        // Force garbage collection when handle is dropped
        if let Ok(state) = self.state.lock() {
            let _ = state.force_gc();
        }
    }
}

pub struct LuaStateLifecycleManager {
    states: Arc<Mutex<HashMap<usize, LuaStateHandle>>>,
    cleanup_interval: Duration,
    max_idle_time: Duration,
    next_id: AtomicUsize,
}

impl LuaStateLifecycleManager {
    pub fn new(cleanup_interval: Duration, max_idle_time: Duration) -> Self {
        Self {
            states: Arc::new(Mutex::new(HashMap::new())),
            cleanup_interval,
            max_idle_time,
            next_id: AtomicUsize::new(1),
        }
    }

    pub fn create_state(&self, config: LuaStateConfig, is_persistent: bool) -> Result<usize> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let state = LuaStateManager::new(config.clone())?;
        let handle = LuaStateHandle::new(id, state, config, is_persistent);

        let mut states = self.states.lock().unwrap();
        states.insert(id, handle);

        Ok(id)
    }

    pub fn get_state(&self, id: usize) -> Result<Arc<Mutex<LuaStateManager>>> {
        let states = self.states.lock().unwrap();

        if let Some(handle) = states.get(&id) {
            handle.touch();
            Ok(Arc::clone(&handle.state))
        } else {
            Err(HypeError::Execution(format!("Lua state {} not found", id)))
        }
    }

    pub fn get_state_handle(&self, id: usize) -> Result<LuaStateHandle> {
        let states = self.states.lock().unwrap();

        if let Some(handle) = states.get(&id) {
            handle.touch();
            Ok(LuaStateHandle {
                id: handle.id,
                state: Arc::clone(&handle.state),
                created_at: handle.created_at,
                last_used: Arc::clone(&handle.last_used),
                usage_count: Arc::clone(&handle.usage_count),
                config: handle.config.clone(),
                is_persistent: handle.is_persistent,
            })
        } else {
            Err(HypeError::Execution(format!("Lua state {} not found", id)))
        }
    }

    pub fn remove_state(&self, id: usize) -> Result<()> {
        let mut states = self.states.lock().unwrap();
        if states.remove(&id).is_some() {
            Ok(())
        } else {
            Err(HypeError::Execution(format!("Lua state {} not found", id)))
        }
    }

    pub fn list_states(&self) -> Vec<usize> {
        let states = self.states.lock().unwrap();
        states.keys().copied().collect()
    }

    pub fn get_state_info(&self, id: usize) -> Result<StateInfo> {
        let states = self.states.lock().unwrap();

        if let Some(handle) = states.get(&id) {
            Ok(StateInfo {
                id: handle.id,
                created_at: handle.created_at,
                last_used: *handle.last_used.lock().unwrap(),
                usage_count: *handle.usage_count.lock().unwrap(),
                idle_time: handle.get_idle_time(),
                age: handle.get_age(),
                is_persistent: handle.is_persistent,
                metrics: handle.get_metrics(),
            })
        } else {
            Err(HypeError::Execution(format!("Lua state {} not found", id)))
        }
    }

    pub fn cleanup_idle_states(&self) -> Result<usize> {
        let mut states_to_remove = Vec::new();
        let mut count = 0;

        {
            let states = self.states.lock().unwrap();
            for (id, handle) in states.iter() {
                if !handle.is_persistent && handle.get_idle_time() > self.max_idle_time {
                    states_to_remove.push(*id);
                }
            }
        }

        {
            let mut states = self.states.lock().unwrap();
            for id in &states_to_remove {
                if states.remove(id).is_some() {
                    count += 1;
                }
            }
        }

        Ok(count)
    }

    pub fn force_gc_all(&self) -> Result<()> {
        let states = self.states.lock().unwrap();
        for handle in states.values() {
            let _ = handle.force_gc();
        }
        Ok(())
    }

    pub fn get_total_states(&self) -> usize {
        self.states.lock().unwrap().len()
    }

    pub fn get_active_states(&self) -> usize {
        let states = self.states.lock().unwrap();
        states
            .values()
            .filter(|handle| handle.get_idle_time() < Duration::from_secs(60))
            .count()
    }

    pub fn get_persistent_states(&self) -> usize {
        let states = self.states.lock().unwrap();
        states
            .values()
            .filter(|handle| handle.is_persistent)
            .count()
    }
}

impl Drop for LuaStateLifecycleManager {
    fn drop(&mut self) {
        // Force cleanup of all states
        if let Ok(mut states) = self.states.lock() {
            states.clear();
        }
    }
}

#[derive(Debug, Clone)]
pub struct StateInfo {
    pub id: usize,
    pub created_at: Instant,
    pub last_used: Instant,
    pub usage_count: usize,
    pub idle_time: Duration,
    pub age: Duration,
    pub is_persistent: bool,
    pub metrics: LuaStateMetrics,
}

pub struct LuaStatePool {
    lifecycle_manager: Arc<LuaStateLifecycleManager>,
    available_states: Arc<Mutex<Vec<usize>>>,
    max_size: usize,
    config: LuaStateConfig,
}

impl LuaStatePool {
    pub fn new(
        config: LuaStateConfig,
        max_size: usize,
        lifecycle_manager: Arc<LuaStateLifecycleManager>,
    ) -> Self {
        Self {
            lifecycle_manager,
            available_states: Arc::new(Mutex::new(Vec::new())),
            max_size,
            config,
        }
    }

    pub fn acquire(&self) -> Result<usize> {
        let mut available = self.available_states.lock().unwrap();

        if let Some(id) = available.pop() {
            Ok(id)
        } else {
            // Create new state if under limit
            if self.lifecycle_manager.get_total_states() < self.max_size {
                self.lifecycle_manager
                    .create_state(self.config.clone(), false)
            } else {
                Err(HypeError::Execution("Lua state pool exhausted".to_string()))
            }
        }
    }

    pub fn release(&self, id: usize) -> Result<()> {
        // Reset state if needed
        if let Ok(state) = self.lifecycle_manager.get_state(id) {
            if let Ok(manager) = state.lock() {
                manager.force_gc()?;
            }
        }

        let mut available = self.available_states.lock().unwrap();
        if available.len() < self.max_size {
            available.push(id);
        }

        Ok(())
    }

    pub fn size(&self) -> usize {
        self.available_states.lock().unwrap().len()
    }

    pub fn clear(&self) -> Result<()> {
        let mut available = self.available_states.lock().unwrap();
        let ids: Vec<usize> = available.drain(..).collect();
        drop(available);

        for id in ids {
            let _ = self.lifecycle_manager.remove_state(id);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_lifecycle_manager_creation() {
        let manager =
            LuaStateLifecycleManager::new(Duration::from_millis(100), Duration::from_secs(1));

        assert_eq!(manager.get_total_states(), 0);
    }

    #[test]
    fn test_state_creation_and_retrieval() {
        let manager =
            LuaStateLifecycleManager::new(Duration::from_millis(100), Duration::from_secs(1));

        let config = LuaStateConfig::default();
        let id = manager.create_state(config, false).unwrap();

        assert!(manager.get_state(id).is_ok());
        assert_eq!(manager.get_total_states(), 1);
    }

    #[test]
    fn test_state_removal() {
        let manager =
            LuaStateLifecycleManager::new(Duration::from_millis(100), Duration::from_secs(1));

        let config = LuaStateConfig::default();
        let id = manager.create_state(config, false).unwrap();

        assert!(manager.remove_state(id).is_ok());
        assert_eq!(manager.get_total_states(), 0);
    }

    #[test]
    fn test_state_pool() {
        let lifecycle_manager = Arc::new(LuaStateLifecycleManager::new(
            Duration::from_millis(100),
            Duration::from_secs(1),
        ));

        let config = LuaStateConfig::default();
        let pool = LuaStatePool::new(config, 3, lifecycle_manager);

        let id1 = pool.acquire().unwrap();
        let id2 = pool.acquire().unwrap();

        assert_ne!(id1, id2);
        assert_eq!(pool.size(), 0);

        pool.release(id1).unwrap();
        assert_eq!(pool.size(), 1);
    }

    #[test]
    fn test_state_handle_touch() {
        let manager =
            LuaStateLifecycleManager::new(Duration::from_millis(100), Duration::from_secs(1));

        let config = LuaStateConfig::default();
        let id = manager.create_state(config, false).unwrap();
        let handle = manager.get_state_handle(id).unwrap();

        let initial_count = handle.get_usage_count();
        handle.touch();
        assert_eq!(handle.get_usage_count(), initial_count + 1);
    }
}
