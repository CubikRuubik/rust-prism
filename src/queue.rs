use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct QueueItem {
    pub id: String,
    pub value: i32,
}

#[derive(Debug, Default)]
pub struct Queue {
    items: Vec<QueueItem>,
}

impl Queue {
    pub fn new() -> Self {
        Queue { items: Vec::new() }
    }

    pub fn enqueue(&mut self, value: i32) -> QueueItem {
        let item = QueueItem {
            id: Uuid::new_v4().to_string(),
            value,
        };
        self.items.push(item.clone());
        item
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn drain(&mut self) -> Vec<QueueItem> {
        let drained = self.items.clone();
        self.items.clear();
        drained
    }
}
