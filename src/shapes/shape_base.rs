use std::f32::consts::PI;

use glam::{Vec3, Vec4};

use super::{ColorType, IndicesType, InitType, RenderShape, RenderType, ShapeBase};

impl RenderShape for ShapeBase {
    fn shape_type(&self) -> super::ShapeType {
        self.shape
    }

    fn should_repaint(&self) -> bool {
        self.modified
    }

    fn get_render_type(&self) -> RenderType {
        self.render_type
    }

    fn set_index(&mut self, index: usize) {
        self.id = index;
    }

    fn get_index(&self) -> usize {
        self.id
    }

    fn points(&self) -> &[crate::buffers::vertex_buffer::PointBuffer] {
        bytemuck::cast_slice(&self.points)
    }

    fn colors(&self) -> &[crate::buffers::vertex_buffer::ColorBuffer] {
        bytemuck::cast_slice(&self.colors)
    }

    fn indices(&self) -> &[u32] {
        &self.indices
    }

    fn transform(&self) -> [[f32; 4]; 4] {
        self.transform.to_cols_array_2d()
    }

    fn parent(&self) -> Option<usize> {
        self.parent
    }
}

impl<'i, T> InitType<'i, T> {
    pub fn len(&self) -> usize {
        match self {
            InitType::Ref(r) => r.len(),
            InitType::Move(vec) => vec.len(),
        }
    }
}

impl<'i, T> From<&'i [T]> for InitType<'i, T> {
    fn from(value: &'i [T]) -> Self {
        Self::Ref(value)
    }
}
impl<'i, T> From<Vec<T>> for InitType<'i, T> {
    fn from(value: Vec<T>) -> Self {
        Self::Move(value)
    }
}
impl<'i> ColorType<'i> {
    pub fn new_uniform(c: Vec4) -> Self {
        Self::Uniform(c)
    }
    pub fn new_each_ref(c: &'i [Vec4]) -> Self {
        Self::Each(InitType::Ref(c))
    }
    pub fn new_each(c: Vec<Vec4>) -> Self {
        Self::Each(InitType::Move(c))
    }
}
impl<'i> IndicesType<'i> {
    pub fn new_all() -> Self {
        Self::Sequence
    }
    pub fn new_ref(ids: &'i [u32]) -> Self {
        Self::Partial(InitType::Ref(ids))
    }
    pub fn new(ids: Vec<u32>) -> Self {
        Self::Partial(InitType::Move(ids))
    }
}

