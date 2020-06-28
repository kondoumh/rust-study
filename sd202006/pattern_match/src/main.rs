enum TempComp {
    Equal,
    Higher,
    Lower,
}

struct Celsius(f64);

#[derive(Copy, Clone, Debug)]
struct Kelvin(f64);

const T0: f64 = 273.15;
const EPS: f64 = 1.0e-10;

trait KelvinConverter {
    fn convert_to_kelvin(&self) -> Kelvin;
}

impl KelvinConverter for Kelvin {
    fn convert_to_kelvin(&self) -> Kelvin {
        self.clone()
    }
}

impl KelvinConverter for Celsius {
    fn convert_to_kelvin(&self) -> Kelvin {
        Kelvin(self.0 + T0)
    }
}

fn comp<T: KelvinConverter, S: KelvinConverter>(x: &T, y: &S) -> TempComp {
    let x_kelvin = x.convert_to_kelvin();
    let y_kelvin = y.convert_to_kelvin();
    if (x_kelvin.0 - y_kelvin.0).abs() < EPS {
        TempComp::Equal
    } else if x_kelvin.0 > y_kelvin.0 {
        TempComp::Higher
    } else {
        TempComp::Lower
    }
}

fn main() {
    let x = Kelvin(300.0);
    let y = Celsius(30.0);
    match comp(&x, &y) {
        TempComp::Equal => println!("Equal"),
        TempComp::Higher => println!("x is higher"),
        TempComp::Lower => println!("x is lower"),
    }
}
