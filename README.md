# bevy_mod_spatial_query

![Build Status](https://github.com/feilkin/bevy_mod_spatial_query/workflows/Rust/badge.svg)

Spatially aware queries for the [Bevy](http://bevyengine.org/) game engine

## Features

- Fast spatial lookup for queries
- Ergonomic interface: `SpatialQuery<Data, Filters>`, just like vanilla `Query`!
- Extendable: You can implement your own spatial lookup algorithms by implementing the `SpatialLookupAlgorithm` trait!

## Installation

`cargo add bevy_mod_spatial_query`

## Usage

````rust
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
````

## Contribution

Found a problem or have a suggestion? Feel free to open an issue.

## License

`bevy_mod_spatial_query` is licensed under the [MIT license](LICENSE).