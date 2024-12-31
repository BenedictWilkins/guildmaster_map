use rand::Rng;
use voronoice;

#[derive(Clone, Copy, Debug)]
pub enum Boundary {
    /// origin centered square boundary with the given `size` as its width and height.
    CenteredSquare(f32),
    /// origin centered rectangle boundary with the given `width` and `height`.
    CenteredRectangle(f32, f32),
}

impl Boundary {
    pub fn size(&self) -> (f32, f32) {
        match self {
            Boundary::CenteredSquare(size) => (*size, *size),
            Boundary::CenteredRectangle(width, height) => (*width, *height),
        }
    }

    pub fn top_left(&self) -> (f32, f32) {
        match self {
            Boundary::CenteredSquare(size) => (-size / 2.0, size / 2.0),
            Boundary::CenteredRectangle(width, height) => (-width / 2.0, height / 2.0),
        }
    }

    pub fn bottom_right(&self) -> (f32, f32) {
        match self {
            Boundary::CenteredSquare(size) => (size / 2.0, -size / 2.0),
            Boundary::CenteredRectangle(width, height) => (width / 2.0, -height / 2.0),
        }
    }
}

impl Default for Boundary {
    fn default() -> Self {
        Self::CenteredSquare(10.0)
    }
}

impl From<Boundary> for voronoice::BoundingBox {
    fn from(boundary: Boundary) -> Self {
        match boundary {
            Boundary::CenteredSquare(size) => {
                voronoice::BoundingBox::new_centered_square(size as f64)
            }
            Boundary::CenteredRectangle(width, height) => {
                voronoice::BoundingBox::new_centered(width as f64, height as f64)
            }
        }
    }
}

pub struct Point(voronoice::Point); // zero cost wrapper, thanks rust!
impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self(voronoice::Point { x, y })
    }
}
impl From<voronoice::Point> for Point {
    fn from(point: voronoice::Point) -> Self {
        Self(point)
    }
}
impl From<Point> for voronoice::Point {
    fn from(point: Point) -> Self {
        point.0
    }
}
impl From<(f32, f32)> for Point {
    fn from(point: (f32, f32)) -> Self {
        Self(voronoice::Point {
            x: point.0 as f64,
            y: point.1 as f64,
        })
    }
}
impl From<(f64, f64)> for Point {
    fn from(point: (f64, f64)) -> Self {
        Self(voronoice::Point {
            x: point.0,
            y: point.1,
        })
    }
}

pub struct VoronoiBuilder {
    inner: voronoice::VoronoiBuilder,
}

impl Default for VoronoiBuilder {
    fn default() -> Self {
        Self {
            inner: voronoice::VoronoiBuilder::default(),
        }
    }
}

impl VoronoiBuilder {
    pub fn set_boundary(mut self, boundary: Boundary) -> Self {
        self.inner = self.inner.set_bounding_box(boundary.into());
        self
    }

    pub fn set_lloyd_relaxation_iterations(mut self, iterations: usize) -> Self {
        self.inner = self.inner.set_lloyd_relaxation_iterations(iterations);
        self
    }

    pub fn set_sites_random(mut self, boundary: Boundary, count: usize) -> Self {
        let x_bounds = (boundary.top_left().0, boundary.bottom_right().0);
        // right handed coordinate system!
        let y_bounds = (boundary.bottom_right().1, boundary.top_left().1);
        let points = VoronoiBuilder::random_points(count, x_bounds, y_bounds);
        self = self.set_boundary(boundary);
        self.set_sites(points)
    }

    pub fn set_sites<T: Into<voronoice::Point>>(mut self, points: Vec<T>) -> Self {
        self.inner = self
            .inner
            .set_sites(points.into_iter().map(|p| p.into()).collect());
        self
    }

    pub fn build(self) -> Voronoi {
        Voronoi {
            voronoi: self.inner.build().unwrap(),
        }
    }

    fn random_points(count: usize, x_bounds: (f32, f32), y_bounds: (f32, f32)) -> Vec<Point> {
        let mut rng = rand::thread_rng();
        (0..count)
            .map(|_| {
                Point::new(
                    rng.gen_range(x_bounds.0 as f64..x_bounds.1 as f64),
                    rng.gen_range(y_bounds.0 as f64..y_bounds.1 as f64),
                )
            })
            .collect()
    }
}

pub struct Voronoi {
    voronoi: voronoice::Voronoi,
}

