use bevy::ecs::component::Component;
use bevy::reflect::Reflect;

#[derive(Hash, Default, Reflect, Component)]
#[reflect(Hash)]
pub struct SpriteSheetAnimation {
    pub dt: u8,
    pub speed: u8,
}
