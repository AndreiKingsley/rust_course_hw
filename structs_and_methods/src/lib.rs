pub mod structs {
    use std::fmt::{Debug};
    use num_traits::Num;

    #[derive(Debug, Hash)]
    pub struct Point<T: Num> {
        pub x: T,
        pub y: T,
    }

    impl<T: Num> Default for Point<T> {
        fn default() -> Self {
            Point {
                x: T::zero(),
                y: T::zero(),
            }
        }
    }

    impl<T: Num> PartialEq for Point<T> {
        fn eq(&self, other: &Self) -> bool {
            self.x == other.x && self.y == other.y
        }
    }

    impl<T: Num> Eq for Point<T> {}

    #[derive(Debug, Hash)]
    pub struct Rect<T: Num> {
        pub upper_left_vertex: Point<T>,
        pub lower_right_vertex: Point<T>,
    }

    impl<T: Num> PartialEq for Rect<T> {
        fn eq(&self, other: &Self) -> bool {
            self.upper_left_vertex == other.upper_left_vertex &&
                self.lower_right_vertex == other.lower_right_vertex
        }
    }

    impl<T: Num> Eq for Rect<T> {}

    impl<T: Num> Default for Rect<T> {
        fn default() -> Self {
            Rect {
                upper_left_vertex: Point {
                    x: T::zero(),
                    y: T::zero(),
                },
                lower_right_vertex: Point {
                    x: T::one(),
                    y: T::one(),
                }
            }

        }
    }

    #[derive(Debug, Hash)]
    pub struct Circle<T: Num> {
        pub center: Point<T>,
        pub radius: T,
    }

    impl<T: Num> PartialEq for Circle<T> {
        fn eq(&self, other: &Self) -> bool {
            self.center == other.center &&
                self.radius == other.radius
        }
    }

    impl<T: Num> Eq for Circle<T> {}

    impl<T: Num> Default for Circle<T> {
        fn default() -> Self {
            Circle {
                center: Point {
                    x: T::zero(),
                    y: T::zero(),
                },
                radius: T::one(),
            }

        }
    }

    #[derive(Debug, Hash)]
    pub enum Figure<T: Num> {
        Rect(Rect<T>),
        Circle(Circle<T>),
    }

    impl<T: Num> PartialEq for Figure<T> {
        fn eq(&self, other: &Self) -> bool {
            match self {
                Figure::Rect(self_rect) => match other {
                    Figure::Rect(other_rect) => {
                        self_rect == other_rect
                    }
                    _ => false
                }
                Figure::Circle(self_circle) => match other {
                    Figure::Circle(other_circle) => {
                        self_circle == other_circle
                    }
                    _ => false
                }
            }
        }
    }

    impl<T: Num> Eq for Figure<T> {}

}