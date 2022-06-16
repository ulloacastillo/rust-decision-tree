use std::fs::File;
use std::io::prelude::*;
use rand::prelude::*;


use csv::Error;
#[derive(Debug)]
struct BestSplitStruct {
    feature_index: usize,
    threshold: f32,
    dataset_left: Vec<Vec<f32>>,
    dataset_right: Vec<Vec<f32>>,
    y_right: Vec<String>,
    y_left: Vec<String>,
    info_gain: f32
}

#[derive(Debug)]
struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    feature_index: usize,
    threshold: f32,
    //for leaf Nodes
    value: String
}

impl Node{
    pub fn new(fi: usize, th: f32, v: String) -> Self {
        Node {
            left: None,
            right: None,
            feature_index: fi,
            threshold: th,
            value: v
        }
    }
}


#[derive(Debug)]
struct DecisionTreeClassifier {
    root: Option<Box<Node>>,

    // stopping conditions
    min_samples_split: usize,
    max_depth: usize
}

impl  DecisionTreeClassifier  {
    pub fn new(mss: usize, md: usize) -> Self{
        DecisionTreeClassifier {
            root: None,
            min_samples_split: mss,
            max_depth: md
        }
    }

    pub fn build_tree(&mut self, X: &Vec<Vec<f32>>, Y: &Vec<String>, curr_depth: usize) -> Option<Box<Node>>{
        let num_samples = X.len();
        let num_features = X[0].len();

        if num_samples >= self.min_samples_split && curr_depth <= self.max_depth {
            let best_split: BestSplitStruct = self.get_best_split(X, Y, num_samples, num_features);
            println!("{:?}", best_split);
            if best_split.info_gain > 0.0 {
                let left_subtree = DecisionTreeClassifier::build_tree(self, &best_split.dataset_left, &best_split.y_left, curr_depth+1);
                let right_subtree = DecisionTreeClassifier::build_tree(self, &best_split.dataset_right, &best_split.y_right, curr_depth+1);
                
                //println!("{:?}", left_subtree);

                return Some(Box::new(Node {
                    left: left_subtree,
                    right: right_subtree,
                    feature_index: best_split.feature_index,
                    threshold: best_split.threshold,
                    value: "".to_string()
                }));
            }

        }
        let leaf_value: String = self.calculate_leaf_value(Y);

        return Some(Box::new(Node {
            left: None,
            right: None,
            feature_index: 0,
            threshold: 0.0,
            value: leaf_value
        }));
    }

