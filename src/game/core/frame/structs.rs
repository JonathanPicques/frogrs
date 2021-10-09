use bevy::ecs::component::Component;
use bevy::reflect::Reflect;

#[derive(Hash, Default, Reflect, Component)]
#[reflect(Hash)]
pub struct FrameCount {
    pub frame: u32,
}
