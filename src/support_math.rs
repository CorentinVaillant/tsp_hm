pub type Point = (f64,f64);

pub fn ditsance_point(p1:Point,p2:Point)->f64{
    let dx = p1.0 - p2.0;
    let dy = p1.1 - p2.1;
    f64::sqrt(dx*dx + dy*dy)
}