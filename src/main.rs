use crate::dtree::{Matrix};

use rand::prelude::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::time::{Instant};

mod dtree;
mod random_forest;
mod utils;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let mut file = File::open("./iris.csv").expect("No se pudo abrir el arcivo");

    let mut content = String::new();

    file.read_to_string(&mut content)
        .expect("No se pudo leer el archivo");

    let mut _reader = csv::Reader::from_reader(content.as_bytes());

    let values: Vec<String> = content.split('\n').map(str::to_string).collect();
    let mut y: Vec<i32> = vec![];
    let mut x: Vec<f32> = vec![];

    // Pretty-print the results.
    let _xs: Vec<String> = values[0].split(',').map(str::to_string).collect();

    for _row in values.iter() {
        let xs: Vec<String> = _row.split(',').map(str::to_string).collect();

        let mut aux: Vec<f32> = vec![];
        for i in 0..&xs.len() - 1 {
            aux.push(xs[i].parse::<f32>().unwrap());
        }
        let a = &xs.len() - 1;
        let b = &xs[a];

        y.push(b.parse::<f32>().unwrap() as i32);
        x.append(&mut aux);
        //&x.push(aux);
    }

    let args: Vec<String> = env::args().collect();

    let rows_train: usize = args[1].parse::<usize>().unwrap();
    let cols_train: usize = args[2].parse::<usize>().unwrap();

    //println!("{:?}", rows_train * 2);

    //setosa = 1; versicolor = 2; virginica = 3

    // Parameters n_trees, min_samples_split, max_depth, n_feats
    //let mut rf = random_forest::RandomForest::new(1, 2, 0, 4, 41);
    let mut tree = dtree::DecisionTreeClassifier::new(2, 0);

    //println!("{}", x.len());

    let _x_array = [
        vec![5.1, 3.5, 1.4, 0.2, 1.],
        vec![5.1, 3.5, 1.4, 0.2, 1.],
        vec![5.1, 3.5, 1.4, 0.2, 1.],
        vec![5.1, 3.5, 1.4, 0.2, 1.],
        vec![5.1, 3.5, 1.4, 0.2, 1.],
        vec![5.1, 3.5, 1.4, 0.2, 1.],
        vec![5.1, 3.5, 1.4, 0.2, 1.],
    ];

    let _y_array = [1, 1, 2, 2, 3, 3, 1];

    let matrix: Matrix = Matrix {
        data: x,
        row: rows_train,
        col: cols_train,
    };

    let now = Instant::now();

    tree.fit(&matrix, &y);

    let now2 = Instant::now();
    println!("{:?}", now2.duration_since(now));

    //let mut file = std::fs::File::create("output.pickle").unwrap();
    //serde_pickle::to_writer(&mut file, &rf, serde_pickle::SerOptions::new()).unwrap();
}
