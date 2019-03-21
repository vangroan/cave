
use crate::grid::GridPosition;

pub type Heuristic = fn(a: &GridPosition, b: &GridPosition) -> u32;

fn manhatten(a: &GridPosition, b: &GridPosition) -> u32 {
    ((a.x() - b.x()).abs() + (a.y() - b.y()).abs() + (a.z() - b.z()).abs()) as u32
}

fn euler(a: &GridPosition, b: &GridPosition) -> u32 {
    let x = ((a.x() - b.x()).pow(2) + (a.y() - b.y()).pow(2) + (a.z() - b.z()).pow(2)) as f64;
    x.sqrt().floor() as u32
}
