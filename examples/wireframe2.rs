//! This example demonstrates Bevy's immediate mode drawing API intended for visual debugging.

use bevy::{
    color::palettes::{css::*, tailwind::CYAN_100},
    prelude::*,
};
use guildmaster_map::voronoi::*;

fn main() {
    let voronoi = VoronoiBuilder::default()
        .set_sites_random(Boundary::CenteredSquare(500.0), 20)
        .set_lloyd_relaxation_iterations(10)
        .build();

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(VoronoiResource::new(voronoi))
        .init_gizmo_group::<MeshGizmos>()
        .add_systems(Startup, setup)
        .add_systems(Update, (draw_example_collection, display_config))
        .run();
}

#[derive(Default, Reflect, GizmoConfigGroup)]
struct MeshGizmos {}

#[derive(Resource)]
struct VoronoiResource {
    voronoi: Voronoi,
    // display options
    pub show_cells: bool,
    pub show_mesh: bool,
    pub show_sites: bool,
    pub show_circumcenters: bool,
}

impl VoronoiResource {
    fn new(voronoi: Voronoi) -> Self {
        Self {
            voronoi,
            show_cells: true,
            show_mesh: false,
            show_sites: true,
            show_circumcenters: true,
        }
    }
    fn voronoi(&self) -> &Voronoi {
        &self.voronoi
    }
}

fn setup(mut commands: Commands, mut gizmo_group: ResMut<GizmoConfigStore>) {
    let (gizmo_config, _) = gizmo_group.config_mut::<MeshGizmos>();
    //gizmo_config.line_style = GizmoLineStyle::Dotted;
    //gizmo_config.line_width = 1.0;
    //gizmo_config.line_perspective = true;

    commands.spawn(Camera2d);
    // text
    commands.spawn((
        Text::new(
            "Press `1` to hide/show voronoi cells\n\
            Press `2` to hide/show voronoi mesh\n\
            Press `3` to hide/show voronoi sites\n\
            Press `4` to hide/show voronoi circumcenters\n",
        ),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.),
            left: Val::Px(12.),
            ..default()
        },
    ));
}

fn as_vec2(point: (f32, f32)) -> Vec2 {
    Vec2::new(point.0, point.1)
}

fn ring<T>(vec: &[T]) -> impl Iterator<Item = (&T, &T)> {
    vec.iter().zip(vec.iter().cycle().skip(1)).take(vec.len())
}

fn draw_example_collection(
    mut gizmos: Gizmos,
    mut mesh_gizmos: Gizmos<MeshGizmos>,
    voronoi: Res<VoronoiResource>,
) {
    if voronoi.show_sites {
        voronoi.voronoi().inner().sites().iter().for_each(|site| {
            gizmos.circle_2d(as_vec2((site.x as f32, site.y as f32)), 5.0, RED);
        });
    }

    if voronoi.show_circumcenters {
        voronoi
            .voronoi()
            .inner()
            .vertices()
            .iter()
            .for_each(|vertex| {
                gizmos.circle_2d(as_vec2((vertex.x as f32, vertex.y as f32)), 5.0, CYAN_100);
            });
    }
    if voronoi.show_cells {
        draw_cells(&mut gizmos, voronoi.voronoi());
    }
    if voronoi.show_mesh {
        draw_mesh(&mut mesh_gizmos, voronoi.voronoi());
    }
}

fn display_config(mut voronoi: ResMut<VoronoiResource>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Digit1) {
        voronoi.show_cells ^= true;
    }
    if keyboard.just_pressed(KeyCode::Digit2) {
        voronoi.show_mesh ^= true;
    }
    if keyboard.just_pressed(KeyCode::Digit3) {
        voronoi.show_sites ^= true;
    }
    if keyboard.just_pressed(KeyCode::Digit4) {
        voronoi.show_circumcenters ^= true;
    }
}

fn draw_cells(gizmos: &mut Gizmos, voronoi: &Voronoi) {
    let vertices = voronoi.inner().vertices();

    voronoi.inner().cells().iter().for_each(|cell| {
        ring(cell).for_each(|(a, b)| {
            let a = &vertices[*a];
            let b = &vertices[*b];
            gizmos.line_2d(
                as_vec2((a.x as f32, a.y as f32)),
                as_vec2((b.x as f32, b.y as f32)),
                BLUE,
            )
        });
    });
}
fn draw_mesh(gizmos: &mut Gizmos<MeshGizmos>, voronoi: &Voronoi) {
    let (vertices, indicies) = voronoi.mesh_buffers();

    // lets debug the mesh buffers!
    for i in indicies.chunks(3) {
        // draw each triangle
        let a = Vec2::new(vertices[i[0] as usize][0], vertices[i[0] as usize][1]);
        let b = Vec2::new(vertices[i[1] as usize][0], vertices[i[1] as usize][1]);
        let c = Vec2::new(vertices[i[2] as usize][0], vertices[i[2] as usize][1]);
        gizmos.arrow_2d(a, b, WHITE);
        gizmos.arrow_2d(b, c, WHITE);
        gizmos.arrow_2d(c, a, WHITE);
    }
}
