use std::ops::{Index, IndexMut};

use glam::UVec2;

/// row-major grid of values backed by a single [Vec]
///
/// uses [glam::Vec2] for indexing
#[derive(Debug)]
pub struct Grid<T> {
    data: Vec<T>,
    dimensions: UVec2,
}

impl<T: Clone> Clone for Grid<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            dimensions: self.dimensions,
        }
    }
}

fn assert_index_in_range(index: UVec2, dimensions: UVec2) {
    assert!(
        index.x < dimensions.x && index.y < dimensions.y,
        "Index out of bounds. {} is not less than {}",
        index,
        dimensions,
    );
}

fn flatten_index(index: UVec2, width: u32) -> usize {
    (index.y * width + index.x) as usize
}

impl<T> Index<UVec2> for Grid<T> {
    type Output = T;

    fn index(&self, index: UVec2) -> &Self::Output {
        assert_index_in_range(index, self.dimensions);
        &self.data[flatten_index(index, self.dimensions.x)]
    }
}

impl<T> IndexMut<UVec2> for Grid<T> {
    fn index_mut(&mut self, index: UVec2) -> &mut Self::Output {
        assert_index_in_range(index, self.dimensions);
        let flat_index = flatten_index(index, self.dimensions.x);
        &mut self.data[flat_index]
    }
}

#[allow(dead_code)]
impl<T> Grid<T> {
    /// initializes the grid by cloning a value
    pub fn from_value(width: u32, height: u32, value: T) -> Self
    where
        T: Clone,
    {
        Self {
            data: vec![value; width as usize * height as usize],
            dimensions: UVec2::new(width, height),
        }
    }

    /// initializes the grid by calling a function for every grid cell
    pub fn from_fn<F>(width: u32, height: u32, func: F) -> Self
    where
        F: Fn(u32, u32) -> T,
    {
        let mut data = Vec::with_capacity(width as usize * height as usize);

        for y in 0..height {
            for x in 0..width {
                data.push(func(x, y));
            }
        }
        Self {
            data,
            dimensions: UVec2::new(width, height),
        }
    }

    pub fn width(&self) -> u32 {
        self.dimensions.x
    }

    pub fn height(&self) -> u32 {
        self.dimensions.y
    }

    pub fn get(&self, index: UVec2) -> Option<&T> {
        self.data.get(flatten_index(index, self.width()))
    }

    pub fn get_mut(&mut self, index: UVec2) -> Option<&mut T> {
        let flat_index = flatten_index(index, self.width());
        self.data.get_mut(flat_index)
    }

    pub fn put(&mut self, index: UVec2, value: T) {
        self[index] = value;
    }

    pub fn iter(&self) -> Cells<T> {
        Cells {
            grid: self,
            index: UVec2::ZERO,
        }
    }

    pub fn enumerate(&self) -> EnumerateCells<T> {
        EnumerateCells {
            grid: self,
            index: UVec2::ZERO,
        }
    }

    pub fn iter_indices(&self) -> CellIndices {
        CellIndices {
            index: UVec2::ZERO,
            dimensions: self.dimensions,
        }
    }
}

#[derive(Clone)]
pub struct Cells<'a, T> {
    grid: &'a Grid<T>,
    index: UVec2,
}

impl<'a, T: 'a> Iterator for Cells<'a, T> {
    type Item = &'a T;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let value = self.grid.get(self.index);
        self.index.x += 1;
        if self.index.x >= self.grid.width() {
            self.index.x = 0;
            self.index.y += 1;
        }
        value
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<'a, T: 'a> ExactSizeIterator for Cells<'a, T> {
    fn len(&self) -> usize {
        flatten_index(self.index, self.grid.width())
    }
}

#[derive(Clone)]
pub struct EnumerateCells<'a, T> {
    grid: &'a Grid<T>,
    index: UVec2,
}

impl<'a, T: 'a> Iterator for EnumerateCells<'a, T> {
    type Item = (UVec2, &'a T);

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let value = self.grid.get(self.index).map(|t| (self.index, t));
        self.index.x += 1;
        if self.index.x >= self.grid.width() {
            self.index.x = 0;
            self.index.y += 1;
        }
        value
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<'a, T: 'a> ExactSizeIterator for EnumerateCells<'a, T> {
    fn len(&self) -> usize {
        flatten_index(self.index, self.grid.width())
    }
}

#[derive(Clone)]
pub struct CellIndices {
    index: UVec2,
    dimensions: UVec2,
}

impl Iterator for CellIndices {
    type Item = UVec2;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let value = Some(self.index).filter(|_| self.index.y < self.dimensions.y);
        self.index.x += 1;
        if self.index.x >= self.dimensions.x {
            self.index.x = 0;
            self.index.y += 1;
        }
        value
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl ExactSizeIterator for CellIndices {
    fn len(&self) -> usize {
        flatten_index(self.index, self.dimensions.x)
    }
}