impl Default for Voronoi {
    fn default() -> Self {
        Self {
            voronoi: voronoice::VoronoiBuilder::default()
                .set_bounding_box(Boundary::default().into())
                .set_lloyd_relaxation_iterations(5)
                .build()
                .unwrap(),
        }
    }
}

impl Voronoi {
    pub fn random(boundary: Boundary, count: usize) {}
    pub fn new(points: Vec<(f32, f32)>, boundary: Boundary, relaxation: usize) -> Self {
        let voronoi = voronoice::VoronoiBuilder::default()
            .set_sites(
                points
                    .iter()
                    .map(|p| voronoice::Point {
                        x: p.0 as f64,
                        y: p.1 as f64,
                    })
                    .collect(),
            )
            .set_bounding_box(boundary.into())
            .set_lloyd_relaxation_iterations(relaxation)
            .build()
            .unwrap();
        Self { voronoi: voronoi }
    }

    pub fn inner(&self) -> &voronoice::Voronoi {
        &self.voronoi
    }

    pub fn bounding_box(&self) -> Vec<[f32; 2]> {
        let bbox = self.voronoi.bounding_box();
        let bbox_center = bbox.center();
        let (h, w) = (bbox.height(), bbox.width());
        vec![
            [
                (bbox_center.x - w / 2.0) as f32,
                (bbox_center.y - h / 2.0) as f32,
            ],
            [
                (bbox_center.x + w / 2.0) as f32,
                (bbox_center.y + h / 2.0) as f32,
            ],
            [
                (bbox_center.x + w / 2.0) as f32,
                (bbox_center.y - h / 2.0) as f32,
            ],
            [
                (bbox_center.x - w / 2.0) as f32,
                (bbox_center.y + h / 2.0) as f32,
            ],
        ]
    }

    /// Build the mesh buffers for the voronoi diagram by converting cells to triangles.
    /// Each cell is represented by a triangle fan with the final vertex of each triangle being the cell site (center), triangles are counter-clockwise order.
    /// You can use a flat vertex attribute with last triangle vertex to store values for each cell (just make sure to do this for all triangles in the cell!).
    /// The buffers are not optimized AT ALL, this might be a problem for large meshes, but its fine as a starting point. Cell triangles do however appear together in the buffers.
    pub fn mesh_buffers(&self) -> (Vec<[f32; 3]>, Vec<u32>) {
        let mut vertices = Vec::new();
        let mut indicies = Vec::new();
        //let mut cells = Vec::new();

        // here we will store the index of the vertex in the new `verticies` buffer, so they can be reused.
        let mut index_map: Vec<Option<usize>> = vec![None; self.voronoi.vertices().len()];
        let old_verticies = self.voronoi.vertices();
        // assuming that the sites and cells are in the same order... it appears so!

        self.voronoi.iter_cells().enumerate().for_each(|(i, cell)| {
            // add the verticies of the cell
            let center_index = vertices.len() as u32;
            let site = cell.site_position();
            vertices.push([site.x as f32, site.y as f32, 0.0]);

            // even though it says `iter_triangles` it is actually iterating over the circumcenters
            let circumcenters = cell.iter_triangles().collect::<Vec<usize>>();

            // the first iteration is just to add the verticies to the buffer, then we will compute the triangle indicies.
            circumcenters.iter().for_each(|i| {
                if index_map[*i].is_none() {
                    index_map[*i] = Some(vertices.len());
                    vertices.push([old_verticies[*i].x as f32, old_verticies[*i].y as f32, 0.0]);
                }
            });
            // compute the triangle indicies, fortunately the circumcenters are already in counter-clockwise order.
            ring(circumcenters.as_slice()).for_each(|(i, j)| {
                indicies.push(index_map[*i].unwrap() as u32);
                indicies.push(index_map[*j].unwrap() as u32);
                indicies.push(center_index); // add the center vertex
            });
        });
        (vertices, indicies)
    }
}

fn ring<T>(vec: &[T]) -> impl Iterator<Item = (&T, &T)> {
    vec.iter().zip(vec.iter().cycle().skip(1)).take(vec.len())
}

pub fn random_points(count: usize, bounds: (f32, f32)) -> Vec<(f32, f32)> {
    let mut rng = rand::thread_rng();
    (0..count)
        .map(|_| {
            (
                rng.gen_range(bounds.0..bounds.1),
                rng.gen_range(bounds.0..bounds.1),
            )
        })
        .collect()
}
