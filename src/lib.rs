pub mod lines;
use lines::*;
pub mod points;
use points::*;
pub mod polygon;
use polygon::*;
pub mod svo_trait;
use svo_trait::*;
pub mod triangles;
use triangles::*;

use rand::Rng;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]  
    fn point_test(){
        let points = Point(generate_points(20));
        let mut point_root = SVO::new_root(points);
        println!("{:?}", point_root.leaf_node_index);
        point_root.insert_entity(Point(vec![Box::new([5.0; 3])]));
    }
   
    #[test]
    fn line_test(){
        let lines = Line(generate_lines(20));
        let mut line_root = SVO::new_root(lines);
        println!("{:?}", line_root.leaf_node_index);
        println!("root lod {:?}", line_root.lod_info.unwrap());
        line_root.search_bound([[5.0; 3], [20.0; 3]]);
        line_root.insert_entity(Line(vec![Box::new([[0.0; 3], [7.0; 3]])]));
        println!("new root lod {:?}", line_root.lod_info.unwrap()); 
    }
    #[test]
    fn triangles_test(){
        let triangles = Triangle(generate_triangles(20));
        let triangle_root = SVO::new_root(triangles);
        println!("{:?}", triangle_root.leaf_node_index);
    }
}


pub fn generate_points(count: usize) -> Vec<Box<[f64; 3]>> {
    let mut rng = rand::thread_rng();
    let mut points = Vec::with_capacity(count);
    for _ in 0..count{
        let point = Box::new([rng.gen_range(0.0..100.0), rng.gen_range(0.0..100.0), rng.gen_range(0.0..100.0)]);
        points.push(point);
    }
    points
}

pub fn generate_lines(count: usize) -> Vec<Box<[[f64; 3]; 2]>> {
    let mut rng = rand::thread_rng();
    let mut lines = Vec::with_capacity(count);
    for _ in 0..count{
        let point_ori = [rng.gen_range(0.0..100.0), rng.gen_range(0.0..100.0), rng.gen_range(0.0..100.0)];
        let line = Box::new([
            point_ori, 
            [rng.gen_range(point_ori[0] - 10.0..point_ori[0] + 10.0), rng.gen_range(point_ori[1] - 10.0..point_ori[1] + 10.0), rng.gen_range(point_ori[2] - 10.0..point_ori[2] + 10.0)]
            ]);
        lines.push(line);
    }
    lines
}

pub fn generate_triangles(count: usize) -> Vec<Box<[[f64; 3]; 3]>> {
    let mut rng = rand::thread_rng();
    let mut triangles = Vec::with_capacity(count);
    for _ in 0..count{
        let point_ori = [rng.gen_range(0.0..100.0), rng.gen_range(0.0..100.0), rng.gen_range(0.0..100.0)];
        let triangle = Box::new([
            point_ori, 
            [rng.gen_range(point_ori[0] - 10.0..point_ori[0] + 10.0), rng.gen_range(point_ori[1] - 10.0..point_ori[1] + 10.0), point_ori[2]],
            [rng.gen_range(point_ori[0] - 10.0..point_ori[0] + 10.0), rng.gen_range(point_ori[1] - 10.0..point_ori[1] + 10.0), point_ori[2]]
        ]);
        triangles.push(triangle);
    }
    triangles
}

