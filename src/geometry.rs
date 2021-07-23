use crate::object::Mesh;
use crate::semantics::{Vertex, VertexNormal, VertexPosition, VertexUV};

pub fn quad(height: f32, width: f32) -> Mesh {
    let vertices = vec![
        Vertex::new(
            // Upper left
            VertexPosition::new([-0.5 * width, 0.5 * height, 0.]),
            VertexNormal::new([0., 0., 1.]),
            VertexUV::new([0., 1.]),
        ),
        Vertex::new(
            // Upper right
            VertexPosition::new([0.5 * width, 0.5 * height, 0.]),
            VertexNormal::new([0., 0., 1.]),
            VertexUV::new([1., 1.]),
        ),
        Vertex::new(
            // Lower left
            VertexPosition::new([-0.5 * width, -0.5 * height, 0.]),
            VertexNormal::new([0., 0., 1.]),
            VertexUV::new([0., 0.]),
        ),
        Vertex::new(
            // Lower right
            VertexPosition::new([0.5 * width, -0.5 * height, 0.]),
            VertexNormal::new([0., 0., 1.]),
            VertexUV::new([1., 0.]),
        ),
    ];
    let indices = vec![0, 1, 2, 1, 2, 3];
    Mesh { vertices, indices }
}

// Cylinder aligned with z-axis
pub fn cylinder(height: f32, radius: f32, res: u32) -> Mesh {
    let co2 = (0..res + 1)
        .map(|n| std::f32::consts::PI * 2. * (n as f32) / (res as f32))
        .map(|a| (a.cos() * radius, a.sin() * radius));
    let co2s = co2.clone().take(res as usize);
    let top = height / 2.;
    let bot = -top;
    let top_verts = co2.clone().enumerate().map(|(i, (x, y))| {
        Vertex::new(
            VertexPosition::new([x, y, top]),
            VertexNormal::new([x, y, 0.]),
            VertexUV::new([0.25 + 0.75 * (i as f32) / (res as f32), 1.]),
        )
    });
    let bot_verts = co2.clone().enumerate().map(|(i, (x, y))| {
        Vertex::new(
            VertexPosition::new([x, y, bot]),
            VertexNormal::new([x, y, 0.]),
            VertexUV::new([0.25 + 0.75 * (i as f32) / (res as f32), 0.]),
        )
    });
    let top_lid_verts = co2s.clone().map(|(x, y)| {
        Vertex::new(
            VertexPosition::new([x, y, top]),
            VertexNormal::new([0., 0., 1.]),
            VertexUV::new([x / 8. / radius + 1. / 8., y / 2. / radius + 0.5]),
        )
    });
    let bot_lid_verts = co2s.clone().map(|(x, y)| {
        Vertex::new(
            VertexPosition::new([x, y, bot]),
            VertexNormal::new([0., 0., -1.]),
            VertexUV::new([x / 8. / radius + 1. / 8., y / 2. / radius + 0.5]),
        )
    });
    let vertices = top_verts
        .chain(bot_verts)
        .chain(top_lid_verts)
        .chain(bot_lid_verts)
        .collect();
    let first_top_lid_idx = 2 * (res + 1);
    let top_lid_indices = (first_top_lid_idx + 1..first_top_lid_idx + res - 1)
        .flat_map(|n| vec![first_top_lid_idx, n, n + 1]);
    let first_bot_lid_idx = 2 * (res + 1) + res;
    let bot_lid_indices = (first_bot_lid_idx + 1..first_bot_lid_idx + res - 1)
        .flat_map(|n| vec![first_bot_lid_idx, n, n + 1]);
    let indices = (0..res)
        .flat_map(|n| vec![n, n + 1, n + res + 1, n + res + 1, n + 1 + res + 1, n + 1])
        .chain(top_lid_indices)
        .chain(bot_lid_indices)
        .collect();
    Mesh { vertices, indices }
}
