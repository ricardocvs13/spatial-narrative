//! Temporal indexing for efficient time-based queries.
//!
//! This module provides temporal indexing using B-tree structures,
//! enabling fast queries like:
//! - Time range queries
//! - Before/after queries
//! - Sliding window iteration
//!
//! # Example
//!
//! ```rust
//! use spatial_narrative::index::TemporalIndex;
//! use spatial_narrative::core::{Timestamp, TimeRange};
//!
//! // Build temporal index
//! let mut index: TemporalIndex<&str> = TemporalIndex::new();
//! index.insert("Morning", &Timestamp::parse("2024-01-01T10:00:00Z").unwrap());
//! index.insert("Afternoon", &Timestamp::parse("2024-01-01T14:00:00Z").unwrap());
//! index.insert("Evening", &Timestamp::parse("2024-01-01T20:00:00Z").unwrap());
//!
//! // Query events in a time range
//! let start = Timestamp::parse("2024-01-01T12:00:00Z").unwrap();
//! let end = Timestamp::parse("2024-01-01T18:00:00Z").unwrap();
//! let results = index.query_range(&TimeRange::new(start, end));
//! assert!(!results.is_empty());
//! ```

use crate::core::{TimeRange, Timestamp};
use std::collections::BTreeMap;

/// Temporal index for efficient time-based queries.
///
/// Uses a B-tree for O(log n) range queries.
#[derive(Debug)]
pub struct TemporalIndex<T> {
    /// B-tree mapping timestamps to item indices
    tree: BTreeMap<i64, Vec<usize>>,
    /// The actual items
    items: Vec<T>,
    /// Timestamps for each item (for iteration)
    timestamps: Vec<Timestamp>,
}

impl<T: Clone> TemporalIndex<T> {
    /// Create an empty temporal index.
    pub fn new() -> Self {
        Self {
            tree: BTreeMap::new(),
            items: Vec::new(),
            timestamps: Vec::new(),
        }
    }

    /// Create a temporal index from items with a timestamp extractor.
    pub fn from_iter<I, F>(iter: I, timestamp_fn: F) -> Self
    where
        I: IntoIterator<Item = T>,
        F: Fn(&T) -> &Timestamp,
    {
        let mut index = Self::new();
        for item in iter {
            let ts = timestamp_fn(&item).clone();
            index.insert(item, &ts);
        }
        index
    }

    /// Insert an item into the index.
    pub fn insert(&mut self, item: T, timestamp: &Timestamp) {
        let idx = self.items.len();
        let key = timestamp.to_unix_millis();

        self.items.push(item);
        self.timestamps.push(timestamp.clone());
        self.tree.entry(key).or_insert_with(Vec::new).push(idx);
    }

    /// Query items within a time range (inclusive).
    pub fn query_range(&self, range: &TimeRange) -> Vec<&T> {
        let start_key = range.start.to_unix_millis();
        let end_key = range.end.to_unix_millis();

        self.tree
            .range(start_key..=end_key)
            .flat_map(|(_, indices)| indices.iter().map(|&i| &self.items[i]))
            .collect()
    }

    /// Query items before a timestamp.
    pub fn before(&self, timestamp: &Timestamp) -> Vec<&T> {
        let key = timestamp.to_unix_millis();

        self.tree
            .range(..key)
            .flat_map(|(_, indices)| indices.iter().map(|&i| &self.items[i]))
            .collect()
    }

    /// Query items after a timestamp.
    pub fn after(&self, timestamp: &Timestamp) -> Vec<&T> {
        let key = timestamp.to_unix_millis();

        self.tree
            .range((key + 1)..)
            .flat_map(|(_, indices)| indices.iter().map(|&i| &self.items[i]))
            .collect()
    }

    /// Query items at or before a timestamp.
    pub fn at_or_before(&self, timestamp: &Timestamp) -> Vec<&T> {
        let key = timestamp.to_unix_millis();

        self.tree
            .range(..=key)
            .flat_map(|(_, indices)| indices.iter().map(|&i| &self.items[i]))
            .collect()
    }

    /// Query items at or after a timestamp.
    pub fn at_or_after(&self, timestamp: &Timestamp) -> Vec<&T> {
        let key = timestamp.to_unix_millis();

        self.tree
            .range(key..)
            .flat_map(|(_, indices)| indices.iter().map(|&i| &self.items[i]))
            .collect()
    }

    /// Get the first (earliest) item.
    pub fn first(&self) -> Option<&T> {
        self.tree
            .iter()
            .next()
            .and_then(|(_, indices)| indices.first().map(|&i| &self.items[i]))
    }

