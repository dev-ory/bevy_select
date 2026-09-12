use crate::neighbor::traits::Direction;
use bevy::ecs::{entity::Entity, message::Message};
use std::marker::PhantomData as Marker;

#[derive(Debug, Message)]
pub struct Step<A: Direction> {
  pub selector: Entity,
  marker: Marker<A>,
}

impl<A: Direction> Step<A> {
  pub fn new(selector: Entity) -> Self {
    Self {
      selector,
      marker: Marker,
    }
  }
}
