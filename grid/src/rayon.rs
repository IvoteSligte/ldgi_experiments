use crate::{CellIndices, Cells, EnumerateCells, Grid};

use rayon::iter::{IterBridge, ParallelBridge};

impl<T> Grid<T> {
    pub fn par_iter<'a>(&'a self) -> IterBridge<Cells<T>>
    where
        Cells<'a, T>: Send,
        <Cells<'a, T> as Iterator>::Item: Send,
    {
        <Cells<T> as ParallelBridge>::par_bridge(self.iter())
    }

    pub fn par_enumerate<'a>(&'a self) -> IterBridge<EnumerateCells<T>>
    where
        EnumerateCells<'a, T>: Send,
        <EnumerateCells<'a, T> as Iterator>::Item: Send,
    {
        <EnumerateCells<T> as ParallelBridge>::par_bridge(self.enumerate())
    }

    pub fn par_iter_indices(&self) -> IterBridge<CellIndices>
    where
        CellIndices: Send,
        <CellIndices as Iterator>::Item: Send,
    {
        <CellIndices as ParallelBridge>::par_bridge(self.iter_indices())
    }
}
