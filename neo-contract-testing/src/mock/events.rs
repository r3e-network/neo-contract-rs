//! Mock implementation of Neo events
//!
//! This module provides a mock implementation of Neo events
//! (notifications) for testing smart contracts.

use alloc::string::String;
use alloc::vec::Vec;
use core::cell::RefCell;
use serde::{Deserialize, Serialize};

/// Event structure
#[derive(Debug, Clone, PartialEq, Serialize,Deserialize)]
pub struct Event {
    /// Event name
    pub name: String,
    /// Event arguments
    pub args: Vec<Vec<u8>>,
    /// Timestamp of the event
    pub timestamp: u64,
}

thread_local! {
    /// Global events state
    static EVENTS: RefCell<Vec<Event>> = RefCell::new(Vec::new());
}

/// Mock implementation of Neo events
pub struct MockEvents;

impl MockEvents {
    /// Reset events to empty state
    pub fn reset() {
        EVENTS.with(|events| {
            events.borrow_mut().clear();
        });
    }

    /// Emit an event (notification)
    pub fn emit(name: &str, args: Vec<Vec<u8>>, timestamp: u64) {
        EVENTS.with(|events| {
            events.borrow_mut().push(Event { name: name.into(), args, timestamp });
        });
    }

    /// Get all events
    pub fn get_all() -> Vec<Event> { EVENTS.with(|events| events.borrow().clone()) }

    /// Check if an event with the given name exists
    pub fn has_event(name: &str) -> bool { EVENTS.with(|events| events.borrow().iter().any(|e| e.name == name)) }

    /// Get events by name
    pub fn get_events_by_name(name: &str) -> Vec<Event> {
        EVENTS.with(|events| events.borrow().iter().filter(|e| e.name == name).cloned().collect())
    }

    /// Get the latest event
    pub fn get_latest_event() -> Option<Event> {
        EVENTS.with(|events| {
            let events = events.borrow();
            if events.is_empty() {
                None
            } else {
                Some(events[events.len() - 1].clone())
            }
        })
    }

    /// Get the latest event with the given name
    pub fn get_latest_event_by_name(name: &str) -> Option<Event> {
        EVENTS.with(|events| {
            let events = events.borrow();
            events.iter().rev().find(|e| e.name == name).cloned()
        })
    }

    /// Get the number of events
    pub fn get_event_count() -> usize { EVENTS.with(|events| events.borrow().len()) }

    /// Get the number of events with the given name
    pub fn get_event_count_by_name(name: &str) -> usize {
        EVENTS.with(|events| events.borrow().iter().filter(|e| e.name == name).count())
    }

    /// Clear events with the given name
    pub fn clear_events_by_name(name: &str) {
        EVENTS.with(|events| {
            let mut events = events.borrow_mut();
            events.retain(|e| e.name != name);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emit_and_get() {
        // Reset events
        MockEvents::reset();

        // Emit an event
        MockEvents::emit("TestEvent", vec![vec![1, 2, 3]], 1000);

        // Check event count
        assert_eq!(MockEvents::get_event_count(), 1);

        // Get all events
        let events = MockEvents::get_all();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].name, "TestEvent");
        assert_eq!(events[0].args.len(), 1);
        assert_eq!(events[0].args[0], vec![1, 2, 3]);
        assert_eq!(events[0].timestamp, 1000);

        // Check has_event
        assert!(MockEvents::has_event("TestEvent"));
        assert!(!MockEvents::has_event("OtherEvent"));
    }

    #[test]
    fn test_multiple_events() {
        // Reset events
        MockEvents::reset();

        // Emit multiple events
        MockEvents::emit("Event1", vec![vec![1]], 1000);
        MockEvents::emit("Event2", vec![vec![2]], 2000);
        MockEvents::emit("Event1", vec![vec![3]], 3000);

        // Check event count
        assert_eq!(MockEvents::get_event_count(), 3);
        assert_eq!(MockEvents::get_event_count_by_name("Event1"), 2);
        assert_eq!(MockEvents::get_event_count_by_name("Event2"), 1);

        // Get events by name
        let event1s = MockEvents::get_events_by_name("Event1");
        assert_eq!(event1s.len(), 2);
        assert_eq!(event1s[0].args[0], vec![1]);
        assert_eq!(event1s[1].args[0], vec![3]);

        // Get latest event
        let latest = MockEvents::get_latest_event().unwrap();
        assert_eq!(latest.name, "Event1");
        assert_eq!(latest.args[0], vec![3]);
        assert_eq!(latest.timestamp, 3000);

        // Get latest event by name
        let latest_event1 = MockEvents::get_latest_event_by_name("Event1").unwrap();
        assert_eq!(latest_event1.args[0], vec![3]);
        assert_eq!(latest_event1.timestamp, 3000);

        let latest_event2 = MockEvents::get_latest_event_by_name("Event2").unwrap();
        assert_eq!(latest_event2.args[0], vec![2]);
        assert_eq!(latest_event2.timestamp, 2000);
    }

    #[test]
    fn test_reset_and_clear() {
        // Reset events
        MockEvents::reset();

        // Emit multiple events
        MockEvents::emit("Event1", vec![vec![1]], 1000);
        MockEvents::emit("Event2", vec![vec![2]], 2000);
        MockEvents::emit("Event1", vec![vec![3]], 3000);

        // Clear events by name
        MockEvents::clear_events_by_name("Event1");

        // Check event count
        assert_eq!(MockEvents::get_event_count(), 1);
        assert_eq!(MockEvents::get_event_count_by_name("Event1"), 0);
        assert_eq!(MockEvents::get_event_count_by_name("Event2"), 1);

        // Reset all events
        MockEvents::reset();

        // Check event count
        assert_eq!(MockEvents::get_event_count(), 0);
    }
}
