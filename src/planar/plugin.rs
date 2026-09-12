use crate::{
  core::Order,
  linear::data::direction::Right,
  neighbor::{messages::Step, systems::step},
  planar::data::direction::{Down, Up},
};
use bevy::{
  app::{App, Plugin as Super, PreUpdate},
  ecs::schedule::{IntoScheduleConfigs, ScheduleLabel},
};

#[derive(Debug)]
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
      .add_message::<Step<Down>>()
      .add_message::<Step<Up>>()
      .add_systems(
        self.schedule.clone(),
        (step::<Down>, step::<Up>)
          .chain()
          .after(step::<Right>)
          .in_set(Order::Navigate),
      );
  }
}
