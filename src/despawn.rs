use bevy::prelude::*;

const DESPAWN_DISTANCE: f32 = 100.0;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_off_screen_entities);
    }
}

fn despawn_off_screen_entities(mut commands: Commands, query: Query<(Entity, &GlobalTransform)>) {
    for (entity, transform) in query.iter() {
        let distance = transform.translation().distance(Vec3::ZERO);

        // entity is out of viewport
        if distance > DESPAWN_DISTANCE {
            commands.entity(entity).despawn_recursive()
        }
    }
}
