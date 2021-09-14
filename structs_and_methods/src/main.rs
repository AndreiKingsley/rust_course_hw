use structs_and_methods::structs::*;
use std::collections::HashMap;

fn main() {
    // I use a coordinate system where y axis is downward
    /*
    let rect_1_1 = Rect {
        upper_left_vertex: Point { x: 1.0, y: 2.1 },
        lower_right_vertex: Point { x: 19.22, y: 19.91 },
    };

    let point_1_1 = Point { x: 1.0, y: 2.0 };

    println!("1.1. {:?} contains {:?}:", rect_1_1, point_1_1);
    println!("{}", rect_1_1.contains(&point_1_1));

    let rect_2_1 = Rect {
        upper_left_vertex: Point { x: -11.11, y: 22.22 },
        lower_right_vertex: Point { x: 33.33, y: 44.55 },
    };

    let point_2_1 = Point { x: 12.0, y: 44.0 };

    println!("1.2. {:?} contains {:?}:", rect_2_1, point_2_1);
    println!("{}", rect_2_1.contains(&point_2_1));

    let circle_3_1 = Circle {
        center: Point { x: 0.0, y: 0.0 },
        radius: 150.0,
    };

    let point_3_1 = Point { x: 0.0, y: 150.0 };

    println!("1.3. {:?} contains {:?}:", circle_3_1, point_3_1);
    println!("{}", circle_3_1.contains(&point_3_1));

    let circle_4_1 = Circle {
        center: Point { x: -100.0, y: -100000.0 },
        radius: 0.2345,
    };

    let point_4_1 = Point { x: 100.100, y: 100.500 };

    println!("1.4. {:?} contains {:?}:", circle_4_1, point_4_1);
    println!("{}", circle_4_1.contains(&point_4_1));

    let rect_5_1 = Rect {
        upper_left_vertex: Point { x: 12.0, y: 25.71 },
        lower_right_vertex: Point { x: 13.5, y: 29.71 },
    };

    println!("1.5. Area of {:?}:", rect_5_1);
    println!("{}", rect_5_1.area());

    let circle_6_1 = Circle {
        center: Point { x: -1748.771199, y: -121.2119 },
        radius: 10.0,
    };

    println!("1.6. Area of {:?}:", circle_6_1);
    println!("{}", circle_6_1.area());

    let figure_7_1 = Figure::Rect(Rect {
        upper_left_vertex: Point { x: -0.1111, y: -0.2222 },
        lower_right_vertex: Point { x: 14.88, y: 13.37 },
    });

    let point_7_1 = Point { x: -1.0, y: -20.0 };

    println!("1.7.{:?} contains {:?}:", figure_7_1, point_7_1);
    println!("{}", figure_7_1.contains(&point_7_1));

    let figure_8_1 = Figure::Circle(Circle {
        center: Point { x: 12.3, y: 13.3 },
        radius: 42424242.42,
    });

    let point_8_1 = Point { x: 0.0, y: 1000.1 };

    println!("1.8. {:?} contains {:?}:", figure_8_1, point_8_1);
    println!("{}", figure_8_1.contains(&point_8_1));

    let mut hash_map_circle: HashMap<Point<f64>, i32> = HashMap::new();
    hash_map_circle.insert(
        point_1_1,
        12
    );
    hash_map_circle.insert(
        point_2_1,
        0
    );
    println!("2.1. Can create HashMap with Point keys: {:?}", hash_map_circle);

    let mut hash_map_rect: HashMap<Rect<f64>, i32> = HashMap::new();
    hash_map_rect.insert(
        rect_1_1,
        12
    );
    hash_map_rect.insert(
        rect_5_1,
        0
    );
    println!("2.2. Can create HashMap with Rect keys: {:?}", hash_map_rect);

    let mut hash_map_figure: HashMap<Figure<f64>, i32> = HashMap::new();
    hash_map_figure.insert(
        figure_7_1,
        12
    );
    hash_map_figure.insert(
        figure_8_1,
        0
    );
    println!("2.3. Can create HashMap with Figure keys: {:?}", hash_map_figure);

    println!("2.4 Default Point: {:?}", Point::default());

    println!("2.5 Default Point: {:?}", Rect::default());

    println!("2.6 Default Point: {:?}", Circle::default());

     */

    let point_1: Point<i32> = Point::default();
    println!("Default for Point: {:?}", point_1);

    let point_2: Point<i32> = Point {
        x: 3,
        y: -4,
    };

    let mut point_hash_map = HashMap::new();

    point_hash_map.insert(point_1, 0);
    point_hash_map.insert(point_2, -1);

    println!("Can create hash map with Point keys: {:?}", point_hash_map);

    let rect_1: Rect<u64> = Rect::default();
    println!("Default for Rect: {:?}", rect_1);

    let rect_2: Rect<u64> = Rect {
        upper_left_vertex: Point {
            x: 12,
            y: 10,
        },
        lower_right_vertex: Point {
            x: 20,
            y: 3000,
        },
    };

    let mut rect_hash_map = HashMap::new();

    rect_hash_map.insert(rect_1, false);
    rect_hash_map.insert(rect_2, true);

    println!("Can create hash map with Rect keys: {:?}", rect_hash_map);

    let circ_1: Circle<usize> = Circle::default();
    println!("Default for Circle: {:?}", circ_1);

    let fig_1 = Figure::Circle(circ_1);
    let fig_2 = Figure::Rect(Rect {
        upper_left_vertex: Point {
            x: 23 as usize,
            y: 10 as usize,
        },
        lower_right_vertex: Point {
            x: 100 as usize,
            y: 10005 as usize,
        },
    });

    let mut fig_hash_map = HashMap::new();

    fig_hash_map.insert(fig_1, "Biba");
    fig_hash_map.insert(fig_2, "Aboba");

    println!("Can create hash map with Rect keys: {:?}", fig_hash_map);
}
