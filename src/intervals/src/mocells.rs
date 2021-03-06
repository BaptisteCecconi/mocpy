use crate::idx::Idx;
use crate::qty::MocQty;
use crate::mocell::{Cell, CellOrCellRange};
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub struct Cells<T: Idx>(pub Vec<Cell<T>>);
impl<T: Idx> Cells<T> {
  pub fn new(cells: Vec<Cell<T>>) -> Self {
    Self(cells)
  }
  pub fn cells(&self) -> &Vec<Cell<T>> { &self.0 }
}

#[derive(Debug)]
pub struct MocCells<T: Idx, Q: MocQty<T>>(pub Cells<T>, PhantomData<Q>);
impl<T: Idx, Q: MocQty<T>> MocCells<T, Q> {
  pub fn new(cells: Cells<T>) -> Self {
    Self(cells, PhantomData)
  }
  pub fn cells(&self) -> &Cells<T> { &self.0 }
}
// We could have chosen (Vec<MocCell<T, Q>)

#[derive(Debug)]
pub struct CellOrCellRanges<T: Idx>(pub Vec<CellOrCellRange<T>>);
impl<T: Idx> CellOrCellRanges<T> {
  pub fn new(elems: Vec<CellOrCellRange<T>>) -> Self {
    Self(elems)
  }
  pub fn elems(&self) -> &Vec<CellOrCellRange<T>> { &self.0 }
}

#[derive(Debug)]
pub struct MocCellOrCellRanges<T: Idx, Q: MocQty<T>>(pub CellOrCellRanges<T>, PhantomData<Q>);
impl<T: Idx, Q: MocQty<T>> MocCellOrCellRanges<T, Q> {
  pub fn new(cells_or_cellranges: CellOrCellRanges<T>) -> Self {
    Self(cells_or_cellranges, PhantomData)
  }
  pub fn elems(&self) -> &CellOrCellRanges<T> { &self.0 }
}