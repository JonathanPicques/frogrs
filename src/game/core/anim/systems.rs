use bevy::prelude::*;
use bevy_ggrs::Rollback;

use crate::game::core::anim::structs::SpriteSheetAnimation;

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
