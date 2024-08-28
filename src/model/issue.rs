use std::{cmp::Ordering, fmt};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Issue {
    pub id: Option<usize>,
    pub uid: Option<String>,
    pub name: Option<String>,
    pub class: Option<String>,
    pub problem: Option<String>,
    pub reg_time: Option<NaiveDateTime>,
    pub app_time: Option<NaiveDateTime>,
    pub closed: Option<bool>,
    pub closed_time: Option<NaiveDateTime>,
}

impl fmt::Display for Issue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl PartialEq for Issue {
    fn eq(&self, other: &Self) -> bool {
        self.app_time == other.app_time && self.closed == other.closed
    }
}

impl PartialOrd for Issue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.closed.cmp(&other.closed) {
            Ordering::Less => return Some(Ordering::Less),
            Ordering::Equal => {}
            Ordering::Greater => return Some(Ordering::Greater),
        }
        match self.app_time.cmp(&other.app_time) {
            Ordering::Less => return Some(Ordering::Greater),
            Ordering::Equal => {}
            Ordering::Greater => return Some(Ordering::Less),
        }
        Some(Ordering::Equal)
    }
}

impl Eq for Issue {}
impl Ord for Issue {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}