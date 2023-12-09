use std::ops::Add;

pub trait Iterotar<T> {
    fn next(&mut self) -> Option<T>;
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

#[macro_export]
macro_rules! vec {
    ($( $x:expr ), *) => {
        {
            let mut temp_vec = Vec::new();
            $(temp_vec.push($x);)
            *temp_vec
        }
    };
}

pub trait HelloMacro {
    fn hello_macro();
}
