use rand::prelude::*;

use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use std::io::prelude::*;
use std::time::Instant;

mod utils;

#[derive(Debug)]
pub struct Matrix {
    pub data: Vec<f32>,
    pub row: usize,
    pub col: usize,
}

pub struct MatrixArray {
    pub row: usize,
    pub col: usize,
    pub data: [f32],
}

#[derive(Debug)]
pub struct BestSplitStruct {
    feature_index: usize,
    threshold: f32,
    dataset_left: Matrix,
    dataset_right: Matrix,
    y_right: Vec<i32>,
    y_left: Vec<i32>,
    info_gain: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    feature_index: usize,
    threshold: f32,
    //for leaf Nodes
    value: i32,
}

impl Node {
    pub fn new(fi: usize, th: f32, v: i32) -> Self {
        Node {
            left: None,
            right: None,
            feature_index: fi,
            threshold: th,
            value: v,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DecisionTreeClassifier {
    root: Option<Box<Node>>,

    // stopping conditions
    min_samples_split: usize,
    max_depth: usize,
}

impl DecisionTreeClassifier {
    pub fn new(mss: usize, md: usize) -> Self {
        DecisionTreeClassifier {
            root: None,
            min_samples_split: mss,
            max_depth: md,
        }
    }

    pub fn build_tree(
        &mut self,
        X: &Matrix,
        Y: &Vec<i32>,
        ps: &HashMap<usize, (f32, f32, Vec<f32>)>,
        curr_depth: usize,
        class_labels: &Vec<i32>,
    ) -> Option<Box<Node>> {
        let num_samples: usize = X.row;
        let num_features: usize = X.col;

        // allow the tree get deeper as possible

        let depth: f32;

        if self.max_depth > 0 {
            depth = self.max_depth as f32;
        } else {
            depth = std::f32::INFINITY;
        }

        //println!("{:?}", curr_depth);

        if (num_samples >= self.min_samples_split) && ((curr_depth as f32) < depth) {
            let best_split: BestSplitStruct =
                self.get_best_split(X, Y, &ps, num_samples, num_features, &class_labels);

            //println!("{:?}", best_split);

            if best_split.info_gain > 0.0 {
                let left_subtree = DecisionTreeClassifier::build_tree(
                    self,
                    &best_split.dataset_left,
                    &best_split.y_left,
                    &ps,
                    curr_depth + 1,
                    &class_labels,
                );
                let right_subtree = DecisionTreeClassifier::build_tree(
                    self,
                    &best_split.dataset_right,
                    &best_split.y_right,
                    &ps,
                    curr_depth + 1,
                    &class_labels,
                );

                //println!("{:?}", left_subtree);

                return Some(Box::new(Node {
                    left: left_subtree,
                    right: right_subtree,
                    feature_index: best_split.feature_index,
                    threshold: best_split.threshold,
                    value: -1,
                }));
            }
        }

        let leaf_value: i32 = self.calculate_leaf_value(Y);

        Some(Box::new(Node {
            left: None,
            right: None,
            feature_index: 0,
            threshold: 0.0,
            value: leaf_value,
        }))
    }

    pub fn get_best_split(
        &mut self,
        X: &Matrix,
        Y: &Vec<i32>,
        ps: &HashMap<usize, (f32, f32, Vec<f32>)>,
        _num_samples: usize,
        num_features: usize,
        class_labels: &Vec<i32>,
    ) -> BestSplitStruct {
        let mut best_split: BestSplitStruct = BestSplitStruct {
            feature_index: 0,
            threshold: 0.0,
            dataset_left: Matrix {
                data: vec![],
                row: 0,
                col: 0,
            },
            dataset_right: Matrix {
                data: vec![],
                row: 0,
                col: 0,
            },
            y_left: vec![],
            y_right: vec![],
            info_gain: 0.0,
        };

        let mut max_info_gain = -std::f32::INFINITY;

        let mut evaluation: (bool, f32);
        let mut candidates: &Vec<f32>;

        for feature_index in 0..num_features {
            let mut c = 0;

            let pss = ps.get(&feature_index).unwrap();

            candidates = &pss.2;

            for &threshold in candidates.iter() {
                //println!("aaaaaaaaaa -> {:?} - {:?}", feature_index, threshold);

                c += 1;

                evaluation = self.evaluate_split(&X, &Y, feature_index, threshold, &class_labels);

                if evaluation.0 {
                    if evaluation.1 > max_info_gain {
                        best_split.feature_index = feature_index;
                        best_split.threshold = threshold;
                        best_split.info_gain = evaluation.1;

                        max_info_gain = evaluation.1;
                    }
                }
            }
        }

        let dataset_splitted: (Matrix, Matrix, Vec<i32>, Vec<i32>) =
            self.split(&X, &Y, best_split.feature_index, best_split.threshold);

        best_split.y_left = dataset_splitted.2;
        best_split.y_right = dataset_splitted.3;
        best_split.dataset_left = dataset_splitted.0;
        best_split.dataset_right = dataset_splitted.1;
        best_split
    }

    pub fn gini_index(&mut self, Y: &Vec<i32>, class_labels: &Vec<i32>) -> f32 {
        //let class_labels = utils::find_unique_values(&Y);
        //let class_labels = Y.unique();

        let y_len = &(Y.len() as i32);

        let gini_map: f32 = class_labels
            .iter()
            .map(|x| {
                let p_cls: f32 = (utils::count_vals(Y, x) as f32) / (*y_len) as f32;
                p_cls * p_cls
            })
            .sum();

        1.0 - gini_map
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

    pub fn evaluate_split(
        &mut self,
        X: &Matrix,
        Y: &Vec<i32>,
        feature_index: usize,
        threshold: f32,
        class_labels: &Vec<i32>,
    ) -> (bool, f32) {
        let n_cols = X.col;
        let n_rows = X.row;

        let mut y_right: Vec<i32> = vec![];
        let mut y_left: Vec<i32> = vec![];

        let mut value: i32;

        for i in 0..n_rows {
            value = Y[i];
            if (X.data[i * n_cols + feature_index] <= threshold) {
                y_left.push(value);
            } else {
                y_right.push(value);
            }
        }

        (
            (y_left.len() > 0 && y_right.len() > 0),
            self.information_gain(&Y, &y_left, &y_right, &class_labels),
        )
    }

    pub fn split(
        &mut self,
        X: &Matrix,
        Y: &Vec<i32>,
        feature_index: usize,
        threshold: f32,
    ) -> (Matrix, Matrix, Vec<i32>, Vec<i32>) {
        let mut vec_left = vec![];
        let mut vec_right = vec![];

        let mut y_right: Vec<i32> = vec![];
        let mut y_left: Vec<i32> = vec![];

        let n_cols = X.col;
        let n_rows = X.row;

        let mut row_left = 0;
        let mut row_right = 0;

        let mut v_y: &i32;
        let mut v: &[f32];

        let now = Instant::now();

        for i in 0..n_rows {
            //println!("{} - {} - {}", i, (i*n_cols), (i*n_cols + n_cols)-1);
            //let now3 = Instant::now();

            v = &(X.data[(i * n_cols)..(i * n_cols + n_cols)]);
            //let now4 = Instant::now();
            //println!("get {:?}", now4.duration_since(now3));

            v_y = &Y[i];

            if v[feature_index] <= threshold {
                vec_left.extend(v.iter());
                y_left.push(*v_y);

                row_left += 1;
            } else {
                vec_right.extend(v.iter());
                y_right.push(*v_y);

                row_right += 1;
            }
        }

        let dataset_left: Matrix = Matrix {
            data: vec_left,
            row: row_left,
            col: n_cols,
        };
        let dataset_right: Matrix = Matrix {
            data: vec_right,
            row: row_right,
            col: n_cols,
        };
        let now2 = Instant::now();
        //println!("split {:?}", now2.duration_since(now));

        (dataset_left, dataset_right, y_left, y_right)
    }

    pub fn information_gain(
        &mut self,
        parent: &Vec<i32>,
        l_child: &Vec<i32>,
        r_child: &Vec<i32>,
        class_labels: &Vec<i32>,
    ) -> f32 {
        //let now = Instant::now();

        let weight_l: f32 = l_child.len() as f32 / parent.len() as f32;
        let weight_r: f32 = r_child.len() as f32 / parent.len() as f32;

        //let now = Instant::now();
        let gain: f32 = self.gini_index(parent, &class_labels)
            - (weight_l * self.gini_index(l_child, &class_labels)
                + weight_r * self.gini_index(r_child, &class_labels));

        //let now2 = Instant::now();
        //println!("info_gain {:?}", now2.duration_since(now));
        gain
    }

    pub fn calculate_leaf_value(&mut self, Y: &Vec<i32>) -> i32 {
        //let now = Instant::now();
        let max = Y
            .iter()
            .fold(HashMap::<i32, usize>::new(), |mut m, x| {
                *m.entry(*x).or_default() += 1;
                m
            })
            .into_iter()
            .max_by_key(|(_, v)| *v)
            .map(|(k, _)| k);
        //let now2 = Instant::now();
        //println!("info_gain {:?}", now2.duration_since(now));
        max.unwrap()
    }

    pub fn fit(&mut self, X: &Matrix, Y: &Vec<i32>) {
        let mut ps: HashMap<usize, (f32, f32, Vec<f32>)> = HashMap::new();

        for f_idx in 0..X.col {
            let feature_values: Vec<f32> = utils::get_column(X, f_idx);
            let mut possible_thresholds: Vec<f32> =
                feature_values.iter().fold(Vec::new(), |mut vect, x| {
                    if !vect.contains(x) {
                        vect.push(*x);
                    }
                    vect
                });
            possible_thresholds.sort_by(|a, b| a.partial_cmp(b).unwrap());

            if possible_thresholds.len() <= 7 {
                ps.insert(
                    f_idx,
                    (
                        possible_thresholds[0],
                        possible_thresholds[possible_thresholds.len() - 1],
                        possible_thresholds,
                    ),
                );
            } else {
                let mut range_threshold: Vec<f32> = Vec::with_capacity(10);

                for i in 0..10 {
                    let delta: &f32 = &((possible_thresholds[possible_thresholds.len() - 1]
                        - possible_thresholds[0])
                        / 9 as f32);
                    range_threshold.push((possible_thresholds[0] + i as f32 * delta));
                }
                ps.insert(
                    f_idx,
                    (
                        possible_thresholds[0],
                        possible_thresholds[possible_thresholds.len() - 1],
                        range_threshold,
                    ),
                );
            }
        }
        //println!("{:?}", ps);
        let now = Instant::now();
        let class_labels = Y.iter().fold(vec![], |mut vect, x| {
            if !vect.contains(x) {
                vect.push(*x);
            }
            vect
        });
        self.root = self.build_tree(X, Y, &ps, 0, &class_labels);

        let now2 = Instant::now();

        println!("build_tree: {:?}", now2.duration_since(now));
    }

    pub fn make_prediction(&self, X: &[f32], tree: &Option<Box<Node>>) -> i32 {
        if tree.as_ref().unwrap().value != -1 {
            return tree.as_ref().unwrap().value;
        }

        let idx: usize = tree.as_ref().unwrap().feature_index;
        let feature_val = X[idx];

        if feature_val <= tree.as_ref().unwrap().threshold {
            let sub_tree_l = &tree.as_ref().unwrap().left;
            self.make_prediction(X, sub_tree_l)
        } else {
            let sub_tree_r = &tree.as_ref().unwrap().right;
            self.make_prediction(X, sub_tree_r)
        }
    }

    pub fn predict(&self, X: &Matrix) -> Vec<i32> {
        let mut predictions: Vec<i32> = vec![];
        let n_cols = X.col;

        for i in 0..X.row {
            let pred: i32 =
                self.make_prediction(&X.data[(i * n_cols)..(i * n_cols + n_cols)], &self.root);
            predictions.push(pred);
        }

        predictions
    }
}
