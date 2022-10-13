//Square
struct Square<Var> //Var n'est pas un type mais c'est le nom de la var qui servira à la construction de la struct via l'impl
{
    side: Var,
}
//Il faut une implémentation spécifique par type d'arg que l'on souhaite passer
impl Square<u32> { 
    fn new( t: u32) -> Self {
        Square { side : t}
    }
}

impl Square<f64> {
    fn new( t: f64) -> Self {
        Square { side : t}
    }
}

impl Square<String> {
    fn new( t: &str) -> Self {
        Square { side : t.to_string()}
    }
}

// Area for square
trait Area {
    fn area(&self) -> f64;
}

impl Area for Square<u32> {
    fn area(&self) -> f64 {
        (self.side * self.side).into()
    }
}

impl Area for Square<f64> {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Area for Square<String> {
    fn area(&self) -> f64 {
        self.side.parse::<f64>().unwrap() * self.side.parse::<f64>().unwrap()
    }
}

//Triangle

struct Triangle<T> //arg générique
{
    base: T,
    hight: T,

}

impl Triangle<f64> {
    fn new( b: f64, h: f64) -> Self {
        Triangle {base: b, hight: h} 
    }
}

impl Area for Triangle<f64> {
    fn area(&self) -> f64 {
        self.base * self.hight / 2.0
    }
}

// Pyramide

struct Pyramid<Type, Hight> 
{
    type_pyramid: Type,
    hight: Hight,

}

impl Pyramid<Square<u32>, f64> {
    fn new( t: Square<u32>, h: f64) -> Self {
        Pyramid { type_pyramid: t, hight: h}
    }
}

impl Pyramid<Triangle<f64>, f64> {
    fn new( t: Triangle<f64>, h: f64) -> Self {
        Pyramid { type_pyramid: t, hight: h}
    }
}

trait Volume {
    fn volume(&self) -> f64;
}

impl Volume for Pyramid<Square<u32>, f64> {
    fn volume(&self) -> f64 {
        self.type_pyramid.area() * self.hight / 3.0
    }
}

impl Volume for Pyramid<Triangle<f64>, f64> {
    fn volume(&self) -> f64 {
        self.type_pyramid.area() * self.hight / 3.0
    }
}


fn main() {
    let square = Square::<u32>::new(5);
    let square_float = Square::<f64>::new(5.4);
    let square_string = Square::<String>::new("6");

    println!("square area is {}", square.area());
    println!("square_float area is {}", square_float.area());
    println!("square_string area is {}", square_string.area());

    let triangle = Triangle::new(14.9, 20.1);
    let pyramid_square = Pyramid::<Square<u32>, f64>::new(square, 24.3);
    let pyramid_triangle = Pyramid::<Triangle<f64>, f64>::new(triangle, 24.3);

    println!("pyramid_square volume is {}", pyramid_square.volume());
    println!("pyramid_triangle volume is {}", pyramid_triangle.volume());
}