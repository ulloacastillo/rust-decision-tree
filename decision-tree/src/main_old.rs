use std::fs::File;
use std::io::prelude::*;


use csv::Error;

struct BestSplitStruct <'d> {
    feature_index: usize,
    threshold: f32,
    dataset_left: Vec<Vec<f32>>,
    dataset_right: Vec<Vec<f32>>,
    y_right: Vec<&'d str>,
    y_left: Vec<&'d str>,
    info_gain: f32
}

#[derive(Debug)]
struct Node <'a>{
    left: Option<Box<Node<'a>>>,
    right: Option<Box<Node<'a>>>,
    feature_index: usize,
    threshold: f32,
    //for leaf Nodes
    value: &'a str
}

impl <'a> Node <'a>{
    pub fn new(fi: usize, th: f32, v: &'a str) -> Self {
        Node {
            left: None,
            right: None,
            feature_index: fi,
            threshold: th,
            value: v
        }
    }
}

impl <'g> From<Node <'g>> for Option<Box<Node <'g>>> {
    fn from(node: Node<'g>) -> Self {
        Some(Box::new(node))
    }
}

struct DecisionTreeClassifier <'a>{
    root: Option<Box<Node<'a>>>,

    // stopping conditions
    min_samples_split: usize,
    max_depth: usize
}

impl <'a> DecisionTreeClassifier <'a> {
    pub fn new(&mut self, mss: usize, md: usize) -> Self{
        DecisionTreeClassifier {
            root: None,
            min_samples_split: mss,
            max_depth: md
        }
    }

    pub fn build_tree<'h>(&mut self, X: &Vec<Vec<f32>>, Y: &'h Vec<&str>, curr_depth: usize) -> Option<Box<Node<'h>>>{
        let num_samples = X.len();
        let num_features = X[0].len();

        if num_samples >= self.min_samples_split && curr_depth <= self.max_depth {
            let best_split: BestSplitStruct = self.get_best_split(X, Y, num_samples, num_features);
            if best_split.info_gain > 0.0 {
                let left_subtree = DecisionTreeClassifier::build_tree(self, &best_split.dataset_left, &best_split.y_left, curr_depth+1);
                let right_subtree = DecisionTreeClassifier::build_tree(self, &best_split.dataset_right, &best_split.y_right, curr_depth+1);
                
                return Some(Box::new(Node {
                    left: left_subtree,
                    right: right_subtree,
                    feature_index: best_split.feature_index,
                    threshold: best_split.threshold,
                    value: ""
                }));
            }

        }
        let leaf_value: &str = self.calculate_leaf_value(Y);

        return Some(Box::new(Node {
            left: None,
            right: None,
            feature_index: 0,
            threshold: 0.0,
            value: leaf_value
        }));
    }

    pub fn get_best_split<'f>(&mut self, X: &Vec<Vec<f32>>, Y: &'f Vec<&'f str>, num_samples: usize, num_features: usize) -> BestSplitStruct<'f>{
        let mut best_split = BestSplitStruct {
            feature_index: 0,
            threshold: 0.0,
            dataset_left: vec![],
            dataset_right: vec![],
            y_left: vec![],
            y_right: vec![],
            info_gain: 0.0
        };

        let mut max_info_gain = -std::f32::INFINITY;

        for feature_index in 0..num_features {
            let feature_values: Vec<f32> = get_column(X, feature_index);
            let possible_thresholds = unique_vals_f32(&feature_values);

            for &threshold in possible_thresholds.iter() {
                let dataset_splitted: (Vec<Vec<f32>>, Vec<Vec<f32>>, Vec<&str>, Vec<&str>) = self.split(X, Y, feature_index, threshold);
                let dataset_left: Vec<Vec<f32>> = dataset_splitted.0;
                let dataset_right: Vec<Vec<f32>> = dataset_splitted.1;
                
                if dataset_left.len() > 0 && dataset_right.len() > 0 {
                    let y_left: Vec<&str> = dataset_splitted.2;
                    let y_right: Vec<&str> = dataset_splitted.3;

                    let curr_info_gain = self.information_gain(Y, &y_left, &y_right);

                    if curr_info_gain>max_info_gain {
                        max_info_gain = curr_info_gain;
                        best_split.feature_index = feature_index;
                        best_split.threshold = threshold;
                        best_split.dataset_left = dataset_left;
                        best_split.dataset_right = dataset_right;
                        best_split.info_gain = curr_info_gain;
                        best_split.y_left = y_left;
                        best_split.y_right = y_right;
                    }
                }
            }
        }
        best_split
    }

    pub fn gini_index(&mut self, Y: &Vec<&str>) -> f32 {
        let class_labels = unique_vals(Y);
        let mut gini = 0.0;
        
        for cls in class_labels.iter() {
            let p_cls: f32 = (count_vals(&Y, &cls) / Y.len()) as f32;
            gini = gini + (p_cls * p_cls);

        }
        gini
    }

    pub fn split<'b>(&mut self, X: &Vec<Vec<f32>>, Y: &'b Vec<&str>, feature_index: usize, threshold: f32) -> (Vec<Vec<f32>>, Vec<Vec<f32>>, Vec<&'b str>, Vec<&'b str>){
        let mut dataset_left: Vec<Vec<f32>> = vec![];
        let mut dataset_right: Vec<Vec<f32>> = vec![];
        let mut y_right: Vec<&str> = vec![];
        let mut y_left: Vec<&str> = vec![];


        for i in 0..X.len() {
            let v: Vec<f32> = X[i].to_vec();
            let v_y: &str = Y[i];
            if v[feature_index] <= threshold {
                dataset_left.push(v);
                y_left.push(v_y);

            }
            else {
                dataset_right.push(v);
                y_right.push(v_y);
            }
        }

        (dataset_left, dataset_right, y_left, y_right)

    }

    pub fn information_gain(&mut self, parent: &Vec<&str>, l_child: &Vec<&str>, r_child: &Vec<&str>) -> f32{
        let weight_l: f32 = (l_child.len() / parent.len()) as f32;
        let weight_r: f32 = (r_child.len() / parent.len()) as f32;

        let gain: f32 = self.gini_index(parent) - (weight_l*self.gini_index(l_child) + weight_r*self.gini_index(r_child));

        gain

    }

    pub fn calculate_leaf_value<'c>(&mut self, Y: &'c Vec<&str> ) -> &'c str{
        let uni_vals: Vec<&str> = unique_vals(Y);
        let mut counts: Vec<usize> = vec![0; uni_vals.len()];

        for i in 0..uni_vals.len() {
            for j in 0..Y.len() {
                if uni_vals[i] == Y[j] {
                    counts[i] = counts[i] + 1;
                }
            }
        }
        
        let mut max_idx = 0;
        let mut max_count = 0;

        for i in 0..counts.len() {
            if counts[i] > max_count {
                max_count = counts[i];
                max_idx = i;
            }
        }
        uni_vals[max_idx]
    }

}

