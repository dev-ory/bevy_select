use crate::neighbor::traits::Direction;
use bevy::ecs::{component::Component, entity::Entity};
use std::marker::PhantomData as Marker;

#[derive(Debug, Component)]
pub struct Fallback<A: Direction> {
  pub entity: Entity,
  marker: Marker<A>,
}

impl<A: Direction> Fallback<A> {
  pub fn new(entity: Entity) -> Self {
    Self {
      entity,
      marker: Marker,
    }
  }
}
