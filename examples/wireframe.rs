// //! This example demonstrates Bevy's immediate mode drawing API intended for visual debugging.

// use std::f32::consts::{FRAC_PI_2, PI, TAU};

// use bevy::{color::palettes::css::*, math::Isometry2d, prelude::*};

// use bevy_mapgen::voronoi::*;
// use delaunator::{Point, Triangulation};

// fn main() {
//     let boundary = 200.0;
//     let points = random_points(4, (-boundary, boundary));
//     let voronoi = Voronoi::new(points)
//         .with_boundary(Boundary::CenteredSquare(boundary * 2.0))
//         .lloyd_relaxation(20);
//     println!("{:?}", voronoi.cells());

//     App::new()
//         .add_plugins(DefaultPlugins)
//         .insert_resource(VoronoiResource(voronoi))
//         .add_systems(Startup, setup)
//         .add_systems(Update, (draw_example_collection,))
//         .run();
// }

// #[derive(Resource)]
// struct VoronoiResource(Voronoi);

// impl VoronoiResource {
//     fn voronoi(&self) -> &Voronoi {
//         &self.0
//     }
// }

// fn setup(mut commands: Commands) {
//     commands.spawn(Camera2d);
//     // text
//     commands.spawn((
//         Text::new(
//             "Hold 'Left' or 'Right' to change the line width of straight gizmos\n\
//         Hold 'Up' or 'Down' to change the line width of round gizmos\n\
//         Press '1' / '2' to toggle the visibility of straight / round gizmos\n\
//         Press 'U' / 'I' to cycle through line styles\n\
//         Press 'J' / 'K' to cycle through line joins",
//         ),
//         Node {
//             position_type: PositionType::Absolute,
//             top: Val::Px(12.),
//             left: Val::Px(12.),
//             ..default()
//         },
//     ));
// }

// fn as_vec2(point: (f32, f32)) -> Vec2 {
//     Vec2::new(point.0, point.1)
// }

// fn ring<T>(vec: &[T]) -> impl Iterator<Item = (&T, &T)> {
//     vec.iter().zip(vec.iter().cycle().skip(1)).take(vec.len())
// }

// fn draw_example_collection(mut gizmos: Gizmos, voronoi: Res<VoronoiResource>, time: Res<Time>) {
//     // draw boundary
//     for (start, end) in ring(&voronoi.0.boundary().bounds()) {
//         gizmos.line_2d(Vec2::new(start.0, start.1), Vec2::new(end.0, end.1), BLUE);
//     }

//     for point in voronoi.0.sites() {
//         gizmos.circle_2d(Vec2::new(point.0, point.1), 4., BLUE);
//     }

//     for point in voronoi.0.circumcenters() {
//         gizmos.circle_2d(Vec2::new(point.0, point.1), 2., RED);
//     }

//     for cell_verts in voronoi.0.cells() {
//         for (start, end) in ring(&cell_verts) {
//             gizmos.line_2d(Vec2::new(start.0, start.1), Vec2::new(end.0, end.1), WHITE);
//         }
//     }

//     // let sin_t_scaled = ops::sin(time.elapsed_secs()) * 50.;
//     // gizmos.line_2d(Vec2::Y * -sin_t_scaled, Vec2::splat(-80.), RED);
//     // gizmos.ray_2d(Vec2::Y * sin_t_scaled, Vec2::splat(80.), LIME);

//     // gizmos
//     //     .grid_2d(
//     //         Isometry2d::IDENTITY,
//     //         UVec2::new(16, 9),
//     //         Vec2::new(80., 80.),
//     //         // Dark gray
//     //         LinearRgba::gray(0.05),
//     //     )
//     //     .outer_edges();

//     // // Triangle
//     // gizmos.linestrip_gradient_2d([
//     //     (Vec2::Y * 300., BLUE),
//     //     (Vec2::new(-255., -155.), RED),
//     //     (Vec2::new(255., -155.), LIME),
//     //     (Vec2::Y * 300., BLUE),
//     // ]);

//     // gizmos.rect_2d(Isometry2d::IDENTITY, Vec2::splat(650.), BLACK);

//     // gizmos.cross_2d(Vec2::new(-160., 120.), 12., FUCHSIA);

//     // let domain = Interval::EVERYWHERE;
//     // let curve = FunctionCurve::new(domain, |t| Vec2::new(t, ops::sin(t / 25.0) * 100.0));
//     // let resolution = ((ops::sin(time.elapsed_secs()) + 1.0) * 50.0) as usize;
//     // let times_and_colors = (0..=resolution)
//     //     .map(|n| n as f32 / resolution as f32)
//     //     .map(|t| (t - 0.5) * 600.0)
//     //     .map(|t| (t, TEAL.mix(&HOT_PINK, (t + 300.0) / 600.0)));
//     // gizmos.curve_gradient_2d(curve, times_and_colors);

