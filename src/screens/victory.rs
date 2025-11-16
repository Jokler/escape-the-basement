//! The victory menu.

use bevy::prelude::*;

use crate::{asset_tracking::LoadResource, audio::sound_effect, screens::Screen, theme::widget};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<VictoryAssets>();
    app.add_systems(OnEnter(Screen::Victory), spawn_victory_menu);
    app.add_systems(OnEnter(Screen::Victory), start_victory_music);
}

fn spawn_victory_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Victory Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Screen::Victory),
        children![
            widget::header("You Win!"),
            widget::button("Quit to title", quit_to_title),
        ],
    ));
}

fn quit_to_title(_: On<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct VictoryAssets {
    #[dependency]
    victory_sound: Handle<AudioSource>,
}

impl FromWorld for VictoryAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            victory_sound: assets.load("audio/music/won.ogg"),
        }
    }
}

fn start_victory_music(mut commands: Commands, victory_assets: Res<VictoryAssets>) {
    commands.spawn((
        Name::new("Victory Music"),
        sound_effect(victory_assets.victory_sound.clone()),
    ));
}
