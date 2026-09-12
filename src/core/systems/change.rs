use crate::core::{components::Selector, events::Changed, messages::Change};
use bevy::ecs::{
  entity::Entity,
  message::MessageReader,
  system::{Commands, Query},
};

pub fn change(
  mut commands: Commands,
  mut changes: MessageReader<Change>,
  mut selectors: Query<(Entity, &mut Selector)>,
) {
  for change in changes.read() {
    if let Ok((entity, mut selector)) = selectors.get_mut(change.selector) {
      let old = selector.target;
      let new = change.target;

      if old != new {
        selector.target = change.target;
        commands.trigger(Changed {
          selector: entity,
          old,
          new,
        });
      }
    }
  }
}
