use bevy::{asset::RenderAssetUsages, prelude::*};
use voronoice::*;
mod voronoi;
fn main() {}

// fn main() {
//     App::new()
//         .add_plugins((
//             DefaultPlugins,
//             MaterialPlugin::<voronoi::VoronoiMaterial>::default(),
//         ))
//         .add_systems(Startup, setup)
//         .run();
// }

// fn setup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<voronoi::VoronoiMaterial>>,
// ) {
//     let sites = vec![(0.5, 0.5), (1.0, 0.5), (0.5, 1.0)];
//     let voronoi = voronoi::voronoi(sites, 1, (10.0, 10.0));
//     let mesh = meshes.add(voronoi::voronoi_mesh(
//         voronoi,
//         RenderAssetUsages::RENDER_WORLD,
//     ));
// let material = materials.add(voronoi::VoronoiMaterial {});
// commands.spawn((
//     Mesh3d(mesh),
//     MeshMaterial3d(material),
//     Transform::from_xyz(0.0, 0.0, 0.0),
// ));

// // camera
// commands.spawn((
//     Camera3d::default(),
//     Transform::from_xyz(0.0, 0.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
// ));
//}
