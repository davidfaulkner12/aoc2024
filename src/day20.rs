#[cfg(test)]
mod tests {
    use core::time;
    use std::{fs, thread};

    use crate::day4::char_matrix;

    use super::*;

    const TEST_DATA: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn test_find_route() {
        let map = char_matrix(TEST_DATA);
    }
}
