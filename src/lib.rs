// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-display-hotplug: display hotplug component model.
//!
//! Real implementation: hotplugs, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hotplug {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DisplayHotplugError {
    EmptyHotplugName,
    DuplicateHotplug(String),
    EmptyEventSubject,
}

impl fmt::Display for DisplayHotplugError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyHotplugName => write!(f, "hotplug name must not be empty"),
            Self::DuplicateHotplug(id) => write!(f, "duplicate hotplug {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for DisplayHotplugError {}

pub struct DisplayHotplug {
    pub name: String,
    hotplugs: BTreeMap<String, Hotplug>,
    events: BTreeMap<String, Event>,
}

impl DisplayHotplug {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            hotplugs: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_hotplug(&mut self, item: Hotplug) -> Result<(), DisplayHotplugError> {
        if item.name.trim().is_empty() {
            return Err(DisplayHotplugError::EmptyHotplugName);
        }
        let id = item.id.clone();
        if self.hotplugs.contains_key(&id) {
            return Err(DisplayHotplugError::DuplicateHotplug(id.clone()));
        }
        self.hotplugs.insert(id, item);
        Ok(())
    }

    pub fn hotplug_count(&self) -> usize {
        self.hotplugs.len()
    }

    pub fn get_hotplug(&self, id: &str) -> Option<&Hotplug> {
        self.hotplugs.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), DisplayHotplugError> {
        if event.subject.trim().is_empty() {
            return Err(DisplayHotplugError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_hotplug(id: &str, name: &str) -> Hotplug {
        Hotplug {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = DisplayHotplug::new("DisplayHotplug");
        assert_eq!(m.name, "DisplayHotplug");
        assert_eq!(m.hotplug_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_hotplug() {
        let mut m = DisplayHotplug::new("DisplayHotplug");
        m.add_hotplug(sample_hotplug("a1", "Alpha")).unwrap();
        assert_eq!(m.hotplug_count(), 1);
        assert!(m.get_hotplug("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = DisplayHotplug::new("DisplayHotplug");
        assert!(m.add_hotplug(sample_hotplug("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = DisplayHotplug::new("DisplayHotplug");
        m.add_hotplug(sample_hotplug("x", "One")).unwrap();
        assert!(m.add_hotplug(sample_hotplug("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = DisplayHotplug::new("DisplayHotplug");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
