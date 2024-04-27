use crate::{assert_index_in_range, flatten_index, Grid};

use glam::{UVec2, Vec2};

impl<T: lerp::Lerp<f32> + Clone> Grid<T> {
    pub fn sample(&self, index: Vec2) -> T {
        assert!(index.cmpge(Vec2::ZERO).any(), "Index must be positive.");

        let t = index.fract();

        let i00 = index.as_uvec2();
        let i11 = i00 + UVec2::ONE;
        let i01 = UVec2::new(i00.x, i11.y);
        let i10 = UVec2::new(i11.x, i00.y);

        assert_index_in_range(i11, self.dimensions);

        let fi00 = flatten_index(i00, self.dimensions.x);
        let fi11 = flatten_index(i11, self.dimensions.x);
        let fi01 = flatten_index(i01, self.dimensions.x);
        let fi10 = flatten_index(i10, self.dimensions.x);

        let lerped0 =
            <T as lerp::Lerp<f32>>::lerp(self.data[fi00].clone(), self.data[fi10].clone(), t.x);
        let lerped1 =
            <T as lerp::Lerp<f32>>::lerp(self.data[fi01].clone(), self.data[fi11].clone(), t.x);

        <T as lerp::Lerp<f32>>::lerp(lerped0, lerped1, t.y)
    }
}
