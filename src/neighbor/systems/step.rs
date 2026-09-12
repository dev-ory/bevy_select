use crate::{
  core::{components::Selector, messages::Change},
  neighbor::{
    components::{Fallback, Neighbor},
    events::Stepped,
    messages::Step,
    traits::Direction,
  },
};
use bevy::ecs::{
  entity::Entity,
  message::{MessageReader, MessageWriter},
  system::{Commands, Query},
};

pub fn step<A: Direction>(
  mut commands: Commands,
  mut steps: MessageReader<Step<A>>,
  mut changes: MessageWriter<Change>,
  selectors: Query<(Entity, &Selector)>,
  neighbors: Query<&Neighbor<A>>,
  fallbacks: Query<&Fallback<A>>,
) {
  for step in steps.read() {
    if let Ok((entity, selector)) = selectors.get(step.selector) {
      let target = if let Some(old) = selector.target
        && let Ok(neighbor) = neighbors.get(old)
      {
        neighbor.entity
      } else if let Ok(fallback) = fallbacks.get(entity) {
        fallback.entity
      } else {
        continue;
      };

      changes.write(Change {
        selector: entity,
        target: Some(target),
      });
      commands.trigger(Stepped::<A>::new(entity));
    }
  }
}
