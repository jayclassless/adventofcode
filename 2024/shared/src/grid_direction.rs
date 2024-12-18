#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
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

pub fn turn_90_cw(direction: GridDirection) -> GridDirection {
    match direction {
        GridDirection::U => { GridDirection::R },
        GridDirection::UR => { GridDirection::DR },
        GridDirection::R => { GridDirection::D },
        GridDirection::DR => { GridDirection::DL },
        GridDirection::D => { GridDirection::L },
        GridDirection::DL => { GridDirection::UL },
        GridDirection::L => { GridDirection::U },
        GridDirection::UL => { GridDirection::UR },
    }
}
