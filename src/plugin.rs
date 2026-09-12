use crate::core::Plugin as CorePlugin;
use bevy::{
  app::{App, Plugin as Super, PreUpdate},
  ecs::schedule::ScheduleLabel,
};

#[cfg(feature = "neighbor")]
use crate::neighbor::Plugin as NeighborPlugin;

#[cfg(feature = "linear")]
use crate::linear::Plugin as LinearPlugin;

#[cfg(feature = "planar")]
use crate::planar::Plugin as PlanarPlugin;

#[cfg(feature = "spatial")]
use crate::spatial::Plugin as SpatialPlugin;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
    app.add_plugins(CorePlugin::new(self.schedule.clone()));

    #[cfg(feature = "neighbor")]
    app.add_plugins(NeighborPlugin::new(self.schedule.clone()));

    #[cfg(feature = "linear")]
    app.add_plugins(LinearPlugin::new(self.schedule.clone()));

    #[cfg(feature = "planar")]
    app.add_plugins(PlanarPlugin::new(self.schedule.clone()));

    #[cfg(feature = "spatial")]
    app.add_plugins(SpatialPlugin::new(self.schedule.clone()));
  }
}
