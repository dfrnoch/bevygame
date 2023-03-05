use crate::loading::TextureAssets;
use crate::AppState;
use bevy::prelude::*;

pub struct NotePlugin;

#[derive(Component, Debug)]
pub struct Note {
    pub time: f32,
}

impl Plugin for NotePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Playing).with_system(spawn_note))
            .add_system_set(SystemSet::on_update(AppState::Playing).with_system(click_note));
    }
}

// {
//     "notes": [
//       {"spawn_time": 0.0, "position": [100.0, 0.0]},
//       {"spawn_time": 1.0, "position": [200.0, 0.0]},
//       {"spawn_time": 2.0, "position": [300.0, 0.0]}
//     ],
//     "sliders": []
//   }
// add a logit to spawn notes at the right time, use the json file from assets/maps/test.json

fn spawn_note(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn(SpriteBundle {
            texture: textures.texture_circle.clone(),
            transform: Transform::from_translation(Vec3::new(100., 100., 0.)),
            ..Default::default()
        })
        .insert(Note { time: 2. });
}

fn click_note(
    mouse_button_input: Res<Input<MouseButton>>,
    interaction_query: Query<&Interaction, With<Note>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        interaction_query.for_each(|interaction| print!("{:?}", interaction));
    }
}
