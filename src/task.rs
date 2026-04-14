use std::fmt;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn simple_id() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_nanos() as u64
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Priority::Low => write!(f, "low"),
            Priority::Medium => write!(f, "medium"),
            Priority::High => write!(f, "high"),
        }
    }
}

#[derive(Debug)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub priority: Priority,
    pub created_at: SystemTime,
    pub due_at: SystemTime,
    pub done: bool,
}

impl Task {
    pub fn new(
        title: impl Into<String>,
        description: impl Into<String>,
        priority: Priority,
        due_at: SystemTime,
    ) -> Self {
        Task {
            id: simple_id(),
            title: title.into(),
            description: description.into(),
            priority,
            created_at: SystemTime::now(),
            due_at,
            done: false,
        }
    }

    pub fn complete(&mut self) {
        self.done = true;
    }

    pub fn is_overdue(&self) -> bool {
        !self.done && SystemTime::now() > self.due_at
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.done {
            "done"
        } else if self.is_overdue() {
            "overdue"
        } else {
            "pending"
        };
        let due_secs = self
            .due_at
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_secs();
        write!(
            f,
            "[{}] {} (priority: {}, status: {}, due: {}s)",
            self.id, self.title, self.priority, status, due_secs
        )
    }
}