    pub fn get_best_split(&mut self, X: &Vec<Vec<f32>>, Y:  &Vec<String>, num_samples: usize, num_features: usize) -> BestSplitStruct{
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
                let dataset_splitted: (Vec<Vec<f32>>, Vec<Vec<f32>>, Vec<String>, Vec<String>) = self.split(X, Y, feature_index, threshold);
                let dataset_left: Vec<Vec<f32>> = dataset_splitted.0;
                let dataset_right: Vec<Vec<f32>> = dataset_splitted.1;
                
                if dataset_left.len() > 0 && dataset_right.len() > 0 {
                    let y_left: Vec<String> = dataset_splitted.2;
                    let y_right: Vec<String> = dataset_splitted.3;

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
        //println!("{:?}", best_split);
        best_split
    }

    pub fn gini_index(&mut self, Y: &Vec<String>) -> f32 {
        let class_labels = unique_vals(&Y);
        
        let mut gini = 0.0;
        
        for cls in class_labels.iter() {
            
            let p_cls: f32 = (((count_vals(&Y, cls.to_string()) as i32) as f32) / (Y.len() as i32) as f32) as f32;
            
            
            gini = gini + (p_cls * p_cls);

        }
        gini
    }

    pub fn split(&mut self, X: &Vec<Vec<f32>>, Y: &Vec<String>, feature_index: usize, threshold: f32) -> (Vec<Vec<f32>>, Vec<Vec<f32>>, Vec<String>, Vec<String>){
        let mut dataset_left: Vec<Vec<f32>> = vec![];
        let mut dataset_right: Vec<Vec<f32>> = vec![];
        let mut y_right: Vec<String> = vec![];
        let mut y_left: Vec<String> = vec![];


        for i in 0..X.len() {
            let v: Vec<f32> = X[i].to_vec();
            let v_y = &Y[i];
            if v[feature_index] <= threshold {
                dataset_left.push(v);
                y_left.push(v_y.to_string());

            }
            else {
                dataset_right.push(v);
                y_right.push(v_y.to_string());
            }
        }

        (dataset_left, dataset_right, y_left, y_right)

    }

    pub fn information_gain(&mut self, parent: &Vec<String>, l_child: &Vec<String>, r_child: &Vec<String>) -> f32{
        let weight_l: f32 = (l_child.len() / parent.len()) as f32;
        let weight_r: f32 = (r_child.len() / parent.len()) as f32;

        let gain: f32 = self.gini_index(parent) - (weight_l*self.gini_index(l_child) + weight_r*self.gini_index(r_child));
        //println!("gini: {}", self.gini_index(parent));
        gain

    }

    pub fn calculate_leaf_value(&mut self, Y: &Vec<String> ) -> String{
        let uni_vals: Vec<String> = unique_vals(Y);
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
        uni_vals[max_idx].to_string()
    }

    pub fn fit(&mut self, X: &Vec<Vec<f32>>, Y: &Vec<String>) {
        self.root = self.build_tree(&X, &Y, 0);
    }

    pub fn make_prediction(&self, X: &Vec<f32>, tree: &Option<Box<Node>>) -> String{
        
        if tree.as_ref().unwrap().value != "" {
            return tree.as_ref().unwrap().value.to_string();
        }

        

        let idx:usize = tree.as_ref().unwrap().feature_index;
        let feature_val = X[idx];

        if feature_val<= tree.as_ref().unwrap().threshold {
            let sub_tree_l = &tree.as_ref().unwrap().left;
            return self.make_prediction(X, &sub_tree_l);
        }

        else {
            let sub_tree_r = &tree.as_ref().unwrap().right;
            return self.make_prediction(X, &sub_tree_r);
        }
    }

    pub fn predict(&self, X: &Vec<Vec<f32>>) -> Vec<String> {
        let mut predictions: Vec<String> = vec![];


        for i in 0..X.len(){
            let pred: String = self.make_prediction(&X[i], &self.root);
            predictions.push(pred);
        }

        return predictions;

     
    }

}

fn unique_vals(arr: &Vec<String>) -> Vec<String> {
    let mut u_vals: Vec<String> = vec![];
    for el in arr.iter() {
        if !u_vals.contains(&el) {
            u_vals.push(el.to_string());
        }
    }
    u_vals
}

fn unique_vals_f32(arr: &Vec<f32>) -> Vec<f32> {
    let mut u_vals: Vec<f32> = vec![];
    for el in arr.iter() {
        if !u_vals.contains(&el) {
            u_vals.push(*el);
        }
    }
    u_vals
}

fn count_vals(arr: &Vec<String>, label: String) -> usize {
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
                //println!("{} {}", j, col);
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


fn split_dataset(X: &mut Vec<Vec<f32>>, Y: &mut Vec<String>, train_size: f32) -> (Vec<Vec<f32>>,  Vec<String>, Vec<Vec<f32>>,Vec<String>) {
    let n_train = (X.len() as f32 * train_size) as usize;
    let n_test = X.len() - n_train;
    
    let mut X_test: Vec<Vec<f32>> = vec![];
    let mut X_train: Vec<Vec<f32>> = vec![];
    let mut Y_test: Vec<String> = vec![];
    let mut Y_train: Vec<String> = vec![];
    let mut idxs: Vec<usize> = (0..X.len()).collect();


    let mut rng = rand::thread_rng();

    idxs.shuffle(&mut rng);

    
    
    for (i, &idx) in idxs.iter().enumerate(){
        let xx: Vec<f32> = X[idx].clone();
        let yy: String = Y[idx].clone();
        
        if i < n_train {
            X_train.push(xx);
            Y_train.push(yy);
        }
        else {
            X_test.push(xx);
            Y_test.push(yy);
        }

        
    }
    return (X_train, Y_train, X_test, Y_test);

}

fn main() -> Result<(), Error> {
    println!("Hello, world!");
    let mut file = File::open("./iris.csv").expect("No se pudo abrir el arcivo");

    let mut content = String::new();

    file.read_to_string(&mut content).expect("No se pudo leer el archivo");


    //println!("{}", content);


    let mut _reader = csv::Reader::from_reader(content.as_bytes());
    //let mut y = Vec::new();




    // Call split, and use collect() to get a string Vec.
    //let strings: Vec<String> = [ "a", "b", "c" ].iter().map(|&s|s.into()).collect();
    /*let values: Vec<String> = content.split('\n').collect();
    let mut y: Vec<String> = vec![];
    let mut x: Vec<Vec<f32>> = vec![];

    // Pretty-print the results.
    let _xs: Vec<String> = values[0].split(',').collect();
    

    for _row in values.iter() {
        let xs: Vec<String> = _row.split(',').collect();
        
        let mut aux: Vec<f32> = vec![];
        for i in 0..xs.len()-1{
            aux.push(xs[i].parse::<f32>().unwrap());
        }
        y.push(xs[xs.len() -1]);
        x.push(aux);
    }
    */

    let values: Vec<String> = content.split('\n').map(str::to_string).collect();
    let mut y: Vec<String> = vec![];
    let mut x: Vec<Vec<f32>> = vec![];

    // Pretty-print the results.
    let _xs: Vec<String> = values[0].split(',').map(str::to_string).collect();
    

    for _row in values.iter() {
        let xs: Vec<String> = _row.split(',').map(str::to_string).collect();
        
        let mut aux: Vec<f32> = vec![];
        for i in 0..&xs.len()-1{
            aux.push(xs[i].parse::<f32>().unwrap());
        }
        let a = &xs.len() -1;
        let b = &xs[a];
        &y.push(b.to_string());
        &x.push(aux);
    }



    //let aa_ = get_column(&x, 1);

    //let mut tree_classifier = DecisionTreeClassifier::new(3, 3);

    //tree_classifier.fit(&x, &y);

    
    //let x_i = vec![5.1,3.5,1.4,0.2];

    //let X_test: Vec<Vec<f32>> = vec![vec![5.0,3.4,1.5,0.2], vec![4.4,2.9,1.4,0.2], vec![4.9,3.1,1.5,0.1], vec![5.6,2.9,3.6,1.3], vec![6.2,2.9,4.3,1.3], vec![6.1,2.6,5.6,1.4], vec![7.7,3.0,6.1,2.3]];

    //let predictions = tree_classifier.predict(&X_test);

    //let prediction = tree_classifier.make_prediction(&x_i, &tree_classifier.root);

    //println!("predictions: {:?}", predictions);

    let (X_train, Y_train, X_test, Y_test) = split_dataset(&mut x, &mut y, 0.25);

    let mut tree_classifier = DecisionTreeClassifier::new(5,5);
    tree_classifier.fit(&X_train, &Y_train);

    let predictions = tree_classifier.predict(&X_test);
    println!("predictions: {:?}", predictions);
    println!("predictions: {:?}", Y_test);




    Ok(())
    
}

