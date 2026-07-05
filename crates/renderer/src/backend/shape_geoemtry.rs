use vyxen_geometry::{Polygon, ShapeType};
use vyxen_math::Vector2;

use crate::{Sprite, backend::Vertex};

pub const CIRCLE_SEGMENTS: usize = 32;

pub fn sprite_geometry(sprite: &Sprite) -> Option<(Vec<Vertex>, Vec<u16>)> {
    match sprite.get_vertices() {
        Some(shape_type) => Some(shape_geometry(shape_type)),
        None => None,
    }
}

pub fn shape_geometry(shape_type: &ShapeType) -> (Vec<Vertex>, Vec<u16>) {
    match shape_type {
        ShapeType::Circle(circle) => circle_geometry(circle.get_radius()),
        ShapeType::Box(bx) => box_geometry(bx),
        ShapeType::Polygon(poly) => polygon_geometry(poly),
        ShapeType::Concave(polygons) => concave_geometry(polygons),
    }
}

pub fn circle_geometry(radius: f32) -> (Vec<Vertex>, Vec<u16>) {
    let mut vertices = Vec::with_capacity(CIRCLE_SEGMENTS + 1);
    let mut indices = Vec::with_capacity(CIRCLE_SEGMENTS * 3);

    vertices.push(Vertex {
        position: [0.0, 0.0, 0.0],
        tex_coords: [0.5, 0.5],
    });

    for i in 0..=CIRCLE_SEGMENTS {
        let angle = (i as f32 / CIRCLE_SEGMENTS as f32) * std::f32::consts::TAU;
        let x = radius * angle.cos();
        let y = radius * angle.sin();
        vertices.push(Vertex {
            position: [x, y, 0.0],
            tex_coords: [x / (radius * 2.0) + 0.5, y / (radius * 2.0) + 0.5],
        });
    }

    for i in 1..=CIRCLE_SEGMENTS {
        indices.push(0);
        indices.push(i as u16);
        indices.push((i + 1) as u16);
    }

    (vertices, indices)
}

pub fn box_geometry(bx: &vyxen_geometry::Box) -> (Vec<Vertex>, Vec<u16>) {
    let vertex_positions = bx.get_vertices();
    let bounds = compute_bounds(vertex_positions);

    let vertices = vertex_positions
        .iter()
        .map(|position| Vertex {
            position: [position.x, position.y, 0.0],
            tex_coords: uv_from_position(*position, bounds),
        })
        .collect();

    let indices = vec![0, 1, 2, 2, 3, 0];
    (vertices, indices)
}

pub fn polygon_geometry(poly: &Polygon) -> (Vec<Vertex>, Vec<u16>) {
    let positions = poly.get_vertices();
    if positions.len() < 3 {
        return (Vec::new(), Vec::new());
    }

    let bounds = compute_bounds(positions);
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for triangle in Polygon::triangulate(positions) {
        let base = vertices.len() as u16;
        for position in triangle.get_vertices() {
            vertices.push(Vertex {
                position: [position.x, position.y, 0.0],
                tex_coords: uv_from_position(*position, bounds),
            });
        }
        indices.extend_from_slice(&[base, base + 1, base + 2]);
    }

    (vertices, indices)
}

pub fn concave_geometry(polygons: &[Polygon]) -> (Vec<Vertex>, Vec<u16>) {
    let mut all_positions = Vec::new();
    for polygon in polygons {
        all_positions.extend_from_slice(polygon.get_vertices());
    }

    let bounds = compute_bounds(&all_positions);
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for polygon in polygons {
        if polygon.get_vertices().len() == 3 {
            let base = vertices.len() as u16;
            for position in polygon.get_vertices() {
                vertices.push(Vertex {
                    position: [position.x, position.y, 0.0],
                    tex_coords: uv_from_position(*position, bounds),
                });
            }
            indices.extend_from_slice(&[base, base + 1, base + 2]);
        } else {
            for triangle in Polygon::triangulate(polygon.get_vertices()) {
                let base = vertices.len() as u16;
                for position in triangle.get_vertices() {
                    vertices.push(Vertex {
                        position: [position.x, position.y, 0.0],
                        tex_coords: uv_from_position(*position, bounds),
                    });
                }
                indices.extend_from_slice(&[base, base + 1, base + 2]);
            }
        }
    }

    (vertices, indices)
}

pub fn compute_bounds(positions: &[Vector2]) -> (Vector2, Vector2) {
    let mut min = Vector2 { x: f32::INFINITY, y: f32::INFINITY };
    let mut max = Vector2 { x: f32::NEG_INFINITY, y: f32::NEG_INFINITY };

    for position in positions {
        min.x = min.x.min(position.x);
        min.y = min.y.min(position.y);
        max.x = max.x.max(position.x);
        max.y = max.y.max(position.y);
    }

    (min, max)
}

pub fn uv_from_position(position: Vector2, bounds: (Vector2, Vector2)) -> [f32; 2] {
    let (min, max) = bounds;
    let width = (max.x - min.x).max(0.0001);
    let height = (max.y - min.y).max(0.0001);

    [
        (position.x - min.x) / width,
        (position.y - min.y) / height,
    ]
}