use bevy::ecs::{entity::Entity, message::Message};

#[derive(Debug, Message)]
pub struct Change {
  pub selector: Entity,
  pub target: Option<Entity>,
}

impl Change {
  pub fn to(selector: Entity, target: Entity) -> Self {
    Self {
      selector,
      target: Some(target),
    }
  }

  pub fn empty(selector: Entity) -> Self {
    Self {
      selector,
      target: None,
    }
  }
}
