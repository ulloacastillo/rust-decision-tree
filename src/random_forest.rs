use crate::dtree;
use rand::prelude::*;

use crate::dtree::Matrix;
use serde::{Deserialize, Serialize};


pub fn bootstrap_random_forest(X: &Vec<Vec<f32>>, Y: &Vec<i32>, n_features: usize, rng: &mut rand::rngs::StdRng) -> (Vec<Vec<f32>>, Vec<i32>) { 
    
    let mut X_: Vec<Vec<f32>> = vec![];
    let mut y_: Vec<i32> = vec![];

    
    // Creating features indexes
    let mut ft_ixs: Vec<usize> = vec![];
    let mut rnd_ft_idx: usize;
    let n_cols = X[0].len();
    for _i in 0..n_features {
        rnd_ft_idx = rng.gen_range(0..n_cols);
        while ft_ixs.contains(&rnd_ft_idx) {
            rnd_ft_idx = rng.gen_range(0..n_cols);
            
        }
        
        ft_ixs.push(rnd_ft_idx);
    }
    //println!("{:?}", ft_ixs);
    let n_samples: usize = X.len();
    for _i in 0..n_samples {
        let rnd_idx = rng.gen_range(0..n_samples);

        let mut X_i: Vec<f32> = vec![];
        for j in 0..n_features {
            X_i.push(X[rnd_idx][j]);
        }
        X_.push(X_i);
        y_.push(Y[rnd_idx]);
    }

    (X_, y_)
}

pub fn swap_matrix_axes(matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>>{
    let mut new_matrix: Vec<Vec<i32>> = vec![];
    let n_rows = matrix.len();
    let n_cols = matrix[0].len();
    for j in 0..n_cols{
        let mut aux: Vec<i32> = vec![];
        for i in 0..n_rows {
            let s_: &i32 = &matrix[i][j];
            
            aux.push(*s_);
        }
        new_matrix.push(aux);
    }

    new_matrix

}

pub fn count_val(el: i32, Y: &Vec<i32>) -> usize{
    let mut c: usize = 0;
    for i in 0..Y.len() {
        if Y[i] == el {
            c += 1;
        }
    }c
}

pub fn get_most_common(Y: &Vec<i32>) -> i32 {
    let mut common_count: usize = 0;
    let mut common_elem: i32 = 0;
    for i in 0..Y.len() {
        let count = count_val(Y[i], Y);
        if count > common_count {
            common_count = count;
            common_elem = Y[i];
        }
    }
    common_elem

}

pub fn accuracy(y: &Vec<String>, y_hat: &Vec<String>) -> f32{
    let mut sum = 0.0;
    for i in 0..y.len(){
        if y[i] == y_hat[i] {
            sum += 1.0;
        }
    }
    sum / (y.len() as f32)
    
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RandomForest {
    n_trees: usize,
    min_samples_split: usize,
    max_depth: usize,
    n_feats: usize,
    seed: u64,
    trees: Vec<dtree::DecisionTreeClassifier>
}

impl RandomForest {
    pub fn new(n_trees: usize, min_samples_split: usize, max_depth: usize, n_feats: usize, seed: u64) -> Self {
        let t: Vec<dtree::DecisionTreeClassifier> = Vec::new();
        RandomForest {
            n_trees,
            min_samples_split,
            max_depth,
            n_feats,
            seed,
            trees: t
        }
    }

    pub fn fit(&mut self, X: &Matrix, Y: &Vec<i32>) {
        
        let t: Vec<dtree::DecisionTreeClassifier> = Vec::new();
        self.trees = t;
        let _rng = rand::rngs::StdRng::seed_from_u64(self.seed);
        
        for i in 0..self.n_trees{
            let mut tree: dtree::DecisionTreeClassifier = dtree::DecisionTreeClassifier::new(self.min_samples_split, self.max_depth);
            //let (X_sample, y_sample) = bootstrap_random_forest(&X, &Y, self.n_feats, &mut rng);
            println!("{:?}", i);
            
            //let (X_sample, y_sample) = bootstrap_random_forest(&X, &Y, 2, self.n_feats, &mut rng);
            
            tree.fit(X, Y);
            self.trees.push(tree);
            
            
        }

        
    }

    pub fn predict(&mut self, X: &Vec<Vec<f32>>) -> Vec<i32> {
        let mut tree_preds: Vec<Vec<i32>> = vec![];
        for i in 0..self.n_trees {
            let y_pred_i = self.trees[i].predict(X);
            tree_preds.push(y_pred_i);
        }
        //println!("{}", tree_preds.len());
        let tree_preds_ = swap_matrix_axes(&tree_preds);
        //println!("{}", tree_preds_.len());
        let mut y_pred = vec![];
        
        for i in 0..tree_preds_.len() {
            let most_common_label = get_most_common(&tree_preds_[i]);
            y_pred.push(most_common_label)
        }
        y_pred

    }

}
