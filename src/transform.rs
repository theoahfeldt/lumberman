use nalgebra::{Matrix2, Matrix3, Matrix4, Rotation2, Translation2, Translation3, UnitQuaternion};

#[derive(Clone)]
pub struct Transform {
    pub scale: Option<[f32; 3]>,
    pub rotation: Option<UnitQuaternion<f32>>,
    pub translation: Option<Translation3<f32>>,
}

impl Transform {
    pub fn to_matrix(&self) -> Matrix4<f32> {
        let mut local_transform = Matrix4::<f32>::identity();
        if let Some(ref scale) = self.scale {
            let scale = Matrix3::from_partial_diagonal(&scale[..]);
            local_transform = scale.to_homogeneous() * local_transform;
        }
        if let Some(ref rotation) = self.rotation {
            local_transform = rotation.to_homogeneous() * local_transform
        }
        if let Some(ref translation) = self.translation {
            local_transform = translation.to_homogeneous() * local_transform
        }
        local_transform
    }

    pub fn new() -> Self {
        Self {
            scale: None,
            rotation: None,
            translation: None,
        }
    }
}

pub struct Transform2 {
    pub scale: Option<[f32; 2]>,
    pub rotation: Option<f32>, // Radians
    pub translation: Option<Translation2<f32>>,
}

impl Transform2 {
    pub fn to_matrix(&self) -> Matrix3<f32> {
        let mut local_transform = Matrix3::<f32>::identity();
        if let Some(ref scale) = self.scale {
            let scale = Matrix2::from_partial_diagonal(&scale[..]);
            local_transform = scale.to_homogeneous() * local_transform;
        }
        if let Some(rotation) = self.rotation {
            local_transform = Rotation2::new(rotation).to_homogeneous() * local_transform
        }
        if let Some(ref translation) = self.translation {
            local_transform = translation.to_homogeneous() * local_transform
        }
        local_transform
    }

    pub fn new() -> Self {
        Self {
            scale: None,
            rotation: None,
            translation: None,
        }
    }
}