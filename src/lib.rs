// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-metrics-collection: metrics collection component model.
//!
//! Real implementation: metrics, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Metric {
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
pub enum MetricsCollectionError {
    EmptyMetricName,
    DuplicateMetric(String),
    EmptyEventSubject,
}

impl fmt::Display for MetricsCollectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyMetricName => write!(f, "metric name must not be empty"),
            Self::DuplicateMetric(id) => write!(f, "duplicate metric {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for MetricsCollectionError {}

pub struct MetricsCollection {
    pub name: String,
    metrics: BTreeMap<String, Metric>,
    events: BTreeMap<String, Event>,
}

impl MetricsCollection {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            metrics: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_metric(&mut self, item: Metric) -> Result<(), MetricsCollectionError> {
        if item.name.trim().is_empty() {
            return Err(MetricsCollectionError::EmptyMetricName);
        }
        let id = item.id.clone();
        if self.metrics.contains_key(&id) {
            return Err(MetricsCollectionError::DuplicateMetric(id.clone()));
        }
        self.metrics.insert(id, item);
        Ok(())
    }

    pub fn metric_count(&self) -> usize {
        self.metrics.len()
    }

    pub fn get_metric(&self, id: &str) -> Option<&Metric> {
        self.metrics.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), MetricsCollectionError> {
        if event.subject.trim().is_empty() {
            return Err(MetricsCollectionError::EmptyEventSubject);
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

    fn sample_metric(id: &str, name: &str) -> Metric {
        Metric {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = MetricsCollection::new("MetricsCollection");
        assert_eq!(m.name, "MetricsCollection");
        assert_eq!(m.metric_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_metric() {
        let mut m = MetricsCollection::new("MetricsCollection");
        m.add_metric(sample_metric("a1", "Alpha")).unwrap();
        assert_eq!(m.metric_count(), 1);
        assert!(m.get_metric("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = MetricsCollection::new("MetricsCollection");
        assert!(m.add_metric(sample_metric("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = MetricsCollection::new("MetricsCollection");
        m.add_metric(sample_metric("x", "One")).unwrap();
        assert!(m.add_metric(sample_metric("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = MetricsCollection::new("MetricsCollection");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
