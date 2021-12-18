use libadvent::*;

fn simulate(target_x: (i32, i32), target_y: (i32, i32), mut x_speed: i32, mut y_speed: i32) -> bool {
    let (mut x, mut y) = (0, 0);

    while y >= target_y.0 {
        x += x_speed;
        y += y_speed;

        if x_speed > 0 {
            x_speed -= 1;
        }
        else if x_speed < 0 {
            x_speed += 1;
        }
        y_speed -= 1;

        if x >= target_x.0 && x <= target_x.1 && y >= target_y.0 && y <= target_y.1 {
            return true;
        }
    }

    return false;
}

fn main() {
    let input = must_read_input();
    let (x, y) = input.split_once(",").unwrap();
    let target_x: (i32, i32) = x.split("=").last().unwrap().split_once("..").map(|(min, max)| (min.parse().unwrap(), max.parse().unwrap())).unwrap();
    let target_y: (i32, i32) = y.split("=").last().unwrap().split_once("..").map(|(min, max)| (min.parse().unwrap(), max.parse().unwrap())).unwrap();

    let (mut max_x, mut max_y) = (0, 0);
    let mut count = 0;
    for y in -500..500 {
        for x in 1..200 {
            if simulate(target_x, target_y, x, y) {
                count += 1;
                if y * (y + 1) / 2 > max_y {
                    max_y = y;
                    max_x = x;
                }
            }
        }
    }

    println!("{} {} {} {}", max_x, max_y, max_y * (max_y + 1) / 2, count);
}