impl ShapeBase {
    pub fn new_raw(
        points: InitType<Vec3>,
        colors: ColorType,
        indices: IndicesType,
        render_type: RenderType,
    ) -> Self {
        let n_points = points.len();
        Self {
            modified: true,
            points: match points {
                InitType::Ref(ps) => ps.to_vec(),
                InitType::Move(vec) => vec,
            },
            colors: match colors {
                ColorType::Each(init_type) => match init_type {
                    InitType::Ref(c) => c.to_vec(),
                    InitType::Move(vec) => vec,
                },
                ColorType::Uniform(vec4) => vec![vec4; n_points],
            },
            indices: match indices {
                IndicesType::Sequence => (0..n_points as u32).collect(),
                IndicesType::Partial(init_type) => match init_type {
                    InitType::Ref(ids) => ids.to_vec(),
                    InitType::Move(vec) => vec,
                },
            },
            render_type,
            ..Default::default()
        }
    }
    pub fn new_square(
        size: f32,
        color: Vec4,
        wire: bool,
    ) -> Self {
        let colors = ColorType::new_uniform(color);
        let indices = IndicesType::new(
            if wire {
                vec![0, 1, 2, 3, 0]
            } else {
                vec![0, 1, 2, 2, 3, 0]
            }
        );
        let half = size / 2.;
        let points = InitType::Move(
            vec![
                Vec3::new(-half, -half, 0.),
                Vec3::new(half, -half, 0.),
                Vec3::new(half, half, 0.),
                Vec3::new(-half, half, 0.),
            ]
        );
        Self::new_raw(points, colors, indices,
            if wire {RenderType::LineStrip} else  {RenderType::Triangle})
    }
    /// Face Order: X, -X, Y, -Y, Z, -Z
    pub fn new_cube(
        size: f32,
        face_color: [Vec4; 6],
        wire: bool,
    ) -> Self {
        let half = size / 2.;
        let ids = if wire {
                vec![0, 1, 1, 2, 2, 3, 3, 0]
            } else {
                vec![0, 1, 2, 2, 3, 0]
            };
        let indices = IndicesType::Partial(
            InitType::Move(
                (0..6).flat_map(|f| {
                    ids.iter().map(|&i| i + f * 4).collect::<Vec<_>>()
                }).collect::<Vec<_>>()
            )
        );
        let points = InitType::Move(
            vec![
                // X
                Vec3::new(half, -half, -half),
                Vec3::new(half, half, -half),
                Vec3::new(half, half, half),
                Vec3::new(half, -half, half),
                // -X
                Vec3::new(-half, half, -half),
                Vec3::new(-half, -half, -half),
                Vec3::new(-half, -half, half),
                Vec3::new(-half, half, half),
                // Y
                Vec3::new(half, half, -half),
                Vec3::new(-half, half, -half),
                Vec3::new(-half, half, half),
                Vec3::new(half, half, half),
                // -Y
                Vec3::new(-half, -half, -half),
                Vec3::new(half, -half, -half),
                Vec3::new(half, -half, half),
                Vec3::new(-half, -half, half),
                // Z
                Vec3::new(-half, -half, half),
                Vec3::new(half, -half, half),
                Vec3::new(half, half, half),
                Vec3::new(-half, half, half),
                // -Z
                Vec3::new(-half, -half, -half),
                Vec3::new(half, -half, -half),
                Vec3::new(half, half, -half),
                Vec3::new(-half, half, -half),
            ]
        );
        let colors = ColorType::new_each(
            vec![
                face_color[0], face_color[0],
                face_color[1], face_color[1],
                face_color[2], face_color[2],
            ]
        );
        Self::new_raw(points, colors, indices, 
            if wire {RenderType::Line} else {RenderType::Triangle} )
            
    }
    pub fn new_circle(
        r: f32,
        sub: u32,
        color: Vec4,
        wire: bool,
    ) -> Self {
        let sub = sub.max(3);
        let d = 2. * PI / sub as f32;
        let (points, indices) = (0..=sub)
            .fold((vec![], vec![]), |(mut ps, mut is), i| {
                if i==0 {
                    ps.push(Vec3::ZERO);
                    return (ps, is);
                }
                let rad = d * (i - 1) as f32;
                ps.push(Vec3::new(rad.cos() * r, rad.sin() * r, 0.));
                if wire {
                    if i==sub {
                        is.extend(vec![i , 1]);
                    } else {
                        is.extend(vec![i , i+1]);
                    }
                } else {
                    if i == sub {
                        is.extend(vec![0, i, 1]);
                    } else {
                        is.extend(vec![0, i, i+1]);
                    }
                }
                (ps, is)
            });
        Self::new_raw(InitType::Move(points),
            ColorType::Uniform(color),
            IndicesType::Partial(InitType::Move(indices)),
            if wire {RenderType::LineStrip} else {RenderType::Triangle})
    }
    pub fn new_sphere(
        r: f32,
        u_sub: u32,
        v_sub: u32,
        color: Vec4,
        wire: bool
    ) -> Self {
        let u_sub = u_sub.max(3);
        let v_sub = v_sub.max(3);
        let v_sub = if v_sub % 2 == 0 { v_sub + 1 } else { v_sub };
        let v_rad = PI / (v_sub - 1) as f32;
        let u_rad = PI * 2. / u_sub as f32;

        // Points
        let mut points = Vec::with_capacity((u_sub * (v_sub - 2) + 2) as usize);
        points.push(Vec3::new(0., 0., r));
        for v in 1..v_sub-1 {
            let z = (v_rad * v as f32).cos();
            for u in 0..u_sub {
                let u = u_rad * u as f32;
                points.push(Vec3::new(u.cos(), u.sin(), z));
            }
        }
        points.push(Vec3::new(0., 0., -r));
        let last = (points.len() - 1) as u32;
        
        // Indices
        let indices = if wire {
            let mut indices = Vec::with_capacity(
                ((v_sub-2) * u_sub + (v_sub-1) * u_sub) as usize
            );
            for v in 0..v_sub-1 {
                let p_shift = if v == 0 { 1 } else  { 1 + (v - 1) * u_sub };
                for u in 0..u_sub {
                    // v-dim
                    if v == 0 {
                        indices.push(0);
                    }
                    indices.push(u + p_shift);
                    if v != (v_sub - 2) && v != 0 {
                        indices.push(u + p_shift + u_sub);
                    }
                    if v == (v_sub - 2) {
                        indices.push(last);
                    }
                    // u_dim
                    if v != 0 {
                        indices.push(u + p_shift);
                        if u == u_sub -1 {
                            indices.push(u + p_shift - u_sub);
                        } else {
                            indices.push(u + p_shift + 1);
                        }
                    }
                }
            }
            indices
        } else {
            let mut indices = Vec::with_capacity(
                (2 * u_sub * (v_sub - 1)) as usize
            );
            for v in 0..v_sub-1 {
                let p_shift = if v == 0 { 1 } else  { 1 + (v - 1) * u_sub };
                for u in 0..u_sub {
                    if v == 0 {
                        indices.push(0);
                        indices.push(u + p_shift);
                        indices.push(
                            if u == u_sub -1 {
                                p_shift
                            } else {
                                u + p_shift + 1
                            }
                        );
                    } else if v == v_sub-2 {
                        indices.push(u + p_shift);
                        indices.push(last);
                        indices.push(
                            if u == u_sub -1 {
                                p_shift
                            } else {
                                u + p_shift + 1
                            }
                        )
                    } else {
                        let p0 = u + p_shift;
                        let p1 = p0 + u_sub;
                        let p2 = if u == u_sub-1 {
                            p_shift + u_sub
                        } else {
                            p1 + 1
                        };
                        let p3 = if u == u_sub-1 {
                            p_shift
                        } else {
                            p0 + 1
                        };
                        indices.push(p0);
                        indices.push(p1);
                        indices.push(p2);
        
                        indices.push(p2);
                        indices.push(p3);
                        indices.push(p0);
                    }
                }
            }
            indices
        };

        Self::new_raw(InitType::Move(points), ColorType::new_uniform(color), IndicesType::new(indices),
        if wire {RenderType::Line} else {RenderType::Triangle} )
    }
    pub fn new_cone(
        r: f32,
        u_sub: u32,
        height: f32,
        color: Vec4,
        wire: bool
    ) -> Self {
        let u_sub = u_sub.max(3);
        let mut points = vec![Vec3::ZERO; u_sub as usize + 2];
        points[1].z = height;
        let u_rad = 2. * PI / u_sub as f32;

        let mut indices = Vec::with_capacity(
            if wire {
                2 * u_sub * 2
            } else {
                2 * u_sub * 3
            } as usize
        );
        (0..u_sub)
            .for_each(|i| {
                let rad = u_rad * i as f32;
                points[(i+2) as usize] = Vec3::new(rad.cos() * r, rad.sin() * r, 0.);
                let next = if i == u_sub { 2 } else { i+1 };
                if wire {
                    indices.push(0);
                    indices.push(i+2);
                    indices.push(i+2);
                    indices.push(1);
                    indices.push(next);
                } else {
                    indices.push(0);
                    indices.push(next);
                    indices.push(i+2);
                    indices.push(i+2);
                    indices.push(next);
                    indices.push(1);
                }
            });
        Self::new_raw(InitType::Move(points), ColorType::new_uniform(color), IndicesType::new(indices),
            if wire {RenderType::Line} else {RenderType::Triangle} )
    }
    pub fn new_cylinder(
        r: f32,
        u_sub: u32,
        height: f32,
        color: Vec4,
        wire: bool
    ) -> Self {
        let u_sub = u_sub.max(3);
        let half = height / 2.;
        let mut points = vec![Vec3::ZERO; 2 * u_sub as usize + 2];
        points[0].z = half;
        points[1].z = -half;
        let mut indices = Vec::with_capacity(
            if wire {
                3 * u_sub
            } else {
                4 * u_sub
            } as usize
        );
        let u_rad = 2. * PI / u_sub as f32;
        (0..u_sub).for_each(|i| {
            let rad = u_rad * i as f32;
            let c = rad.cos() * r;
            let s = rad.sin() * r;

            let p0 = i+2;
            let p1 = p0 + u_sub;
            let p3 = if i==u_sub-1 { 2 } else { i+1 };
            let p2 = p3 + u_sub;

            points[(i + 2) as usize] = Vec3::new(c, s, half);
            points[(i + 2 + u_sub) as usize] = Vec3::new(c, s, -half);


            if wire {
                indices.push(p0);
                indices.push(p1);

                indices.push(p0);
                indices.push(p3);

                indices.push(p1);
                indices.push(p2);
            } else {
                indices.push(p0);
                indices.push(p3);
                indices.push(0);

                indices.push(p0);
                indices.push(p1);
                indices.push(p2);

                indices.push(p2);
                indices.push(p3);
                indices.push(p0);

                indices.push(p1);
                indices.push(1);
                indices.push(p2);
            }
        });
        Self::new_raw(InitType::Move(points), ColorType::new_uniform(color), IndicesType::new(indices),
            if wire {RenderType::Line} else {RenderType::Triangle} )
    }
    pub fn new_arrow(
        arrow_radius: f32,
        height: f32,
        tail_ratio: f32,
        tail_height_ratio: f32,
        u_sub: u32,
        color: Vec4,
        wire: bool,
    ) -> Self {
        let cylinder_height = height * tail_height_ratio;
        let Self {
            points, mut indices, ..
        } = Self::new_cone(arrow_radius, u_sub, height * (1. - tail_height_ratio), color, wire);
        let (tail_points, tail_indices) = if wire {
            (vec![Vec3::ZERO, Vec3::new(0., 0., height * tail_height_ratio)], vec![0, 1])
        } else {
            let Self {
                points: t_points, indices: t_indices, ..
            } = Self::new_cylinder(arrow_radius * tail_ratio, u_sub, cylinder_height, color, wire);
            (t_points, t_indices)
        };
        let n = indices.len() as u32;
        tail_indices.iter().for_each(|i| {
            indices.push(i + n);
        });
        let mut arrow_points = Vec::with_capacity(points.len() + tail_points.len());
        for p in points {
            arrow_points.push(Vec3::new(p.x, p.y, p.z + cylinder_height));
        }
        let half = cylinder_height / 2.;
        for p in tail_points {
            arrow_points.push(Vec3::new(p.x, p.y, p.z + half));
        }
        
        Self::new_raw(InitType::Move(arrow_points),
            ColorType::new_uniform(color), IndicesType::new(indices),
            if wire {RenderType::Line} else {RenderType::Triangle})
    }
}