use bevy::prelude::*;

use crate::AppState;

pub struct HudPlugin;

#[derive(Component)]
struct Hud;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::Playing).with_system(hud_system));
    }
}

fn hud_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
) {
    let window = windows.get_primary_mut().unwrap();
    let window_size = Vec2::new(window.width(), window.height());

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

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
                                    font: font.clone(),
                                    font_size: 40.0,
                                    color: Color::WHITE,
                                },
                            },
                            TextSection {
                                value: "0".to_string(),
                                style: TextStyle {
                                    font: font.clone(),
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
