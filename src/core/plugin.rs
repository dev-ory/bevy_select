use crate::core::{Order, messages::Change, systems::change};
use bevy::{
  app::{App, Plugin as Super, PreUpdate},
  ecs::schedule::{IntoScheduleConfigs, ScheduleLabel},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Plugin<A: ScheduleLabel + Clone = PreUpdate> {
  pub schedule: A,
}

impl<A: ScheduleLabel + Clone> Plugin<A> {
  pub fn new(schedule: A) -> Self {
    Self { schedule }
  }
}

impl Default for Plugin<PreUpdate> {
  fn default() -> Self {
    Self::new(PreUpdate)
  }
}

impl<A: ScheduleLabel + Clone> Super for Plugin<A> {
  fn build(&self, app: &mut App) {
    app
      .configure_sets(
        self.schedule.clone(),
        (Order::Input, Order::Navigate, Order::Apply).chain(),
      )
      .add_message::<Change>()
      .add_systems(self.schedule.clone(), change.in_set(Order::Apply));
  }
}
