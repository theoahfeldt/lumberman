use crate::object::Mesh;
use crate::semantics::{Vertex, VertexNormal, VertexPosition, VertexUV};
use itertools::Itertools;

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

pub fn cylinder(height: f32, radius: f32, res: u32) -> Mesh {
    let co2 = (0..res + 1)
        .map(|n| std::f32::consts::PI * 2. * (n as f32) / (res as f32))
        .map(|a| (a.cos() * radius, a.sin() * radius));
    let (co2_top, co2_bot) = co2.enumerate().tee();
    let top = height / 2.;
    let bot = -top;
    let top_verts = co2_top.map(|(i, (x, y))| {
        Vertex::new(
            VertexPosition::new([x, y, top]),
            VertexNormal::new([x, y, 0.]),
            VertexUV::new([(i as f32) / (res as f32), 1.]),
        )
    });
    let bot_verts = co2_bot.map(|(i, (x, y))| {
        Vertex::new(
            VertexPosition::new([x, y, bot]),
            VertexNormal::new([x, y, 0.]),
            VertexUV::new([(i as f32) / (res as f32), 0.]),
        )
    });
    let vertices = top_verts.chain(bot_verts).collect();
    let indices = (0..res)
        .flat_map(|n| vec![n, n + 1, n + res + 1, n + res + 1, n + 1 + res + 1, n + 1])
        .collect();
    Mesh { vertices, indices }
}
