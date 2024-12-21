use std::collections::HashSet;
use shared::args::get_reader_for_arg;
use shared::grid::Grid;
use shared::grid_direction::GridDirection;
use shared::point::Point;


type Garden = Grid<char>;

#[derive(Debug)]
struct Plot {
    plant: char,
    points: Vec<Point>,
    perimeter: usize,
    area: usize,
    sides: usize,
}


fn find_plots(garden: &Garden) -> Vec<Plot> {
    let mut plots = Vec::new();
    let mut visited: HashSet<Point> = HashSet::new();

    for i in garden {
        if !visited.contains(&i.point) {
            let mut plot = Plot {
                plant: i.value,
                points: Vec::new(),
                perimeter: 0,
                area: 0,
                sides: 0,
            };

            explore_plot(garden, &mut plot, &mut visited, &i.point);
            plot.sides = count_sides(garden, &plot);

            plots.push(plot);
        }
    }

    plots
}

fn explore_plot(garden: &Garden, plot: &mut Plot, visited: &mut HashSet<Point>, point: &Point) -> bool {
    if let Some(value) = garden.get_point(point) {
        if value == plot.plant {
            if visited.contains(point) {
                return true;
            }
            visited.insert(*point);

            plot.area += 1;
            plot.points.push(*point);

            for neighbor in point.direct_neighbors() {
                if !explore_plot(garden, plot, visited, &neighbor) {
                    plot.perimeter += 1;
                }
            }

            return true;
        }
    }

    false
}

const CORNER_DIRECTIONS: [GridDirection; 5] = [
    GridDirection::U,
    GridDirection::R,
    GridDirection::D,
    GridDirection::L,
    GridDirection::U,
];

fn count_sides(garden: &Garden, plot: &Plot) -> usize {
    let mut sides = 0;

    for point in &plot.points {
        for (target, a, b, corner) in CORNER_DIRECTIONS
            .windows(2)
            .map(|w| (point, point.translate(w[0]), point.translate(w[1]), point.translate(w[0]).translate(w[1]))) {

                let targetv = garden.get_point(target).unwrap();
                let av = garden.get_point(&a);
                let bv = garden.get_point(&b);
                let cornerv = garden.get_point(&corner);

                if av != Some(targetv) && bv != Some(targetv) {
                    sides += 1;
                } else if av == Some(targetv) && bv == Some(targetv) && cornerv != Some(targetv) {
                    sides += 1;
                }
            }
    }

    sides
}


fn main() {
    let file = get_reader_for_arg(1);
    let garden: Garden = Grid::<char>::chars_from_reader(file);

    let plots = find_plots(&garden);

    let price = plots.iter().map(|p| p.area * p.perimeter).sum::<usize>();
    println!("Part 1 Total Price: {}", price);

    let price = plots.iter().map(|p| p.area * p.sides).sum::<usize>();
    println!("Part 2 Total Price: {}", price);
}
