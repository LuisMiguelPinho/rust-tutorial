
struct Point(f32, f32, f32);
enum Axis {X, Y, Z, }

fn equals(p1: &Point, p2: &Point) -> bool {
    // TODO
}

fn distance(p1: &Point, p2: &Point) -> f32 {
    ((p1.0 - p2.0).powf(2.0) + (p1.1 - p2.1).powf(2.0) + (p1.2 - p2.2).powf(2.0)).sqrt()
}

fn distances(v: &Vec<Point>) -> f32 {
    // TODO
}

fn reflect(p: &mut Point, a: &Axis){
    // TODO
}

fn reflect_all_points(v: &mut Vec<Point>, a: Axis) {
    // TODO
}

fn add(v: &mut Vec<Point>, p: Point){
    // TODO
}

fn find(v: &Vec<Point>, p: Point) -> bool{
    // TODO
}


fn main() {
    let mut vec: Vec<Point> = vec![];

    add(&mut vec, Point(0.0,0.0,0.0));

    if find(&vec, Point(0.0,0.0,0.0)) {
        println!("Found!");
    }

    add(&mut vec, Point(1.0,1.0,1.0));
    add(&mut vec, Point(2.0,1.0,2.0));
    add(&mut vec, Point(2.0,2.0,2.0));

    println!("Max distance = {}", distances(&vec));

    reflect_all_points(&mut vec, Axis::Y);

    for p in vec {
        println!("Point p = ({}, {}, {})", p.0, p.1, p.2);
    }    
}
