use bevy::ecs::schedule::SystemSet as Super;

#[derive(Debug, Super, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Order {
  Input,
  Navigate,
  Apply,
}
