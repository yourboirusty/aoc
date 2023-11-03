use std::cmp::max;
use std::cmp::min;

use crate::day::Day;
use crate::day::Solveable;

fn get_dist(len: usize, start: usize, dir: i8) -> usize {
    if dir == 0 {
        return len * 100;
    } else if dir > 0 {
        (len - 1 - start) / dir.abs() as usize
    } else {
        start / dir.abs() as usize
    }
}

fn get_new_coord(start: usize, offset: i8, dir: i8) -> usize {
    (start as i8 + offset * dir) as usize
}

trait Grid {
    fn get_at_coord(&self, start: (usize, usize), offset: i8, dir: (i8, i8)) -> u64;
    fn in_bounds(&self, start: (usize, usize), offset: i8, dir: (i8, i8)) -> bool;
}
impl Grid for Vec<Vec<u64>> {
    fn in_bounds(&self, start: (usize, usize), offset: i8, dir: (i8, i8)) -> bool {
        let new_coord = (
            get_new_coord(start.0, offset, dir.0),
            get_new_coord(start.1, offset, dir.1),
        );
        new_coord.0 < self.len() && new_coord.1 < self[0].len()
    }
    fn get_at_coord(&self, start: (usize, usize), offset: i8, dir: (i8, i8)) -> u64 {
        self[get_new_coord(start.0, offset, dir.0)][get_new_coord(start.1, offset, dir.1)]
    }
}

fn get_max_product(grid: &Vec<Vec<u64>>, start: (usize, usize), dir: (i8, i8)) -> u64 {
    // dist = number of spaces available after start
    let max_distance = min(
        get_dist(grid.len(), start.0, dir.0),
        get_dist(grid[0].len(), start.1, dir.1),
    );
    if max_distance < 3 {
        return 0;
    }
    let mut product = (0i8..4)
        .map(|i| grid.get_at_coord(start, i, dir))
        .product::<u64>();
    let mut max_product = product;
    for i in 4..max_distance {
        if !grid.in_bounds(start, i as i8, dir) {
            break;
        }
        if grid.get_at_coord(start, i as i8 - 4, dir) == 0 {
            product = (0..4)
                .map(|j| grid.get_at_coord(start, i as i8 - 3 + j, dir))
                .product();
        } else {
            product = product / grid.get_at_coord(start, i as i8 - 4, dir)
                * grid.get_at_coord(start, i as i8, dir);
        }
        max_product = std::cmp::max(product, max_product);
    }
    max_product
}

struct Part1;
impl Solveable for Part1 {
    fn solve(&self, lines: &Vec<String>) -> String {
        let grid = lines
            .iter()
            .map(|l| l.split(" "))
            .map(|l| l.map(|n| n.parse::<u64>().unwrap()).collect::<Vec<u64>>())
            .collect::<Vec<Vec<u64>>>();
        let mut max_product = 0;
        // horizontal/vertical
        for start in 0..grid.len() {
            for dir in [
                (1, 0),
                (0, 1),
                (1, 1),
                (1, -1),
                (-1, 1),
                (-1, -1),
                (-1, 0),
                (0, -1),
            ] {
                max_product = max(max_product, get_max_product(&grid, (start, 0), dir));
                max_product = max(max_product, get_max_product(&grid, (0, start), dir));
                max_product = max(
                    max_product,
                    get_max_product(&grid, (start, grid[0].len() - 1), dir),
                );
                max_product = max(
                    max_product,
                    get_max_product(&grid, (grid.len() - 1, start), dir),
                );
            }
        }

        max_product.to_string()
    }
}

get_day_fn!(Part1);
