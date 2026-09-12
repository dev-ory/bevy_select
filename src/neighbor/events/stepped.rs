use crate::neighbor::traits::Direction;
use bevy::ecs::{entity::Entity, event::EntityEvent};
use std::marker::PhantomData as Marker;

#[derive(Debug, EntityEvent)]
pub struct Stepped<A: Direction> {
  #[event_target]
  pub selector: Entity,
  marker: Marker<A>,
}

impl<A: Direction> Stepped<A> {
  pub fn new(selector: Entity) -> Self {
    Self {
      selector,
      marker: Marker,
    }
  }
}
