use crate::svo_trait::*;

#[derive(Debug, Clone)]
pub struct Point(pub Vec<Box<[f64; 3]>>);

impl SVONode for Point{
    type Row = Box<[f64; 3]>;

    fn bounde_get(&self)->[[f64; 3]; 2] {
        let mut min_corner = [std::f64::INFINITY; 3];
        let mut max_corner = [std::f64::NEG_INFINITY; 3];
        let points = self.0.clone();
        if points.len() < 2{
            println!("too few points");
        }
        for point in &points{
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
        [min_corner, max_corner]
    }

    fn entities_distribute(&self, bounds: [[f64; 3]; 2])->Vec<Self> {
        let mid = compute_mid_point(bounds[0], bounds[1]);
        let mut child_list = vec![Point(Vec::new()); 8];
        let points_cloud = self.0.clone();
        for point in points_cloud{
            let mut index = 0;
            if point[0] > mid[0]{
                index += 4;
            }
            if point[1] > mid[1]{
                index += 2;
            }
            if point[2] > mid[2]{
                index += 1;
            }      
            child_list[index].0.push(point.clone());
        }
        child_list
    }

    fn entity_position(&self, bounds: [[f64; 3]; 2])->Vec<NodeBoundary>{
        let mid = compute_mid_point(bounds[0], bounds[1]);
        let mut index = 0;
        let mut min = bounds[0];
        let mut max = mid;
        if self.0[0][0] > mid[0]{
            index += 4;
            min[0] = mid[0];
            max[0] = bounds[1][0];
        }
        if self.0[0][1] > mid[1]{
            index += 2;
            min[1] = mid[1];
            max[1] = bounds[1][1];
        }
        if self.0[0][2] > mid[2]{
            index += 1;
            min[2] = mid[2];
            max[2] = bounds[1][2];
        }
        vec![NodeBoundary{index, min, max}]
    }
    
    fn if_leaf(&self)->bool {
        if self.0.len()<= 5{ //single leaf node's element upper limit
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
        for i in 0..3{
            let mut bound = bounds;
            if self.0[0][i] < bounds[0][i]{
                bound[0][i] = self.0[0][i];
                if_change = true;
            }
            if self.0[0][i] > bounds[1][i]{
                bound[1][i] = self.0[0][i];
                if_change = true;
            }
            min = bound[0];
            max = bound[1];
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
        self.0[0].clone()
    }
    
}
