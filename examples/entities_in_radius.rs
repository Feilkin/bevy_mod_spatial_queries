//! Example which shows how to use spatial queries to find entities in a radius.

use bevy::prelude::*;
use bevy_mod_spatial_queries::prelude::*;

/// Number of rows of circles to spawn.
const ROWS: usize = 72;
/// Number of columns of circles to spawn.
const COLUMNS: usize = 128;
/// Radius of spawned circles.
const CIRCLE_RADIUS: f32 = 5.0;
/// Radius used when looking up nearby circles.
const LOOKUP_RADIUS: f32 = 10.0;

fn main() {
    let mut app = App::new();

    let mut app = app
        .add_plugins(DefaultPlugins)
        .add_plugins(SpatialQueriesPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, change_color_on_hover);

    app.run();
}

/// Resource for storing the used materials.
///
/// This allows us to easily re-used to existing materials when swapping between
/// hovered and default states. Re-using the materials also allows bevy to batch
/// the circle draw calls, significantly improving rendering performance.
#[derive(Resource)]
struct ExampleMaterials {
    /// Material for the default, non-hovered state.
    default_material: Handle<ColorMaterial>,
    /// Material for the hovered state.
    hovered_material: Handle<ColorMaterial>,
}

/// Component used to mark the entities we want to find with our spatial query.
#[derive(Component)]
struct CircleMarker;

/// System used to set up necessary entities and resources at application startup.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let default_material = materials.add(Color::linear_rgb(0.1, 0.1, 0.1));
    let hovered_material = materials.add(Color::linear_rgb(0.8, 0.8, 0.8));

    let mesh = meshes.add(Circle::new(10.));

    for row in 0..72 {
        for col in 0..128 {
            commands.spawn((
                Mesh2d(mesh.clone()),
                MeshMaterial2d(default_material.clone()),
                Transform::from_translation(Vec3::new(
                    col as f32 * 10. - 1280. / 2.,
                    row as f32 * 10. - 720. / 2.,
                    0.0,
                )),
                CircleMarker,
            ));
        }
    }

    commands.spawn(Camera2d);
    commands.insert_resource(ExampleMaterials {
        default_material,
        hovered_material,
    });
}

/// System which changes the material of entities that are near the cursor using spatial queries.
fn change_color_on_hover(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    window: Query<&Window>,
    mut circles: SpatialQuery<&mut MeshMaterial2d<ColorMaterial>, With<CircleMarker>>,
    materials: Res<ExampleMaterials>,
) {
    let (camera, camera_transform) = *camera_query;
    let Ok(window) = window.get_single() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    for mut circle_material in circles.in_radius(world_position.extend(0.), 20.) {
        circle_material.0 = materials.hovered_material.clone();
    }
}
