#[derive(Clone, Copy, Debug)]
pub enum GridDirection {
    U,
    UR,
    R,
    DR,
    D,
    DL,
    L,
    UL,
}

pub const ALL_GRID_DIRECTIONS: [GridDirection; 8] = [
    GridDirection::U,
    GridDirection::UR,
    GridDirection::R,
    GridDirection::DR,
    GridDirection::D,
    GridDirection::DL,
    GridDirection::L,
    GridDirection::UL,
];
