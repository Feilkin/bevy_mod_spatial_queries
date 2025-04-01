# bevy_mod_spatial_query

[![Build Status](https://github.com/feilkin/bevy_mod_spatial_query/workflows/Rust/badge.svg)](https://github.com/Feilkin/bevy_mod_spatial_query/actions)
[![Crates.io Version](https://img.shields.io/crates/v/bevy_mod_spatial_query)](https://crates.io/crates/bevy_mod_spatial_query)
[![docs.rs](https://img.shields.io/docsrs/bevy_mod_spatial_query)](https://docs.rs/bevy_mod_spatial_query/latest/bevy_mod_spatial_query/index.html)
[![Static Badge](https://img.shields.io/badge/License-MIT-blue)](https://github.com/Feilkin/bevy_mod_spatial_query/blob/master/LICENSE)

Spatially aware queries for the [Bevy](http://bevyengine.org/) game engine

## Features

- Fast spatial lookup for queries
- Ergonomic interface: `SpatialQuery<Data, Filters>`, just like vanilla `Query`!
- Extendable: You can implement your own spatial lookup algorithms by implementing the `SpatialLookupAlgorithm` trait!

## Installation

`cargo add bevy_mod_spatial_query`

## Usage

```rust
use bevy::prelude::*;
use bevy_mod_spatial_query::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins(SpatialQueryPlugin)
        .add_systems(Update, your_awesome_system);

    app.run();
}

#[derive(Component)]
struct Player;

fn your_awesome_system(
    player: Single<&Transform, With<Player>>,
    nearby_lights: SpatialQuery<&mut PointLight>
) {
    for light in nearby_lights.in_radius(player.translation, 10.) {
        // Do something with the lights...
    }
}
```

### Choosing a lookup algorithm

By default, the crate uses a naive lookup algorithm, which simply iterates over all entities in the world and returns
those matching the spatial query. This is actually the fastest way to do spatial queries for most use cases, and more
advanced algorithms are only beneficial for cases where you need many (1000+) queries per frame, for example if
implementing an SPH fluid simulation using entities. For these rare cases a BVH-based algorithm is provided.

You are also free to implement your own lookup algorithms via the `SpatialLookupAlgorithm` trait.

To set the used algorithm, add the plugin like so:

```rust
fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .insert_resource(SpatialLookupState::with_algorithm(Bvh::default()))
        .add_plugins(SpatialQueryPlugin);

    app.run();
}
```

## Contribution

Found a problem or have a suggestion? Feel free to open an issue.

## License

`bevy_mod_spatial_query` is licensed under the [MIT license](LICENSE).