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

pub fn turn_90_ccw(direction: GridDirection) -> GridDirection {
    match direction {
        GridDirection::U => { GridDirection::L },
        GridDirection::UR => { GridDirection::UL },
        GridDirection::R => { GridDirection::U },
        GridDirection::DR => { GridDirection::UR },
        GridDirection::D => { GridDirection::R },
        GridDirection::DL => { GridDirection::DR },
        GridDirection::L => { GridDirection::D },
        GridDirection::UL => { GridDirection::DL },
    }
}

pub fn from_char(value: char) -> Option<GridDirection> {
    match value {
        '^' => { Some(GridDirection::U) },
        '>' => { Some(GridDirection::R) },
        'v' => { Some(GridDirection::D) },
        '<' => { Some(GridDirection::L) },
        _ => { None },
    }
}
