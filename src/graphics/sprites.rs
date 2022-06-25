// use bevy::prelude::*;

pub fn atlas_uvs(x: u8, y: u8, size: u8) -> Vec::<[f32; 2]> {
    let step = 1.0 / size as f32;
    let base_u = step * x as f32;
    let base_v = step * y as f32;

    vec!(
        [base_u, base_v],
        [base_u, base_v + step],
        [base_u + step, base_v + step],
        [base_u + step, base_v]
    )
}