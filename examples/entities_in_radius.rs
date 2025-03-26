use bevy::prelude::*;
use bevy_mod_spatial_queries::prelude::*;

fn main() {
    let mut app = App::new();

    let mut app = app
        .add_plugins(DefaultPlugins)
        .add_plugins(SpatialQueriesPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, change_color_on_hover);

    app.run();
}

#[derive(Resource)]
struct ExampleMaterials {
    default_material: Handle<ColorMaterial>,
    hovered_material: Handle<ColorMaterial>,
}

#[derive(Component)]
struct CircleMarker;

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
