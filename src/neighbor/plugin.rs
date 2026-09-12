use crate::{
  core::Order,
  neighbor::{data::direction::Center, messages::Step, systems::step},
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
    app.add_message::<Step<Center>>().add_systems(
      self.schedule.clone(),
      step::<Center>.in_set(Order::Navigate),
    );
  }
}
