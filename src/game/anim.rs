use bevy::ecs::query::With;
use bevy::ecs::system::Query;
use bevy::prelude::*;
use bevy_ggrs::Rollback;

use super::GAME_FPS;

#[derive(Hash, Default, Reflect)]
#[reflect(Hash)]
pub struct SpriteSheetAnimation {
    pub dt: u8,
    pub speed: u8,
}

pub fn speed_as_secs(secs: f32) -> u8 {
    return (secs * GAME_FPS as f32) as u8;
}

pub fn animate_sprite_system(
    mut query: Query<
        (
            &Handle<TextureAtlas>,
            &mut TextureAtlasSprite,
            &mut SpriteSheetAnimation,
        ),
        With<Rollback>,
    >,
    textures: ResMut<Assets<TextureAtlas>>,
) {
    for (texture_atlas, mut sprite, mut sprite_sheet_animation) in query.iter_mut() {
        if let Some(texture_atlas) = textures.get(texture_atlas) {
            let nb_frames = texture_atlas.textures.len() as u32;

            if sprite_sheet_animation.dt > sprite_sheet_animation.speed {
                sprite.index = (sprite.index + 1) % nb_frames;
                sprite_sheet_animation.dt = 0;
            }
            sprite_sheet_animation.dt += 1;
        }
    }
}
