use std::fmt::Debug;

#[allow(unused)]
pub trait SVONode: Clone + Debug {
    type Row;
    fn bounde_get(&self)->[[f64; 3]; 2];
    fn entities_distribute(&self, bounds: [[f64; 3]; 2])->Vec<Self>;
    fn entity_position(&self, bounds: [[f64; 3]; 2])->Vec<NodeBoundary>;
    fn if_leaf(&self)->bool;
    fn is_empty(&self)->bool;
    fn contain(&self, element: &Self)->bool;
    fn if_cross(&self, bounds: [[f64; 3]; 2])->(bool, [[f64; 3]; 2]);
    fn push(&mut self, element: Self);
    fn len(&self)->usize;
    fn row(&self)->Self::Row;
    fn retain(&mut self, element: &Self);
}

pub struct SVO<T> {
    pub is_leaf: bool,
    pub index: Vec<u8>,
    pub entities: T,
    pub children: [Option<Box<SVO<T>>>; 8],
    pub bound: Option<[[f64; 3]; 2]>,
    pub leaf_node_index: Option<Vec<Vec<u8>>>,
    pub depth: Option<u32>,
    pub lod_info: Option<i32>,
}

impl<T: SVONode> SVO<T>{
    pub fn new_root(entities: T) -> Box<Self>{
        let bounds = entities.bounde_get();
        println!("min: {:?}, max: {:?}",bounds[0], bounds[1]);
        let mut root = Box::new(
            SVO{
                is_leaf: false,
                index: vec![0],
                entities: entities.clone(),
                children: Default::default(),
                bound: Some(bounds),
                leaf_node_index: None,
                depth: None,
                lod_info: None,
            }
        );
        let mut leaf_node_index: Vec<Vec<u8>> = Vec::new();
        let mut depth = 0;
        root.build_svo(bounds[0], bounds[1], entities, &mut leaf_node_index, &mut depth);
        root.leaf_node_index = Some(leaf_node_index);
        root.depth = Some(depth);
        root.set_lod_info();
        root
    }
}

impl<T: SVONode> SVO<T>{
    pub fn build_svo(
        &mut self, 
        min_corner: [f64; 3], 
        max_corner: [f64; 3], 
        entity_clouds: T, 
        leaf_node_index: &mut Vec<Vec<u8>>,
        depth: &mut u32,
    ){
        let mut index = self.index.clone();
        if entity_clouds.if_leaf() {
            self.is_leaf = true;
            self.entities = entity_clouds;
            self.lod_info = Some(1);
            index.remove(0);
            *depth = std::cmp::max(*depth, index.len() as u32);
            leaf_node_index.push(index);
            self.set_lod_info();
            return;
        }    
        let mid = compute_mid_point(min_corner, max_corner);
        let child_list = entity_clouds.entities_distribute([min_corner, max_corner]);
        for i in 0..8 {
            let (child_min, child_max) = compute_child_bounds(i, min_corner, mid, max_corner);
            if child_list[i].is_empty() {
                self.children[i] = None; 
            } else {
                let mut child_index = index.clone();
                child_index.push(i as u8);
                self.children[i] = Some(Box::new(
                    SVO{
                        is_leaf: false,
                        index: child_index,
                        entities: child_list[i].clone(),
                        children: Default::default(),
                        bound: None,
                        leaf_node_index: None,
                        depth: None,
                        lod_info: None,
                    }
                ));
                self.children[i].as_mut().unwrap().build_svo(child_min, child_max,  child_list[i].clone(), leaf_node_index, depth);
            }
        }
        self.set_lod_info();
    }

    pub fn search_bound(&self, bounds: [[f64; 3]; 2]){
        let [min_corner, max_corner] = self.bound.unwrap().clone();
        if !if_intersect((min_corner, max_corner), bounds){
            println!("bound {:?} doesn't intersect with root", bounds);
            return;
        }
        let mut use_node_index = Default::default();
        if let Some(leaf_node_index) = self.leaf_node_index.clone(){
            use_node_index = leaf_node_index;
            
        }
        for leaf_index in use_node_index{
            self.search(&leaf_index, &leaf_index, bounds, min_corner, max_corner)
        }
    }

