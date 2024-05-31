/// - You have a cubic storeroom where width, length and height of the room are each **n** units.
/// - You are asked to place **n** boxes in this room where each box is a cube of unit side length.
///   - There are however some rules to placing the boxes:
///     - You can place the boxes anywhere on the floor.
///     - If box **x** is placed on top of the box **y**
///       - then each side of the four vertical sides of the box **y** must either be adjacent to another box or to a wall.
///
/// # Given an integer **n**, return the minimum possible number of boxes touching the floor.
///
/// ## Example 1:
///
/// <img src="https://assets.leetcode.com/uploads/2021/01/04/3-boxes.png"></img>
///
/// Input: n = 3
/// Output: 3
/// Explanation: The figure above is for the placement of the three boxes.
/// These boxes are placed in the corner of the room, where the corner is on the left side.
///
/// ## Example 2:
///
/// <img src="https://assets.leetcode.com/uploads/2021/01/04/4-boxes.png"></img>
///
/// Input: n = 4
/// Output: 3
/// Explanation: The figure above is for the placement of the four boxes.
/// These boxes are placed in the corner of the room, where the corner is on the left side.
///
/// ## Example 3:
///
/// <img src="https://assets.leetcode.com/uploads/2021/01/04/10-boxes.png"></img>
///
/// Input: n = 10
/// Output: 6
/// Explanation: The figure above is for the placement of the ten boxes.
/// These boxes are placed in the corner of the room, where the corner is on the back side.
///
/// ## Constraints:
/// - 1 <= n <= 109
pub fn minimum_boxes(n: usize) -> usize {
    // true means there is a box at that spot
    // outer most collection is height
    let mut storage_room = vec![vec![vec![false; n]; n]; n];

    let mut floor_box_count = 0;

    for _box_number in 1..=n {
        let valid_indexes = valid_indexes(&storage_room).collect::<Vec<_>>();
        let (i, j, k) = find_best_index(&valid_indexes).expect("there is always a valid index");
        storage_room[i][j][k] = true;

        // println!("Box #{} placed at ({}, {}, {})\n", _box_number, i, j, k);

        if i == 0 {
            floor_box_count += 1;
        }
    }

    // for i in 0..3 {
    //     println!("level {}", i);
    //     for j in 0..3 {
    //         for k in 0..3 {
    //             print!("{:>5}, ", storage_room[i][j][k]);
    //         }
    //         println!()
    //     }
    //     println!()
    // }

    return floor_box_count;
}

/// This function returns the index that
/// - has the highest value for first subindex,
/// - and the lowest average between the other subindexes
/// - Returns [None] if `indexes` is empty
fn find_best_index(
    indexes: &[(usize, usize, usize)],
) -> Option<(usize, usize, usize)> {
    indexes.iter().cloned().fold(None, |best, current| match best {
        None => Some(current),
        Some(best) => {
            let current_average = (current.1 + current.2) as f64 / 2.0;
            let best_average = (best.1 + best.2) as f64 / 2.0;

            let best =
                if (current.0 > best.0) || (current.0 == best.0 && current_average < best_average) {
                    current
                } else {
                    best
                };

            // println!("current best: level {}, j_k_avg: {}", best.0, (best.1 + best.2) / 2);

            Some(best)
        }
    })
}

/// Returns an iterator yielding all indexes.
fn all_indexes(len: usize) -> impl Iterator<Item = (usize, usize, usize)> {
    (0..len).flat_map(move |i| (0..len).flat_map(move |j| (0..len).map(move |k| (i, j, k))))
}

use rayon::iter::{IntoParallelIterator, ParallelIterator};

/// Returns an iterator yielding each valid spot's index
fn valid_indexes<'a>(
    storage_room: &'a Vec<Vec<Vec<bool>>>,
) -> impl Iterator<Item = (usize, usize, usize)> + 'a {
    let max_index: usize = storage_room.len() - 1;

    // filter out any indexes that can't support a box
    return all_indexes(max_index).filter(move |&(i, j, k)| {
        // where are the walls?
        let is_wall_below = i == 0;
        let is_wall_left = j == 0;
        let is_wall_right = j == max_index;
        let is_wall_behind = k == 0;
        let is_wall_infront = k == max_index;

        // is current index occupied?
        if storage_room[i][j][k] {
            return false;
        }

        // is the current index supported by the floor?
        if is_wall_below {
            // println!("({}, {}, {}) is supported by the floor", i, j, k);
            return true;
        }

        // is the current index supported by another box below the current index?
        if !storage_room[i - 1][j][k] {
            return false;
        }

        // is the box below supported on all four sides?
        if (is_wall_left || storage_room[i - 1][j - 1][k])
            && (is_wall_right || storage_room[i - 1][j + 1][k])
            && (is_wall_behind || storage_room[i - 1][j][k - 1])
            && (is_wall_infront || storage_room[i - 1][j][k + 1])
        {
            // println!("({}, {}, {}) is supported by the box below", i, j, k);
            return true;
        }

        return false;
    });
}

