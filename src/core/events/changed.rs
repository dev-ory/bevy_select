use bevy::ecs::{entity::Entity, event::EntityEvent};

#[derive(Debug, EntityEvent)]
pub struct Changed {
  #[event_target]
  pub selector: Entity,
  pub old: Option<Entity>,
  pub new: Option<Entity>,
}
