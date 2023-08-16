#[macro_export]
macro_rules! sub {
    ($a:expr, $b:expr) => {
        {
            $a - $b
        }
    };
    ($a:expr) => {
        $a + 100
    }
}
fn main() {
    let a = 100;
    let b = 22;
    println!("{}",  sub!(a,b));
    println!("{}",  sub!(a));
}