pub fn minimum_boxes_par(n: usize) -> usize {
    // true means there is a box at that spot
    // outer most collection is height
    let mut storage_room = vec![vec![vec![false; n]; n]; n];

    let mut floor_box_count = 0;

    for _box_number in 1..=n {
        let valid_indexes = valid_indexes_par(&storage_room).collect::<Vec<_>>();
        let (i, j, k) = find_best_index(&valid_indexes).expect("there is always a valid index");
        storage_room[i][j][k] = true;

        // println!("Box #{} placed at ({}, {}, {})\n", _box_number, i, j, k);

        if i == 0 {
            floor_box_count += 1;
        }
    }

    // for i in 0..3 {
    //     println!("level {}", i);
    //     for j in 0..3 {
    //         for k in 0..3 {
    //             print!("{:>5}, ", storage_room[i][j][k]);
    //         }
    //         println!()
    //     }
    //     println!()
    // }

    return floor_box_count;
}
/// Returns an iterator yielding all indexes in parallel.
fn all_indexes_par(len: usize) -> impl ParallelIterator<Item = (usize, usize, usize)> {
    (0..len).into_par_iter().flat_map(move |i| (0..len).into_par_iter().flat_map(move |j| (0..len).into_par_iter().map(move |k| (i, j, k))))
}
/// Returns an iterator yielding each valid spot's index
fn valid_indexes_par<'a>(
    storage_room: &'a Vec<Vec<Vec<bool>>>,
) -> impl ParallelIterator<Item = (usize, usize, usize)> + 'a {
    let max_index: usize = storage_room.len() - 1;

    // filter out any indexes that can't support a box
    return all_indexes_par(max_index).filter(move |&(i, j, k)| {
        // where are the walls?
        let is_wall_below = i == 0;
        let is_wall_left = j == 0;
        let is_wall_right = j == max_index;
        let is_wall_behind = k == 0;
        let is_wall_infront = k == max_index;

        // is current index occupied?
        if storage_room[i][j][k] {
            return false;
        }

        // is the current index supported by the floor?
        if is_wall_below {
            // println!("({}, {}, {}) is supported by the floor", i, j, k);
            return true;
        }

        // is the current index supported by another box below the current index?
        if !storage_room[i - 1][j][k] {
            return false;
        }

        // is the box below supported on all four sides?
        if (is_wall_left || storage_room[i - 1][j - 1][k])
            && (is_wall_right || storage_room[i - 1][j + 1][k])
            && (is_wall_behind || storage_room[i - 1][j][k - 1])
            && (is_wall_infront || storage_room[i - 1][j][k + 1])
        {
            // println!("({}, {}, {}) is supported by the box below", i, j, k);
            return true;
        }

        return false;
    });
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn input_3_output_3() {
        assert_eq!(3, minimum_boxes(3));
    }
    #[test]
    fn input_4_output_3() {
        assert_eq!(3, minimum_boxes(4));
    }
    #[test]
    fn input_10_output_6() {
        assert_eq!(6, minimum_boxes(10));
    }
    #[test]
    fn input_15_output_9() {
        assert_eq!(9, minimum_boxes(15));
    }
    #[test]
    fn input_126_output_39() {
        assert_eq!(39, minimum_boxes(126))
    }
    #[test]
    fn input_126_output_39_par() {
        assert_eq!(39, minimum_boxes_par(126))
    }
    #[test]
    fn test_all_indexes() {
        let all_indexes = all_indexes(3).collect::<Vec<_>>();
        let expected_all_indexes = &[
            (0, 0, 0),
            (0, 0, 1),
            (0, 0, 2),
            (0, 1, 0),
            (0, 1, 1),
            (0, 1, 2),
            (0, 2, 0),
            (0, 2, 1),
            (0, 2, 2),
            (1, 0, 0),
            (1, 0, 1),
            (1, 0, 2),
            (1, 1, 0),
            (1, 1, 1),
            (1, 1, 2),
            (1, 2, 0),
            (1, 2, 1),
            (1, 2, 2),
            (2, 0, 0),
            (2, 0, 1),
            (2, 0, 2),
            (2, 1, 0),
            (2, 1, 1),
            (2, 1, 2),
            (2, 2, 0),
            (2, 2, 1),
            (2, 2, 2),
        ];
        assert_eq!(all_indexes, expected_all_indexes);
    }
}

fn main() {}
