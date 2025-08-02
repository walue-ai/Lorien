use bevy::prelude::*;
use crate::{
    events::ToolChangeEvent,
    resources::ActiveTool,
};

pub fn tool_input_system(
    mut tool_events: EventReader<ToolChangeEvent>,
    mut active_tool: ResMut<ActiveTool>,
) {
    for event in tool_events.read() {
        active_tool.tool_type = event.tool_type;
        active_tool.brush_size = event.size;
        active_tool.brush_color = event.color;
        active_tool.opacity = event.opacity;
    }
}
