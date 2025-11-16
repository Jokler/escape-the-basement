use avian2d::prelude::{CollisionStart, Sensor};
use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
};
use bevy_ecs_ldtk::LdtkEntity;
use bevy_ecs_ldtk::{LevelSelection, app::LdtkEntityAppExt};

use crate::{
    asset_tracking::LoadResource,
    audio::sound_effect,
    game::{colliders::ColliderBundle, player::Player},
    screens::Screen,
};

pub fn plugin(app: &mut App) {
    app.load_resource::<DoorAssets>();
    app.register_ldtk_entity::<DoorBundle>("Door");
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
#[component(on_add = on_door_add)]
pub struct Door;

pub fn on_door_add(mut world: DeferredWorld, context: HookContext) {
    let door_entity = context.entity;
    world
        .commands()
        .entity(door_entity)
        .observe(on_player_entered_door);
}

#[derive(Clone, Debug, Default, Bundle, LdtkEntity)]
pub struct DoorBundle {
    door: Door,

    #[sprite_sheet]
    sprite_sheet: Sprite,

    #[from_entity_instance]
    collider_bundle: ColliderBundle,

    sensor: Sensor,
}

fn on_player_entered_door(
    event: On<CollisionStart>,
    mut commands: Commands,
    door_assets: Res<DoorAssets>,
    player_query: Query<&Player>,
    level_selection: ResMut<LevelSelection>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    // `colider1` and `body1` refer to the event target and its body.
    // `collider2` and `body2` refer to the other collider and its body.
    let other_entity = event.collider2;

    if player_query.contains(other_entity) {
        commands.spawn((
            Name::from("Door Sound"),
            sound_effect(door_assets.use_sound.clone()),
        ));

        let indices = match level_selection.into_inner() {
            LevelSelection::Indices(indices) => indices,
            _ => panic!("level selection should always be Indices in this game"),
        };

        indices.level += 1;

        if indices.level > 4 {
            next_screen.set(Screen::Victory);
        }
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct DoorAssets {
    #[dependency]
    pub use_sound: Handle<AudioSource>,
}

impl FromWorld for DoorAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            use_sound: assets.load("audio/sound_effects/door.ogg"),
        }
    }
}
