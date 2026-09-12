use crate::{
  core::Order,
  neighbor::{messages::Step, systems::step},
  planar::data::direction::Up,
  spatial::data::direction::{Back, Forward},
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
      .add_message::<Step<Back>>()
      .add_message::<Step<Forward>>()
      .add_systems(
        self.schedule.clone(),
        (step::<Back>, step::<Forward>)
          .chain()
          .after(step::<Up>)
          .in_set(Order::Navigate),
      );
  }
}
