use std::sync::Mutex;

use crate::error::{Result, TimeFlowError};
use crate::models::{DistributionStrategy, TaskType, TimerInfo, TimerState};
use crate::services::storage::StorageService;

pub struct TimerService {
    state: Mutex<Option<TimerState>>,
    storage: StorageService,
}

impl TimerService {
    pub fn new(storage: StorageService) -> Result<Self> {
        // Try to recover timer state from storage
        let state = storage.load_timer_state()?;
        
        Ok(Self {
            state: Mutex::new(state),
            storage,
        })
    }

    /// Start a new timer
    pub fn start(
        &self,
        task_name: String,
        category_path: Option<String>,
        task_type: TaskType,
        distribution_strategy: Option<DistributionStrategy>,
    ) -> Result<TimerInfo> {
        let mut state_guard = self.state.lock().unwrap();

        if state_guard.is_some() {
            return Err(TimeFlowError::TimerAlreadyRunning);
        }

        let timer_state = TimerState::new(task_name, category_path, task_type, distribution_strategy);
        
        // Persist for crash recovery
        self.storage.save_timer_state(&timer_state)?;
        
        let info = TimerInfo::from(&timer_state);
        *state_guard = Some(timer_state);

        Ok(info)
    }

    /// Stop the running timer and return elapsed time
    pub fn stop(&self) -> Result<(TimerState, u32)> {
        let mut state_guard = self.state.lock().unwrap();

        let state = state_guard
            .take()
            .ok_or(TimeFlowError::NoTimerRunning)?;

        let elapsed_minutes = state.elapsed_minutes().max(1); // Minimum 1 minute

        // Clear persisted state
        self.storage.clear_timer_state()?;

        Ok((state, elapsed_minutes))
    }

    /// Get current timer state (for polling)
    pub fn get_state(&self) -> Option<TimerInfo> {
        let state_guard = self.state.lock().unwrap();
        state_guard.as_ref().map(TimerInfo::from)
    }

    /// Check if timer is running
    pub fn is_running(&self) -> bool {
        let state_guard = self.state.lock().unwrap();
        state_guard.is_some()
    }

    /// Discard the running timer without saving
    pub fn discard(&self) -> Result<()> {
        let mut state_guard = self.state.lock().unwrap();
        *state_guard = None;
        self.storage.clear_timer_state()?;
        Ok(())
    }
}
