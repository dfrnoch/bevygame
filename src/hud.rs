use bevy::prelude::*;

use crate::{loading::FontAssets, AppState};

pub struct HudPlugin;

#[derive(Component)]
struct Hud;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Playing).with_system(hud_system));
    }
}

fn hud_system(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection {
                                value: "Score: ".to_string(),
                                style: TextStyle {
                                    font: font_assets.fira_sans.clone(),
                                    font_size: 40.0,
                                    color: Color::WHITE,
                                },
                            },
                            TextSection {
                                value: "0".to_string(),
                                style: TextStyle {
                                    font: font_assets.fira_sans.clone(),
                                    font_size: 40.0,
                                    color: Color::WHITE,
                                },
                            },
                        ],
                        alignment: Default::default(),
                    },
                    ..Default::default()
                })
                .insert(Hud);
        });
}