fn unique_vals<'a>(arr: &'a Vec<&str>) -> Vec<&'a str> {
    let mut u_vals: Vec<&str> = vec![];
    for el in arr.iter() {
        if !u_vals.contains(&el) {
            u_vals.push(el);
        }
    }
    u_vals
}

fn unique_vals_f32<'a>(arr: &'a Vec<f32>) -> Vec<f32> {
    let mut u_vals: Vec<f32> = vec![];
    for el in arr.iter() {
        if !u_vals.contains(&el) {
            u_vals.push(*el);
        }
    }
    u_vals
}

fn count_vals(arr: &Vec<&str>, label: &str) -> usize {
    let mut c = 0;
    for el in arr.iter() {
        if el == &label {
            c = c + 1;
        }
    }
    c
}

fn get_column(matrix: &Vec<Vec<f32>>, col: usize) -> Vec<f32>{
    let mut column: Vec<f32> = vec![];
    for row in matrix.iter() {
        for (j, &colu) in row.iter().enumerate() {
            if j == col {
                println!("{} {}", j, col);
                column.push(colu);
            }
        }
    }
    column
}

/*impl From<Node <'_>> for Option<Box<Node <'_>>> {
    fn from(node: Node<'_>) -> Self {
        Some(Box::new(node))
    }
}*/

fn main() -> Result<(), Error> {
    println!("Hello, world!");
    let mut file = File::open("./iris.csv").expect("No se pudo abrir el arcivo");

    let mut content = String::new();

    file.read_to_string(&mut content).expect("No se pudo leer el archivo");


    //println!("{}", content);


    let mut _reader = csv::Reader::from_reader(content.as_bytes());
    //let mut y = Vec::new();




    // Call split, and use collect() to get a string Vec.
    let values: Vec<&str> = content.split('\n').collect();
    let mut y: Vec<&str> = vec![];
    let mut x: Vec<Vec<f32>> = vec![];

    // Pretty-print the results.
    let _xs: Vec<&str> = values[0].split(',').collect();
    

    for _row in values.iter() {
        let xs: Vec<&str> = _row.split(',').collect();
        
        let mut aux: Vec<f32> = vec![];
        for i in 0..xs.len()-1{
            aux.push(xs[i].parse::<f32>().unwrap());
        }
        y.push(xs[xs.len() -1]);
        x.push(aux);
        

    }

    for _row in x.iter(){
        //println!("VALUES: {:?}", _row);
        continue;
    }
    
    let a = 4;
    let b = 2.0;

    let n = Node::new(a, b, "xd");
    println!("VALUES: {:?}", n);
    
    let aa_ = get_column(&x, 1);

    println!("VALUES: {:?}", aa_);

    Ok(())
    
}

