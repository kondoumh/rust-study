pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

//use a::series::of;

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

//use TrafficLight::{Red, Yellow};

fn main() {
    a::series::of::nested_modules();
    let green = TrafficLight::Green;
}