//     // my_gizmos
//     //     .rounded_rect_2d(Isometry2d::IDENTITY, Vec2::splat(630.), BLACK)
//     //     .corner_radius(ops::cos(time.elapsed_secs() / 3.) * 100.);

//     // // Circles have 32 line-segments by default.
//     // // You may want to increase this for larger circles.
//     // my_gizmos
//     //     .circle_2d(Isometry2d::IDENTITY, 300., NAVY)
//     //     .resolution(64);

//     // my_gizmos.ellipse_2d(
//     //     Rot2::radians(time.elapsed_secs() % TAU),
//     //     Vec2::new(100., 200.),
//     //     YELLOW_GREEN,
//     // );

//     // // Arcs default resolution is linearly interpolated between
//     // // 1 and 32, using the arc length as scalar.
//     // my_gizmos.arc_2d(
//     //     Rot2::radians(sin_t_scaled / 10.),
//     //     FRAC_PI_2,
//     //     310.,
//     //     ORANGE_RED,
//     // );
//     // my_gizmos.arc_2d(Isometry2d::IDENTITY, FRAC_PI_2, 80.0, ORANGE_RED);
//     // my_gizmos.long_arc_2d_between(Vec2::ZERO, Vec2::X * 20.0, Vec2::Y * 20.0, ORANGE_RED);
//     // my_gizmos.short_arc_2d_between(Vec2::ZERO, Vec2::X * 40.0, Vec2::Y * 40.0, ORANGE_RED);

//     // gizmos.arrow_2d(
//     //     Vec2::ZERO,
//     //     Vec2::from_angle(sin_t_scaled / -10. + PI / 2.) * 50.,
//     //     YELLOW,
//     // );

//     // // You can create more complex arrows using the arrow builder.
//     // gizmos
//     //     .arrow_2d(
//     //         Vec2::ZERO,
//     //         Vec2::from_angle(sin_t_scaled / -10.) * 50.,
//     //         GREEN,
//     //     )
//     //     .with_double_end()
//     //     .with_tip_length(10.);
// }

// // fn update_config(
// //     mut config_store: ResMut<GizmoConfigStore>,
// //     keyboard: Res<ButtonInput<KeyCode>>,
// //     time: Res<Time>,
// // ) {
// //     let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
// //     if keyboard.pressed(KeyCode::ArrowRight) {
// //         config.line_width += 5. * time.delta_secs();
// //         config.line_width = config.line_width.clamp(0., 50.);
// //     }
// //     if keyboard.pressed(KeyCode::ArrowLeft) {
// //         config.line_width -= 5. * time.delta_secs();
// //         config.line_width = config.line_width.clamp(0., 50.);
// //     }
// //     if keyboard.just_pressed(KeyCode::Digit1) {
// //         config.enabled ^= true;
// //     }
// //     if keyboard.just_pressed(KeyCode::KeyU) {
// //         config.line_style = match config.line_style {
// //             GizmoLineStyle::Solid => GizmoLineStyle::Dotted,
// //             _ => GizmoLineStyle::Solid,
// //         };
// //     }
// //     if keyboard.just_pressed(KeyCode::KeyJ) {
// //         config.line_joints = match config.line_joints {
// //             GizmoLineJoint::Bevel => GizmoLineJoint::Miter,
// //             GizmoLineJoint::Miter => GizmoLineJoint::Round(4),
// //             GizmoLineJoint::Round(_) => GizmoLineJoint::None,
// //             GizmoLineJoint::None => GizmoLineJoint::Bevel,
// //         };
// //     }

// //     let (my_config, _) = config_store.config_mut::<MyRoundGizmos>();
// //     if keyboard.pressed(KeyCode::ArrowUp) {
// //         my_config.line_width += 5. * time.delta_secs();
// //         my_config.line_width = my_config.line_width.clamp(0., 50.);
// //     }
// //     if keyboard.pressed(KeyCode::ArrowDown) {
// //         my_config.line_width -= 5. * time.delta_secs();
// //         my_config.line_width = my_config.line_width.clamp(0., 50.);
// //     }
// //     if keyboard.just_pressed(KeyCode::Digit2) {
// //         my_config.enabled ^= true;
// //     }
// //     if keyboard.just_pressed(KeyCode::KeyI) {
// //         my_config.line_style = match my_config.line_style {
// //             GizmoLineStyle::Solid => GizmoLineStyle::Dotted,
// //             _ => GizmoLineStyle::Solid,
// //         };
// //     }
// //     if keyboard.just_pressed(KeyCode::KeyK) {
// //         my_config.line_joints = match my_config.line_joints {
// //             GizmoLineJoint::Bevel => GizmoLineJoint::Miter,
// //             GizmoLineJoint::Miter => GizmoLineJoint::Round(4),
// //             GizmoLineJoint::Round(_) => GizmoLineJoint::None,
// //             GizmoLineJoint::None => GizmoLineJoint::Bevel,
// //         };
// //     }
// // }

fn main() {}
