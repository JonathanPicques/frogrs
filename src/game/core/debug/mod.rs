use bevy::prelude::*;

pub fn debug_system(world: &mut World) {
    let mut all_entities = world.query::<Entity>();

    let entities = world.entities();
    let keyboard = world.get_resource::<Input<KeyCode>>().unwrap();
    let archetypes = world.archetypes();
    let components = world.components();

    if keyboard.just_pressed(KeyCode::F1) {
        for entity in all_entities.iter(&world) {
            println!("Entity: {:?}", entity);
            if let Some(parent) = world.entity(entity).get::<Parent>() {
                println!("\tParent: {:?}", parent.0);
            }
            if let Some(entity_location) = entities.get(entity) {
                if let Some(archetype) = archetypes.get(entity_location.archetype_id) {
                    for component in archetype.components() {
                        if let Some(component_info) = components.get_info(component) {
                            println!("\tComponent: {}", component_info.name());
                        }
                    }
                }
            }
        }
    }
}