    /// Get the last (latest) item.
    pub fn last(&self) -> Option<&T> {
        self.tree
            .iter()
            .next_back()
            .and_then(|(_, indices)| indices.last().map(|&i| &self.items[i]))
    }

    /// Returns items in chronological order.
    pub fn chronological(&self) -> Vec<&T> {
        self.tree
            .iter()
            .flat_map(|(_, indices)| indices.iter().map(|&i| &self.items[i]))
            .collect()
    }

    /// Returns items in reverse chronological order.
    pub fn reverse_chronological(&self) -> Vec<&T> {
        self.tree
            .iter()
            .rev()
            .flat_map(|(_, indices)| indices.iter().rev().map(|&i| &self.items[i]))
            .collect()
    }

    /// Returns the time range spanned by all items.
    pub fn time_range(&self) -> Option<TimeRange> {
        let first_key = self.tree.keys().next()?;
        let last_key = self.tree.keys().next_back()?;

        let start = Timestamp::from_unix_millis(*first_key)?;
        let end = Timestamp::from_unix_millis(*last_key)?;

        Some(TimeRange::new(start, end))
    }

    /// Create a sliding window iterator over the items.
    pub fn sliding_window(&self, window_size: chrono::Duration) -> SlidingWindowIter<'_, T> {
        SlidingWindowIter {
            index: self,
            window_millis: window_size.num_milliseconds(),
            current_start: self.tree.keys().next().copied(),
        }
    }

    /// Returns the number of indexed items.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns true if the index is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get all items in the index.
    pub fn items(&self) -> &[T] {
        &self.items
    }
}

impl<T: Clone> Default for TemporalIndex<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Iterator over sliding time windows.
pub struct SlidingWindowIter<'a, T> {
    index: &'a TemporalIndex<T>,
    window_millis: i64,
    current_start: Option<i64>,
}

impl<'a, T: Clone> Iterator for SlidingWindowIter<'a, T> {
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.current_start?;
        let end = start + self.window_millis;

        // Find next window start
        self.current_start = self.index.tree.range((start + 1)..).next().map(|(k, _)| *k);

        let items: Vec<_> = self
            .index
            .tree
            .range(start..end)
            .flat_map(|(_, indices)| indices.iter().map(|&i| &self.index.items[i]))
            .collect();

        if items.is_empty() && self.current_start.is_some() {
            self.next() // Skip empty windows
        } else if items.is_empty() {
            None
        } else {
            Some(items)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_timestamp(hour: u32) -> Timestamp {
        Timestamp::parse(&format!("2024-01-01T{:02}:00:00Z", hour)).unwrap()
    }

    #[test]
    fn test_temporal_index_new() {
        let index: TemporalIndex<String> = TemporalIndex::new();
        assert!(index.is_empty());
    }

    #[test]
    fn test_temporal_index_insert() {
        let mut index = TemporalIndex::new();
        index.insert("Morning", &make_timestamp(9));
        index.insert("Noon", &make_timestamp(12));
        index.insert("Evening", &make_timestamp(18));

        assert_eq!(index.len(), 3);
    }

    #[test]
    fn test_temporal_index_query_range() {
        let mut index = TemporalIndex::new();
        index.insert("9am", &make_timestamp(9));
        index.insert("12pm", &make_timestamp(12));
        index.insert("3pm", &make_timestamp(15));
        index.insert("6pm", &make_timestamp(18));

        let range = TimeRange::new(make_timestamp(11), make_timestamp(16));
        let results = index.query_range(&range);

        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_temporal_index_before_after() {
        let mut index = TemporalIndex::new();
        index.insert("9am", &make_timestamp(9));
        index.insert("12pm", &make_timestamp(12));
        index.insert("3pm", &make_timestamp(15));

        let before = index.before(&make_timestamp(12));
        assert_eq!(before.len(), 1);
        assert_eq!(*before[0], "9am");

        let after = index.after(&make_timestamp(12));
        assert_eq!(after.len(), 1);
        assert_eq!(*after[0], "3pm");
    }

    #[test]
    fn test_temporal_index_first_last() {
        let mut index = TemporalIndex::new();
        index.insert("First", &make_timestamp(8));
        index.insert("Middle", &make_timestamp(12));
        index.insert("Last", &make_timestamp(20));

        assert_eq!(index.first(), Some(&"First"));
        assert_eq!(index.last(), Some(&"Last"));
    }

    #[test]
    fn test_temporal_index_chronological() {
        let mut index = TemporalIndex::new();
        // Insert out of order
        index.insert("C", &make_timestamp(15));
        index.insert("A", &make_timestamp(9));
        index.insert("B", &make_timestamp(12));

        let ordered: Vec<_> = index.chronological();
        assert_eq!(ordered, vec![&"A", &"B", &"C"]);
    }
}
