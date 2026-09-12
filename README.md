# bevy_select

`bevy_select` is a small, composable selection and directional-navigation library for Bevy 0.19.
It keeps selection state in a `Selector`, represents navigation requests as buffered Bevy messages,
and publishes observer events when selection changes or a directional step is taken.

<<<<<<< HEAD
Currently, it is quite 'opinionated' in the way I organize, import, and name things, but I do plan on eventually including a prelude-like module. However, this is just a personal-use crate, so don't expect it to be updated or maintained.
=======
This is just a personal-use crate, so don't expect it to be updated or maintained, unless I need it.
>>>>>>> 2b6b12d (added prelude)

## Features

Features build on one another. The default feature set is deliberately empty, so applications can
choose the smallest navigation model they need.

| Feature    | Adds                                                 | Directions                                                 |
| ---------- | ---------------------------------------------------- | ---------------------------------------------------------- |
| `neighbor` | Generic directional navigation and a center fallback | `Center`                                                   |
| `linear`   | One-dimensional navigation                           | `Left`, `Right`, `Center`                                  |
| `planar`   | Two-dimensional navigation                           | `Left`, `Right`, `Up`, `Down`, `Center`                    |
| `spatial`  | Three-dimensional navigation                         | `Left`, `Right`, `Up`, `Down`, `Forward`, `Back`, `Center` |

For example, enable 2D navigation:

```toml
[dependencies]
bevy_select = { version = "0.0.1", features = ["planar"] }
```

## Quick start

Add the root plugin, create selectable entities, connect them with `Neighbor` components, and send
a `Step` message when input asks to move:

```rust
use bevy::prelude::*;
use bevy_select::{
    Plugin as SelectPlugin,
    core::components::Selector,
    linear::data::direction::Right,
    neighbor::{components::Neighbor, messages::Step},
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SelectPlugin::default()))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    let right = commands.spawn_empty().id();
    let left = commands.spawn(Neighbor::<Right>::new(right)).id();
    let selector = commands.spawn(Selector { target: Some(left) }).id();

    commands.write_message(Step::<Right>::new(selector));
}
```

The next navigation pass selects `right`.

## How navigation works

The plugin configures three public system sets in order:

```text
Order::Input -> Order::Navigate -> Order::Apply
```

Applications can put input systems in `Order::Input`. Directional systems consume `Step<D>` in
`Order::Navigate` and write `Change` messages. The core system consumes `Change` in
`Order::Apply`, updates `Selector::target`, and triggers `Changed` only when the target actually
changes.

`Fallback<D>` can be placed on a selector to provide a target when its current selection has no
`Neighbor<D>`.

## Events

- `Changed` is an `EntityEvent` targeted at the selector. It contains the old and new targets.
- `Stepped<D>` is an `EntityEvent` targeted at the selector after a valid directional step is
  resolved.

Use Bevy observers to react to either event, for example to update visuals or play sound.

## Custom schedules

`Plugin::default()` uses Bevy's `PreUpdate` schedule. To use another schedule, pass its label:

```rust
app.add_plugins(SelectPlugin::new(MyNavigationSchedule));
```

The host application is responsible for adding and running custom schedules.

## License and repository

This package intentionally does not declare a license or repository URL yet. Set those fields in
`Cargo.toml` before publishing so downstream users know their rights and can find the source.
