use bevy::prelude::*;
use crate::events::CanvasEvent;

#[derive(Resource)]
pub struct HistoryManager {
    pub events: Vec<CanvasEvent>,
    pub current_index: usize,
    pub max_history: usize,
}

impl Default for HistoryManager {
    fn default() -> Self {
        Self {
            events: Vec::new(),
            current_index: 0,
            max_history: 1000,
        }
    }
}

impl HistoryManager {
    pub fn push_event(&mut self, event: CanvasEvent) {
        if self.current_index < self.events.len() {
            self.events.truncate(self.current_index);
        }

        self.events.push(event);
        self.current_index = self.events.len();

        if self.events.len() > self.max_history {
            self.events.remove(0);
            self.current_index -= 1;
        }
    }

    pub fn can_undo(&self) -> bool {
        self.current_index > 0
    }

    pub fn can_redo(&self) -> bool {
        self.current_index < self.events.len()
    }

    pub fn undo(&mut self) -> Option<&CanvasEvent> {
        if self.can_undo() {
            self.current_index -= 1;
            self.events.get(self.current_index)
        } else {
            None
        }
    }

    pub fn redo(&mut self) -> Option<&CanvasEvent> {
        if self.can_redo() {
            let event = self.events.get(self.current_index);
            self.current_index += 1;
            event
        } else {
            None
        }
    }
}
