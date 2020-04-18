extern crate time;

use time::OffsetDateTime;

pub const SMART_LISTS: &[&str; 1] = &["today"];

#[derive(Debug)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub done: bool,
    pub today: String,
    pub list: String,
    pub priority: Priority,
}

impl Task {
    pub fn new(title: String, list: String, priority: Priority) -> Task {
        Task {
            id: 0,
            title,
            done: false,
            today: "".to_string(),
            list,
            priority,
        }
    }

    pub fn create(
        id: u32,
        title: String,
        done: u32,
        list: String,
        priority: String,
        today: String,
    ) -> Task {
        Task {
            id,
            title,
            done: done == 1,
            today,
            list,
            priority: Priority::from(&priority).unwrap(),
        }
    }

    pub fn is_in_list(&self, list: &str) -> bool {
        match list.to_lowercase().as_str() {
            "today" => self.is_marked_for_today(),
            _ => self.list.to_lowercase().eq(list.to_lowercase().as_str()),
        }
    }

    pub fn mark_for_today(self: &mut Self) {
        self.today = Task::today();
    }

    pub fn unmark_for_today(self: &mut Self) {
        self.today = String::new();
    }
    pub fn is_marked_for_today(&self) -> bool {
        self.today.eq(Task::today().as_str())
    }

    fn today() -> String {
        OffsetDateTime::now().format("%F")
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Priority {
    Low,
    High,
    Medium,
}

impl Priority {
    pub fn from(priority: &str) -> Result<Priority, &'static str> {
        match priority.to_lowercase().as_str() {
            "l" | "low" => Ok(Priority::Low),
            "h" | "high" => Ok(Priority::High),
            "m" | "medium" => Ok(Priority::Medium),
            _ => Err("Unsupported priority"),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Priority::Low => String::from("low"),
            Priority::High => String::from("high"),
            Priority::Medium => String::from("medium"),
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::task::{Priority, Task};

    #[test]
    fn test_priority() {
        assert_eq!(Priority::from("l").unwrap(), Priority::Low);
    }

    #[test]
    fn test_mark_for_today() {
        let mut task = Task::create(
            1,
            "test-todo".to_string(),
            1,
            "inbox".to_string(),
            "high".to_string(),
            "".to_string(),
        );

        task.mark_for_today();
        assert_eq!(task.is_marked_for_today(), true);

        task.unmark_for_today();
        assert_eq!(task.is_marked_for_today(), false);
    }

    #[test]
    fn test_in_list() {
        let mut task = Task::create(
            1,
            "test-todo".to_string(),
            1,
            "inbox".to_string(),
            "high".to_string(),
            "".to_string(),
        );

        task.mark_for_today();
        assert_eq!(task.is_in_list("Today"), true);
        assert_eq!(task.is_in_list("Inbox"), true);

        task.unmark_for_today();
        assert_eq!(task.is_in_list("today"), false);
    }
}
