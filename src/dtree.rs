use std::fs::File;
use std::io::prelude::*;
use rand::prelude::*;
use csv::Error;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use array_tool::vec::Uniq;

mod utils;

#[derive(Debug)]
pub struct Matrix {
  pub data: Vec<f32>,
  pub row: usize,
  pub col: usize
}

#[derive(Debug)]
pub struct BestSplitStruct {
    feature_index: usize,
    threshold: f32,
    dataset_left: Matrix,
    dataset_right: Matrix,
    y_right: Vec<i32>,
    y_left: Vec<i32>,
    info_gain: f32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    feature_index: usize,
    threshold: f32,
    //for leaf Nodes
    value: i32
}

impl Node{
    pub fn new(fi: usize, th: f32, v: i32) -> Self {
        Node {
            left: None,
            right: None,
            feature_index: fi,
            threshold: th,
            value: v
        }
    }
}


#[derive(Debug, Deserialize, Serialize)]
pub struct DecisionTreeClassifier {
    root: Option<Box<Node>>,

    // stopping conditions
    min_samples_split: usize,
    max_depth: usize
}

impl DecisionTreeClassifier  {
    pub fn new(mss: usize, md: usize) -> Self{
        DecisionTreeClassifier {
            root: None,
            min_samples_split: mss,
            max_depth: md
        }
    }

    pub fn build_tree(&mut self, X: &Matrix, Y: &Vec<i32>, curr_depth: usize) -> Option<Box<Node>>{
        let num_samples = X.row;
        let num_features = X.col;

        // allow the tree get deeper as possible

        let depth = std::f32::INFINITY;

        if self.max_depth > 0 {
            let depth = self.max_depth as f32;
        }

        

        if num_samples >= self.min_samples_split && (curr_depth as f32) <= depth {
            let best_split: BestSplitStruct = self.get_best_split(&X, &Y, num_samples, num_features);
            
            //println!("{:?}", best_split);

            if best_split.info_gain > 0.0 {
                
                let left_subtree = DecisionTreeClassifier::build_tree(self, &best_split.dataset_left, &best_split.y_left, curr_depth+1);
                let right_subtree = DecisionTreeClassifier::build_tree(self, &best_split.dataset_right, &best_split.y_right, curr_depth+1);
                
                //println!("{:?}", left_subtree);

                return Some(Box::new(Node {
                    left: left_subtree,
                    right: right_subtree,
                    feature_index: best_split.feature_index,
                    threshold: best_split.threshold,
                    value: 0
                }));
            }

        }
        let leaf_value: i32 = self.calculate_leaf_value(&Y);

        return Some(Box::new(Node {
            left: None,
            right: None,
            feature_index: 0,
            threshold: 0.0,
            value: leaf_value
        }));
    }

    pub fn get_best_split(&mut self, X: &Matrix, Y:  &Vec<i32>, num_samples: usize, num_features: usize) -> BestSplitStruct{
        let mut best_split = BestSplitStruct {
            feature_index: 0,
            threshold: 0.0,
            dataset_left: Matrix {data: vec![], row: 0, col: 0},
            dataset_right: Matrix {data: vec![], row: 0, col: 0},
            y_left: vec![],
            y_right: vec![],
            info_gain: 0.0
        };


        let mut max_info_gain = -std::f32::INFINITY;
        

        for feature_index in 0..num_features {
            
            let feature_values: Vec<f32> = utils::get_column(&X, feature_index);
            
            let possible_thresholds = utils::unique_vals_f32(&feature_values);
            //let possible_thresholds = &feature_values.iter().fold(vec![], |mut vect, x| {if !vect.contains(x) {vect.push(*x);} vect});
            
            //let possible_thresholds = feature_values.unique();
            //println!("{:?}", feature_values);
            
            
            let mut c = 0;

            for &threshold in possible_thresholds.iter() {
                //println!("aaaaaaaaaa -> {:?} - {:?}", feature_index, threshold);
                
                c += 1;
                let dataset_splitted: (Matrix, Matrix, Vec<i32>, Vec<i32>) = self.split(&X, &Y, feature_index, threshold);
                
                let dataset_left: Matrix = dataset_splitted.0;
                let dataset_right: Matrix = dataset_splitted.1;
                
                

                if dataset_left.row > 0 && dataset_right.row > 0 {
                    
                    let y_left: Vec<i32> = dataset_splitted.2;
                    let y_right: Vec<i32> = dataset_splitted.3;
                    
                    let curr_info_gain = self.information_gain(&Y, &y_left, &y_right);
                    
                    if curr_info_gain>max_info_gain {
                        
                        
                        best_split.feature_index = feature_index;
                        best_split.threshold = threshold;
                        best_split.dataset_left = dataset_left;
                        best_split.dataset_right = dataset_right;
                        best_split.info_gain = curr_info_gain;
                        best_split.y_left = y_left;
                        best_split.y_right = y_right;
                        max_info_gain = curr_info_gain;
                    }
                }

                //println!("-------");
                //println!("a--->{:?}", best_split.feature_index);
                //println!("a--->{:?}", best_split.threshold);
                //println!("a--->{:?}", best_split.info_gain);

            }

        }

        
        //println!("{:?}", best_split);
        best_split
    }