    pub fn insert_entity(&mut self, entity: T){
        if self.entities.contain(&entity){
            println!("{:?} element already exist", entity);
            return;
        }
        let mut if_change = false;
        let mut min = Default::default();
        let mut max = Default::default();
        if let Some(bound) = self.bound {
            (if_change, [min, max]) = entity.if_cross(bound);
        }
        let mut use_node_index = Default::default();
        if let Some(leaf_node_index) = self.leaf_node_index.clone(){
            use_node_index = leaf_node_index;
        }
        if if_change{
            let mut new_elist= self.entities.clone();
            new_elist.push(entity);
            self.bound = Some([min, max]);
            let mut leaf_node_index: Vec<Vec<u8>> = Vec::new();
            let mut depth = 0;
            self.build_svo(min, max, new_elist, &mut leaf_node_index, &mut depth);
            self.leaf_node_index = Some(leaf_node_index);
            self.depth = Some(depth);
            println!("crate!");
        }else{
            self.insert(entity, [min, max], &mut use_node_index);
            self.leaf_node_index = Some(use_node_index);
            println!("insert!");
        }
    }

    pub fn point_delete(&mut self, entity: T){
        let length = self.entities.len();
        self.entities.retain(&entity);
        if length == self.entities.len(){
            println!("{:?} element didn't exist", entity);
            return;
        }
        if self.entities.len() == 0{
            println!("root has been empty");
            return;
        }
        let bounds = self.bound.as_mut().unwrap().clone();
        let mut leaf_node_index = self.leaf_node_index.as_mut().unwrap().clone();
        self.delete(&entity, bounds, &mut leaf_node_index);
        self.leaf_node_index = Some(leaf_node_index);
    }

}

impl<T: SVONode> SVO<T>{
    fn search(
        &self,  
        static_index: &Vec<u8>,
        leaf_index: &Vec<u8>,
        bounds: [[f64; 3]; 2],
        min_corner: [f64; 3], 
        max_corner: [f64; 3], 
    ){
        if !if_intersect((min_corner, max_corner), bounds){
            return;
        }
        let mut index = leaf_index.clone();
        if self.is_leaf{
            println!("node {:?} was selected", static_index);
            return;
        }
        let inde = index[0] as usize;
        index.remove(0);
        let mid = compute_mid_point(min_corner, max_corner);
        let mut min = min_corner;
        let mut max = mid;

        if (inde >> 2) & 1 == 1{
            min[0] = mid[0];
            max[0] = max_corner[0];
        }
        if (inde >> 1) & 1 == 1{
            min[1] = mid[1];
            max[1] = max_corner[1];
        }
        if let Some(child) = &self.children[inde] {
            child.search(static_index,&index, bounds, min, max);
        }
    }

    fn insert(&mut self, entity: T, bounds: [[f64; 3];2], leaf_node_index: &mut Vec<Vec<u8>>){
        self.entities.push(entity.clone());
        if self.is_leaf{
            println!("insert into {:?}",self.index);
            self.set_lod_info();
            return;
        }
        let boundary = entity.entity_position(bounds);
        let entity_clo = entity.clone();
        for node in boundary{
            let entity_clone = entity_clo.clone(); 
            match &mut self.children[node.index]{
                Some(child)=>{
                    child.insert(entity_clone, [node.min, node.max], leaf_node_index);
                }
                None=>{
                    let mut inde = self.index.clone();
                    inde.push(node.index as u8);
                    inde.remove(0);
                    println!("crate new node {:?} and insert", inde);
                    self.children[node.index] = Some(Box::new(
                        SVO{
                            is_leaf: false,
                            index: inde,
                            entities: entity_clone.clone(),
                            children: [None, None, None, None, None, None, None, None],
                            bound: None,
                            leaf_node_index: None,
                            depth: None,
                            lod_info: None,
                        }
                    ));
                    let mut depth = 0;
                    self.children[node.index].as_mut().unwrap().build_svo(node.min, node.max, entity_clone, leaf_node_index, &mut depth);
                    self.depth = Some(depth);
                }
            }
        }
        self.set_lod_info();
    }

