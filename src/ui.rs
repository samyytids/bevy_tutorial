use bevy::prelude::*;
use crate::Money;
pub struct GameUi;

#[derive(Component)]
pub struct MoneyText;


impl Plugin for GameUi {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_game_ui)
            .add_systems(Update, update_money_ui);
    }
}

fn spawn_game_ui(mut commands: Commands) {
    commands
        .spawn((
            // This is the fundamental UI component. 
            NodeBundle {
                // This details how to make the ui pretty.
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(10.0),
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::BLUE.into(),
                ..default()
            },
            Name::new("UI Root"),
        ))
        .with_children(|commands| {
            commands.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Money!",
                        TextStyle {
                            font_size: 32.0,
                            ..default()
                        },
                    ),
                    ..default()
                },
                // This is a tag component all it does is make it easy to 
                // query for specific stuff. 
                MoneyText,
            ));
        });
}

// This simply queries our money value. 
fn update_money_ui(mut texts: Query<&mut Text, With<MoneyText>>, money: Res<Money>) {
    // This then iterates through the results and then formats that text into
    // a string that we use to update the text within our ui. 
    for mut text in &mut texts {
        text.sections[0].value = format!("Money: £{:?}", money.0);
    }
}