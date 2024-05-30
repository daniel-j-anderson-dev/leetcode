/// - You have a cubic storeroom where width, length and height of the room are each **n** units.
/// - You are asked to place **n** boxes in this room where each box is a cube of unit side length.
/// - There are however some rules to placing the boxes:
///   - You can place the boxes anywhere on the floor.
///   - If box **x** is placed on top of the box **y**
///     - then each side of the four vertical sides of the box **y** must either be adjacent to another box or to a wall.
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

    for _ in 0..n {
        let valid_positions = valid_positions(&storage_room);
        let (i, j, k) = find_best_position(valid_positions).expect("there are valid positions");
        storage_room[i][j][k] = true;
    }

    let mut floor_box_count = 0;
    for j in 0..n {
        for k in 0..n {
            if storage_room[0][j][k] {
                floor_box_count += 1;
            }
        }
    }

    for i in 0..n {
        println!("row {}", i);
        for j in 0..n {
            for k in 0..n {
                print!("{} ", storage_room[i][j][k]);
            }
            println!();
        }
        println!();
    }

    return floor_box_count;
}

fn find_best_position(
    indexes: impl Iterator<Item = (usize, usize, usize)>,
) -> Option<(usize, usize, usize)> {
    indexes.fold(None, |best, current| match best {
        None => Some(current),
        Some(best) => {
            let item_avg = (current.1 + current.2) as f64 / 2.0;
            let best_avg = (best.1 + best.2) as f64 / 2.0;
            if (current.0 > best.0) || (current.0 == best.0 && item_avg < best_avg) {
                Some(current)
            } else {
                Some(best)
            }
        }
    })
}

/// Returns an iterator yielding each valid spot's index
fn valid_positions(
    storage_room: &Vec<Vec<Vec<bool>>>,
) -> impl Iterator<Item = (usize, usize, usize)> {
    let mut valid_positions = Vec::new();

    let n = storage_room.len();

    'vertical: for i in 0..n {
        let is_wall_below = i == 0;
        let is_wall_above = i == n - 1;

        'lateral: for j in 0..n {
            let is_wall_left = j == 0;
            let is_wall_right = j == n - 1;

            'bilateral: for k in 0..n {
                let is_wall_behind = k == 0;
                let is_wall_infront = k == n - 1;

                // is current position occupied?
                if storage_room[i][j][k] {
                    continue 'bilateral;
                }

                // is the current position supported by the floor?
                if is_wall_below {
                    valid_positions.push((i, j, k));
                    continue 'bilateral;
                }

                // is the current position supported by another box below the current position?
                if storage_room[i - 1][j][k] {
                    // is the box below supported on all four sides?
                    if (is_wall_left || storage_room[i - 1][j - 1][k])
                        && (is_wall_right || storage_room[i - 1][j + 1][k])
                        && (is_wall_behind || storage_room[i - 1][j][k - 1])
                        && (is_wall_infront || storage_room[i - 1][j][k + 1])
                    {
                        valid_positions.push((i, j, k));
                        continue 'bilateral;
                    }
                }
            }
        }
    }

    return valid_positions.into_iter();
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_valid_positions() {
        let sr1 = vec![
            vec![
                // level 0 Floor
                vec![true, true, false], // front
                vec![true, false, false],
                vec![false, false, false], // back
            ],
            vec![
                // level 1
                vec![false, false, false], // front
                vec![false, false, false],
                vec![false, false, false], // back
            ],
            vec![
                // level 2 Ceiling
                vec![false, false, false], // front
                vec![false, false, false],
                vec![false, false, false], // back
            ],
        ];

        let expected_valid_sr1 = &[
            (0, 0, 2),
            (0, 1, 1),
            (0, 1, 2),
            (0, 2, 0),
            (0, 2, 1),
            (0, 2, 2),
            (1, 0, 0),
        ];

        let valid_sr1 = valid_positions(&sr1).collect::<Vec<_>>();

        assert_eq!(valid_sr1.as_slice(), expected_valid_sr1);
    }
    #[test]
    fn case1() {
        assert_eq!(3, minimum_boxes(3));
    }
    #[test]
    fn case2() {
        assert_eq!(3, minimum_boxes(4));
    }
    #[test]
    fn case3() {
        assert_eq!(6, minimum_boxes(10));
    }
}

fn main() {}
