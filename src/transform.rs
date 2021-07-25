use rapier3d::na::{Matrix2, Matrix3, Matrix4, Rotation2, Translation3};

pub fn rotation2(rad: f32) -> Matrix4<f32> {
    Rotation2::new(rad).to_homogeneous().to_homogeneous()
}

pub fn scale2(x: f32, y: f32) -> Matrix4<f32> {
    Matrix2::from_partial_diagonal(&[x, y])
        .to_homogeneous()
        .to_homogeneous()
}

pub fn scale3(x: f32, y: f32, z: f32) -> Matrix4<f32> {
    Matrix3::from_partial_diagonal(&[x, y, z]).to_homogeneous()
}

pub fn translation2(x: f32, y: f32) -> Matrix4<f32> {
    translation3(x, y, 0.)
}

pub fn translation3(x: f32, y: f32, z: f32) -> Matrix4<f32> {
    Translation3::new(x, y, z).to_homogeneous()
}
