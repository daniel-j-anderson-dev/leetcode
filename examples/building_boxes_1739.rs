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
    let mut box_count = n;

    // true means there is a box at that spot
    // outer most collection is height
    let mut storage_room = vec![vec![vec![false; n]; n]; n];

    'up_down: for i in (0..n).cycle() {
        let is_wall_below = i == 0;
        let is_wall_above = i == n - 1;

        'left_right: for j in (0..n).cycle() {
            let is_wall_left = j == 0;
            let is_wall_right = j == n - 1;

            'forward_backward: for k in (0..n).cycle() {
                let is_wall_behind = k == 0;
                let is_wall_infront = k == n - 1;
                
                let is_box_below = !is_wall_below && storage_room[i - 1][j][k];
                let is_box_left = !is_wall_left && storage_room[i][j - 1][k];
                let is_box_right = !is_wall_right && storage_room[i][j + 1][k];
                let is_box_behind = !is_wall_behind && storage_room[i][j][k - 1];
                let is_box_infront = !is_wall_infront && storage_room[i][j][k + 1];

                if is_box_below && (is_box_left || is_wall_left) && (is_box_right || is_box_right) && (is_box_behind || is_wall_behind) && (is_box_infront || is_wall_infront) {
                    // place box at (i, j, k)
                    storage_room[i][j][k] = true;
                    box_count -= 1;
                } else if is_wall_below && is_wall_left && is_wall_behind && !storage_room[i][j][k] {
                    storage_room[i][j][k] = true;
                    box_count -= 1;
                } else if is_wall_below {
                    storage_room[i][j][k] = true;
                    box_count -= 1;
                } else {}

                if box_count == 0 {
                    break 'up_down;
                }
            }
        }
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

#[test]
fn all_cases() {
    case1();
    case2();
    case3();
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

fn main() {}
