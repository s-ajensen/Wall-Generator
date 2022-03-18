use std::cmp;
use std::process;
use crate::point;

pub fn line(p1: &point::Point, p2: &point::Point) -> Vec<point::Point> {
    let mut points : Vec<point::Point> = vec![];
    let segments = distance(p1, p2);
    for i in 0..segments {
        let t =  if segments == 0 {0.0} else {i as f64 / segments as f64};
        points.push(lerp_point(p1, p2, t));
    }
    return points;
}

pub fn lerp(start: usize, end: usize, t: f64) -> f64 {
    if t < 0.0 {
        println!("Lerp parameter cannot be negative! Aborting...");
        process::abort();
    }

    if t > 1.0 {
        println!("Lerp parameter cannot be greater than 1.0! Aborting...");
        process::abort();
    }

    return start as f64 + (t * (end as f64 - start as f64));
}

fn lerp_point(p1: &point::Point, p2: &point::Point, t: f64) -> point::Point {
    let x = lerp(p1.x, p2.x, t) as usize;
    let y = lerp(p1.y, p2.y, t) as usize;
    return point::Point::new(x, y);
}

fn distance(p1: &point::Point, p2: &point::Point) -> usize {
    let dx = (p1.x as i64 - p2.x as i64).abs() as usize;
    let dy = (p1.y as i64 - p2.y as i64).abs() as usize;
    return cmp::max(dx, dy);
}