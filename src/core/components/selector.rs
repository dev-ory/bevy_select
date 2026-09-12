use bevy::ecs::{component::Component, entity::Entity};

#[derive(Debug, Component, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Selector {
  pub target: Option<Entity>,
}
