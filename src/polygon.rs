use crate::svo_trait::*;
use std::collections::HashSet;

//not settle yet

#[derive(Debug, Clone)]
pub struct Polygon(pub Vec<Box<Poly>>);

impl SVONode for Polygon{
    type Row = Box<Vec<[f64; 3]>>;
    fn bounde_get(&self)->[[f64; 3]; 2] {
        let mut min_corner = [std::f64::INFINITY; 3];
        let mut max_corner = [std::f64::NEG_INFINITY; 3];
        let rectangles = self.0.clone();
        for rectangle in &rectangles{
            for inde in 0..2{
                let point = rectangle.edge[inde];
                if point[0] > max_corner[0]{
                    max_corner[0] = point[0];
                }
                if point[1] > max_corner[1]{
                    max_corner[1] = point[1];
                }
                if point[2] > max_corner[2]{
                    max_corner[2] = point[2];
                }
                if point[0] < min_corner[0]{
                    min_corner[0] = point[0];
                }
                if point[1] < min_corner[1]{
                    min_corner[1] = point[1];
                }
                if point[2] < min_corner[2]{
                    min_corner[2] = point[2];
                }
            }
        }
        [min_corner, max_corner]
    }

    fn entities_distribute(&self, bounds: [[f64; 3]; 2])->Vec<Self> {
        let mid = compute_mid_point(bounds[0], bounds[1]);
        let mut child_list = vec![Polygon(Vec::new()); 8];
        let rectangle_cloud = self.0.clone();
        for rectangle in rectangle_cloud{
            let mut index_array: HashSet<usize> = HashSet::new();
            let frames = [
                [rectangle.edge[0], rectangle.edge[1]],
                [rectangle.edge[1], rectangle.edge[0]],
            ];
            for i in 0..8{
                let mut min = bounds[0];
                let mut max = mid; 
                let binary_array: [usize; 3] = [
                    (i >> 2) & 1,
                    (i >> 1) & 1,
                    i & 1,     
                ];
                if binary_array[0] == 1 {
                    min[0] = mid[0];
                    max[0] = bounds[1][0];
                }
                if binary_array[1] == 1 {
                    min[1] = mid[1];
                    max[1] = bounds[1][1];
                }
                if binary_array[2] == 1 {
                    min[2] = mid[2];
                    max[2] = bounds[1][2];
                }
                for frame in frames{
                    if line_intersect(frame, min, max){
                        index_array.insert(i);
                    }
                }
                
            }
            for index in index_array{
                child_list[index].0.push(rectangle.clone());
            }           
        }
        child_list
    }

    fn entity_position(&self, bounds: [[f64; 3]; 2])->Vec<NodeBoundary>{
        let mid = compute_mid_point(bounds[0], bounds[1]);
        let mut node: Vec<NodeBoundary> = Vec::new();
        let mut index_array: HashSet<usize> = HashSet::new();
        let rectangle = self.0[0].edge;
        let frames = [
                [rectangle[0], rectangle[1]],
                [rectangle[1], rectangle[0]],
            ];
        for frame in frames{
            for i in 0..8{
                let mut min = bounds[0];
                let mut max = mid; 
                let len = index_array.len();
                let binary_array: [usize; 3] = [
                    (i >> 2) & 1,
                    (i >> 1) & 1,
                    i & 1,     
                ];
                if binary_array[0] == 1 {
                    min[0] = mid[0];
                    max[0] = bounds[1][0];
                }
                if binary_array[1] == 1 {
                    min[1] = mid[1];
                    max[1] = bounds[1][1];
                }
                if binary_array[2] == 1 {
                    min[2] = mid[2];
                    max[2] = bounds[1][2];
                }
                if line_intersect(frame, min, max){
                    index_array.insert(i);
                    if index_array.len() != len{
                        node.push(NodeBoundary{index: i, min, max});
                    }
                }
            }
        }
        node
    }

    fn if_leaf(&self)->bool {
        if self.0.len()<= 5{
            return true;
        }
        false
    }

    fn is_empty(&self)->bool {
        self.0.is_empty()
    }

    fn contain(&self, element: &Self)->bool {
        self.0.contains(&element.0[0])
    }

    fn if_cross(&self, bounds: [[f64; 3]; 2])->(bool, [[f64; 3]; 2]) {
        let mut if_change = false;
        let mut min = Default::default();
        let mut max = Default::default();
        for point in self.0[0].edge{
            for i in 0..3{
                let mut bound = bounds;
                if point[i] < bounds[0][i]{
                    bound[0][i] = point[i];
                    if_change = true;
                }
                if point[i] > bounds[1][i]{
                    bound[1][i] = point[i];
                    if_change = true;
                }
                min = bound[0];
                max = bound[1];
            }
        }
        (if_change, [min, max])
    }

    fn push(&mut self, element: Self) {
        self.0.push(element.0[0].clone());
    }

    fn len(&self)->usize {
        self.0.len()
    }
    fn retain(&mut self, element: &Self) {
        self.0.retain(|x| **x != *element.0[0]);
    }
    fn row(&self)->Self::Row{
        self.0[0].vertex.clone()
    }
}

impl Polygon{
    fn new(polygons: Vec<Box<Vec<[f64; 3]>>>)->Self{
        let mut final_list = Vec::new();
        for polygon in polygons{
            let mut min_corner = [std::f64::INFINITY; 3];
            let mut max_corner = [std::f64::NEG_INFINITY; 3];
            for index in 0..polygon.len(){
                if polygon[index][0] > max_corner[0]{
                    max_corner[0] = polygon[index][0];
                }
                if polygon[index][1] > max_corner[1]{
                    max_corner[1] = polygon[index][1];
                }
                if polygon[index][2] > max_corner[2]{
                    max_corner[2] = polygon[index][2];
                }
                if polygon[index][0] < min_corner[0]{
                    min_corner[0] = polygon[index][0];
                }
                if polygon[index][1] < min_corner[1]{
                    min_corner[1] = polygon[index][1];
                }
                if polygon[index][2] < min_corner[2]{
                    min_corner[2] = polygon[index][2];
                }
            } 
            final_list.push(Box::new(Poly{vertex: polygon, edge:[min_corner, max_corner]}));
        }
        Polygon(final_list)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Poly{
    vertex: Box<Vec<[f64; 3]>>,
    edge: [[f64; 3]; 2]
}