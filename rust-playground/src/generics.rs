
struct Point<T> {
  x: T,
  y: T
}

struct Line<T> {
  start: Point<T>,
  end: Point<T>,
}

fn generics() {
  let a:Point<f64> = Point { x: 0.0, y: 0.0 };
  let b:Point<f64> = Point { x: 1.2, y: 3.4 };

  let myLine = Line {
    start: a,
    end: b
  };
}

fn main() {
  generics();
}
