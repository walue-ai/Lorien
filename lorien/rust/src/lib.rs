mod components;
mod systems;
mod resources;
mod events;

use bevy::prelude::*;
use bevy_godot4::prelude::*;
use godot::prelude::*;

use components::*;
use systems::*;
use resources::*;
use events::*;

#[bevy_app]
fn build_app(app: &mut App) {
    app.add_plugins(MinimalPlugins)
        .add_plugins(bevy::log::LogPlugin::default())
        .add_plugins(bevy::asset::AssetPlugin::default())
        .add_plugins(bevy::state::app::StatesPlugin)
        .init_resource::<ActiveTool>()
        .init_resource::<CanvasState>()
        .init_resource::<SpatialIndex>()
        .init_resource::<HistoryManager>()
        .add_event::<CanvasEvent>()
        .add_event::<ToolChangeEvent>()
        .add_event::<StrokeInputEvent>()
        .add_systems(Update, (
            tool_input_system,
            stroke_creation_system,
            stroke_optimization_system,
        ))
        .add_systems(Update, spatial_index_system)
        .add_systems(Update, history_system)
        .add_systems(Update, stroke_rendering_system);
}
