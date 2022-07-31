use std::fs::File;
use std::io::prelude::*;
use rand::prelude::*;
use csv::Error;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn split_dataset(X: &mut Vec<Vec<f32>>, Y: &mut Vec<String>, train_size: f32) -> (Vec<Vec<f32>>,  Vec<String>, Vec<Vec<f32>>,Vec<String>) {
    let n_train = (X.len() as f32 * train_size) as usize;
    let n_test = X.len() - n_train;
    
    let mut X_test: Vec<Vec<f32>> = vec![];
    let mut X_train: Vec<Vec<f32>> = vec![];
    let mut Y_test: Vec<String> = vec![];
    let mut Y_train: Vec<String> = vec![];
    let mut idxs: Vec<usize> = (0..X.len()).collect();


    let mut rng = rand::rngs::StdRng::seed_from_u64(41);

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

pub fn accuracy_per_label(Y: &Vec<String>, Y_hat: &Vec<String>) -> Vec<f32> {
    let mut acc: Vec<f32> = vec![];

    let labels = unique_vals(&Y);


    for i in 0..labels.len() {
        let mut c = 0.0;
        for j in 0..Y_hat.len() {
            if Y[j] == labels[i] {
                if Y_hat[j] == Y[j] {
                    c = c + 1.0;
                }
            }
        }

        let total_labels = count_vals(&Y, labels[i].clone());
        let label_acc = c / (total_labels as f32);
        acc.push(label_acc);
    }

    acc
}

pub fn count_vals(arr: &Vec<String>, label: String) -> usize {
    let mut c = 0;
    for el in arr.iter() {
        if el == &label {
            c = c + 1;
        }
    }
    
    c
}

pub fn unique_vals(arr: &Vec<String>) -> Vec<String> {
    let mut u_vals: Vec<String> = vec![];
    for el in arr.iter() {
        if !u_vals.contains(&el) {
            u_vals.push(el.to_string());
        }
    }
    u_vals
}

pub fn unique_vals_f32(arr: &Vec<f32>) -> Vec<f32> {
    let mut u_vals: Vec<f32> = vec![];
    for el in arr.iter() {
        if !u_vals.contains(&el) {
            u_vals.push(*el);
        }
    }
    println!("antes: {:?}", u_vals);
    u_vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("despues: {:?}", u_vals);
    let returnded = u_vals.clone();

    return returnded;
}


pub fn get_column(matrix: &Vec<Vec<f32>>, col: usize) -> Vec<f32>{
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