    pub fn gini_index(&mut self, Y: &Vec<i32>) -> f32 {
        let now = Instant::now();
        //let class_labels = utils::find_unique_values(&Y);
        //let class_labels = Y.unique();
        let class_labels = &Y.iter().fold(vec![], |mut vect, x| {if !vect.contains(x) {vect.push(*x);} vect});
        let now2 = Instant::now();
        //println!("unique_vals,{:?},{:?}", now2.duration_since(now), Y.len());
        
        let now = Instant::now();
        let mut gini = 0.0;
        
        for cls in class_labels {
            
            
            let p_cls: f32 = ((utils::count_vals(&Y, &cls)  as f32) / (Y.len() as i32) as f32) as f32;
            //let p_cls: f32 = ((utils::count_value_occurrences(&Y, &cls)  as f32) / (Y.len() as i32) as f32) as f32;
            
            
            gini = gini + (p_cls * p_cls);

        }
        let now2 = Instant::now();
        //println!("Gini,{:?},{:?}", now2.duration_since(now), Y.len());
        (1.0 - gini)
        //println!("gini: {}", gini);
    }

    /*
    pub fn split(&mut self, X: &Vec<Vec<f32>>, Y: &Vec<i32>, feature_index: usize, threshold: f32) -> (Vec<Vec<f32>>, Vec<Vec<f32>>, Vec<i32>, Vec<i32>){
        let mut dataset_left: Vec<Vec<f32>> = vec![];
        let mut dataset_right: Vec<Vec<f32>> = vec![];
        let mut y_right: Vec<i32> = vec![];
        let mut y_left: Vec<i32> = vec![];

        //println!("{:?} -- {:?}", X, Y);
        //println!("{}", Y.len());
        let n_rows = X.len();
        for i in 0..n_rows {
            let v: &Vec<f32> = &X[i];
            let v_y: &i32 = &Y[i];
            if v[feature_index] <= threshold {
                dataset_left.push(v.to_vec());
                y_left.push(*v_y);
            }
            else {
                dataset_right.push(v.to_vec());
                y_right.push(*v_y);
            }
        }
        //println!("{:?} -- {:?}", dataset_left, dataset_right);
        (dataset_left, dataset_right, y_left, y_right)
    }
    */

    pub fn split(&mut self, X: &Matrix, Y: &Vec<i32>, feature_index: usize, threshold: f32) -> (Matrix, Matrix, Vec<i32>, Vec<i32>){
  
        let mut vec_left = vec![];
        let mut vec_right = vec![];
        
        let mut y_right: Vec<i32> = vec![];
        let mut y_left: Vec<i32> = vec![];
      
        let n_cols = X.col;
        let n_rows = X.row;
      
        let mut row_left = 0;
        let mut row_right = 0;
        
        for i in 0..n_rows{
            //println!("{} - {} - {}", i, (i*n_cols), (i*n_cols + n_cols)-1);
            let mut v = &(X.data[(i*n_cols)..(i*n_cols + n_cols)]);

            let v_y: &i32 = &Y[i];
            if v[feature_index] <= threshold {
              vec_left.extend(v.iter());
              y_left.push(*v_y);
              row_left += 1;
            }
            else {
              vec_right.extend(v.iter());
              y_right.push(*v_y);
              row_right += 1;
            }
        }
      
        let mut dataset_left: Matrix = Matrix { data: vec_left, row: row_left, col: n_cols};
        let mut dataset_right: Matrix = Matrix { data: vec_right, row: row_right, col: n_cols};
      
        //println!("{:?} -- {:?}", dataset_left, dataset_right);
        (dataset_left, dataset_right, y_left, y_right)
      
      }

    pub fn information_gain(&mut self, parent: &Vec<i32>, l_child: &Vec<i32>, r_child: &Vec<i32>) -> f32{
        
        let weight_l: f32 = (l_child.len() as f32 / parent.len() as f32) as f32;
        let weight_r: f32 = (r_child.len() as f32 / parent.len() as f32) as f32;
        //println!("{:?} -- {:?}", weight_l, weight_r);

        
        let gain: f32 = self.gini_index(parent) - (weight_l*self.gini_index(l_child) + weight_r*self.gini_index(r_child));
        //println!("gini: {}", gain);
        
        gain

    }

    pub fn calculate_leaf_value(&mut self, Y: &Vec<i32> ) -> i32{
        let uni_vals: Vec<i32> = utils::unique_vals(&Y);
        //let uni_vals = &Y.iter().fold(vec![], |mut vect, x| {if !vect.contains(x) {vect.push(*x);} vect});
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

    pub fn fit(&mut self, X: &Matrix, Y: &Vec<i32>) {
        self.root = self.build_tree(&X, &Y, 0);
    }

    pub fn make_prediction(&self, X: &Vec<f32>, tree: &Option<Box<Node>>) -> i32{
        
        if tree.as_ref().unwrap().value != 0 {
            return tree.as_ref().unwrap().value;
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

    pub fn predict(&self, X: &Vec<Vec<f32>>) -> Vec<i32> {
        let mut predictions: Vec<i32> = vec![];


        for i in 0..X.len(){
            let pred: i32 = self.make_prediction(&X[i], &self.root);
            predictions.push(pred);
        }

        return predictions;

     
    }

}