    fn delete(&mut self, entity: &T, bounds: [[f64; 3];2], leaf_node_index: &mut Vec<Vec<u8>>){
        if self.is_leaf{
            self.set_lod_info();
            return;
        }
        let boundary = entity.entity_position(bounds);
        for node in boundary{
            if self.children[node.index].as_mut().unwrap().entities.len() == 1{
                let child_index = self.children[node.index].as_mut().unwrap().index.clone();
                self.children[node.index] = None;
                leaf_node_index.retain(|item| {
                    !item.iter().zip(&child_index).all(|(&a, &b)| a == b)
                });
                println!("node {:?} has been empty", node.index);
                return;
            }
            self.children[node.index].as_mut().unwrap().delete(&entity, [node.min, node.max], leaf_node_index);
        }
        self.set_lod_info();
    }

    fn set_lod_info(&mut self){
        //leaf node create new img
        if self.is_leaf{
            self.lod_info = Some(1);
            return;
            // todo
        }
        //collect childs' info
        let mut lod = 0;
        for i in 0..8{
            if let Some(child) = &self.children[i] {
                lod += child.lod_info.unwrap();
                
            }
        }
        // todo
        self.lod_info = Some(lod);

    }
}

pub fn if_intersect(
    (min_corner, max_corner): ([f64; 3], [f64; 3]), 
    bound: [[f64; 3];2]
)-> bool{
    if max_corner[0] < bound[0][0] || bound[1][0] < min_corner[0] 
    || max_corner[1] <bound[0][1] || bound[1][1] < min_corner[1] 
    || max_corner[2] <bound[0][2] || bound[1][2] < min_corner[2] {
        false
    } else {
        true
    }
}

pub fn compute_mid_point(min: [f64; 3], max: [f64; 3]) -> [f64; 3]{
    [
        (min[0] + max[0]) / 2.0,
        (min[1] + max[1]) / 2.0,
        (min[2] + max[2]) / 2.0 
    ]
}

pub fn compute_child_bounds(index: usize, min_corner: [f64; 3], mid: [f64; 3], max_corner: [f64; 3]) -> ([f64; 3], [f64; 3]) {
    let mut child_min = [min_corner[0], min_corner[1], min_corner[2]];
    let mut child_max = [mid[0], mid[1], mid[2]];
    let binary_array: [usize; 3] = [
        (index >> 2) & 1,
        (index >> 1) & 1,
        index & 1,     
    ];
    
    if binary_array[0] == 1{
        child_min[0] = mid[0];
        child_max[0] = max_corner[0];
    }

    if binary_array[1] == 1{
        child_min[1] = mid[1];
        child_max[1] = max_corner[1];
    }

    if binary_array[2] == 1{
        child_min[2] = mid[2];
        child_max[2] = max_corner[2];
    }

    (child_min, child_max)
}

pub fn line_intersect(line: [[f64; 3]; 2], min: [f64; 3], max: [f64; 3])->bool{
    let line_start = line[0];
    let line_end = line[1];
    if in_boundary(line_start, min, max) || in_boundary(line_end, min, max){
        true
    }else {
        for i in 0..3{
            if intervals_intersect([line_start[i], line_end[i]], [min[i], max[i]]){
                continue;
            }else {
                return false;
            }
        }
        true
    }
}

fn in_boundary(point: [f64; 3], min: [f64; 3], max: [f64; 3])->bool{
    for i in 0..3{
        if point[i] <= max[i] && point[i] >= min[i]{
            continue;
        }else {
            return false;
        }
    }
    true
}
fn intervals_intersect(line: [f64; 2], bound: [f64; 2])->bool{
    if (line[0] > bound[1] && line[1] > bound[1]) || (line[0] < bound[0] && line[1] < bound[0]){
        return false;
    }else {
        true     
    }
}

#[derive(Debug, Clone)]
pub struct NodeBoundary {
    pub index: usize,
    pub min: [f64; 3],
    pub max: [f64; 3],
}