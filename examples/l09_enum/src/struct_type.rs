#[allow(dead_code)]
enum Shape {
    Rectangle {
        height: f64,
        width: f64,
    },
    Triangle {
        height: f64,
        bottom: f64,
    },
    Circle {
        radius: f64,
    },
    Trapezium {
        upper: f64,
        bottom: f64,
        height: f64,
    },
}
impl Shape {
    #[allow(dead_code)]
    pub fn area(&self) -> f64 {
        match self {
            Self::Rectangle { height, width } => height * width,
            Self::Triangle { height, bottom } => (height * bottom) / 2.0,
            Self::Circle { radius } => radius * radius * std::f64::consts::PI,
            Self::Trapezium {
                upper,
                bottom,
                height,
            } => (upper + bottom) * height / 2.0,
        }
    }
}

impl ToString for Shape {
    fn to_string(&self) -> String {
        match self {
            Self::Rectangle { .. } => "四角形",
            Self::Triangle { .. } => "三角形",
            Self::Circle { .. } => "円",
            Self::Trapezium { .. } => "台形",
        }
        .to_string()
    }
}

#[test]
fn shape_area() {
    let rectangle = Shape::Rectangle {
        height: 10.0,
        width: 5.5,
    };
    let triangle = Shape::Triangle {
        height: 10.0,
        bottom: 5.5,
    };
    let circle = Shape::Circle { radius: 3.5 };
    let trapezium = Shape::Trapezium {
        upper: 10.0,
        bottom: 20.0,
        height: 5.0,
    };
    assert_eq!(rectangle.area(), 55.0);
    assert_eq!(triangle.area(), 27.5);
    assert_eq!(circle.area().floor(), 38.0);
    assert_eq!(trapezium.area(), 75.0);
}

#[test]
fn shape_to_string() {
    assert_eq!(Shape::Circle { radius: 2.0 }.to_string(), "円");
}
