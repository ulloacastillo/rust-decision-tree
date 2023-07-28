use crate::dtree::Matrix;

use rand::prelude::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

mod dtree;
mod random_forest;
mod utils;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let rows_train: usize = args[1].parse::<usize>().unwrap();
    let cols_train: usize = args[2].parse::<usize>().unwrap();
    let file_name = &args[3];

    let mut file = File::open(format!("{}.csv", file_name)).expect("No se pudo abrir el arcivo");

    let mut content = String::new();

    file.read_to_string(&mut content)
        .expect("No se pudo leer el archivo");

    let mut _reader = csv::Reader::from_reader(content.as_bytes());

    let values: Vec<String> = content.split('\n').map(str::to_string).collect();
    let mut y: Vec<i32> = vec![];
    let mut x: Vec<f32> = vec![];

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

    let mut tree = dtree::DecisionTreeClassifier::new(2, 10);

    let matrix_width = x.len() / rows_train;
    let submatrix: Vec<f32> = x
        .chunks(matrix_width)
        .take(rows_train)
        .flat_map(|row| row.iter().take(cols_train).cloned())
        .collect();

    let sub_y: Vec<i32> = y.iter().take(rows_train).cloned().collect();

    let matrix: Matrix = Matrix {
        data: submatrix,
        row: rows_train,
        col: cols_train,
    };

    tree.fit(&matrix, &sub_y);

    let X_test: Vec<f32> = vec![
        5.5, 2.4, 3.7, 1.0, 5.1, 3.8, 1.5, 0.3, 5.9, 3.0, 5.1, 1.8, 7.7, 2.6, 6.9, 2.3, 5.1, 3.5,
        1.4, 0.2, 5.4, 3.7, 1.5, 0.2, 5.1, 3.5, 1.4, 0.3, 5.6, 3.0, 4.1, 1.3, 4.5, 2.3, 1.3, 0.3,
        7.0, 3.2, 4.7, 1.4, 5.2, 3.4, 1.4, 0.2, 6.7, 3.0, 5.2, 2.3, 4.6, 3.2, 1.4, 0.2, 4.9, 3.1,
        1.5, 0.2, 6.0, 2.9, 4.5, 1.5, 5.2, 3.5, 1.5, 0.2, 6.4, 3.1, 5.5, 1.8, 6.0, 2.7, 5.1, 1.6,
        6.3, 2.8, 5.1, 1.5, 5.6, 2.5, 3.9, 1.1, 5.0, 3.0, 1.6, 0.2, 6.0, 2.2, 4.0, 1.0, 5.5, 2.3,
        4.0, 1.3, 5.6, 2.8, 4.9, 2.0, 5.8, 4.0, 1.2, 0.2, 4.9, 3.0, 1.4, 0.2, 4.8, 3.1, 1.6, 0.2,
        5.4, 3.4, 1.7, 0.2, 6.3, 3.4, 5.6, 2.4, 5.0, 3.3, 1.4, 0.2, 4.9, 3.6, 1.4, 0.1,
    ];

    let y_test = vec![
        2, 1, 3, 3, 1, 1, 1, 2, 1, 2, 1, 3, 1, 1, 2, 1, 3, 2, 3, 2, 1, 2, 2, 3, 1, 1, 1, 1, 3, 1, 1,
    ];

    let pred_matrix: Matrix = Matrix {
        data: X_test,
        row: 31,
        col: 4,
    };
    //let predictions = tree.predict(&pred_matrix);

    // let unique_labels = utils::unique_vals(&y_test);

    // let mut confusion_matrix = vec![vec![0; unique_labels.len()]; unique_labels.len()];

    // let mut c: f32 = 0.;

    // for i in 0..predictions.len() {
    //     if predictions[i] == y_test[i] {
    //         c += 1.0;
    //     }

    //     confusion_matrix[predictions[i] as usize - 1][y_test[i] as usize - 1] += 1;
    // }

    // println!("accuracy: {:?}", c / y_test.len() as f32);
    // println!("{:?}", confusion_matrix);
    // println!("{:?}", confusion_matrix);

    /*


            let X_test_iris = vec![
                6.7, 3.3, 5.7, 2.1, 4.7, 3.2, 1.3, 0.2, 6.3, 2.8, 5.1, 1.5, 4.4, 3.0, 1.3, 0.2, 6.0, 2.2,
                5.0, 1.5, 5.0, 3.5, 1.6, 0.6, 4.9, 2.5, 4.5, 1.7, 6.7, 3.3, 5.7, 2.5, 6.4, 3.2, 5.3, 2.3,
                7.0, 3.2, 4.7, 1.4, 6.4, 3.2, 4.5, 1.5, 7.3, 2.9, 6.3, 1.8, 6.7, 3.1, 5.6, 2.4, 6.2, 2.9,
                4.3, 1.3, 6.0, 2.7, 5.1, 1.6, 5.1, 3.8, 1.9, 0.4, 6.7, 2.5, 5.8, 1.8, 5.0, 3.2, 1.2, 0.2,
                5.8, 2.7, 3.9, 1.2, 5.1, 3.5, 1.4, 0.2, 6.9, 3.1, 5.4, 2.1, 5.7, 4.4, 1.5, 0.4, 4.9, 2.4,
                3.3, 1.0, 5.7, 3.8, 1.7, 0.3, 5.5, 2.5, 4.0, 1.3, 7.7, 3.8, 6.7, 2.2, 5.2, 2.7, 3.9, 1.4,
                4.8, 3.4, 1.6, 0.2, 5.0, 3.3, 1.4, 0.2, 5.7, 2.9, 4.2, 1.3, 6.0, 2.9, 4.5, 1.5, 5.9, 3.2,
                4.8, 1.8, 5.4, 3.9, 1.7, 0.4, 5.1, 3.5, 1.4, 0.3, 4.3, 3.0, 1.1, 0.1, 7.1, 3.0, 5.9, 2.1,
                4.9, 3.0, 1.4, 0.2, 6.2, 3.4, 5.4, 2.3,
            ];
            let y_test_iris = vec![
                0, 1, 0, 1, 0, 1, 0, 0, 0, 2, 2, 0, 0, 2, 2, 1, 0, 1, 2, 1, 0, 1, 2, 1, 2, 0, 2, 1, 1, 2,
                2, 2, 1, 1, 1, 0, 1, 0,
            ];

            let pred_matrix: Matrix = Matrix {
                data: X_test_iris,
                row: 38,
                col: 4,
            };
            let predictions = tree.predict(&pred_matrix);

            let unique_labels = utils::unique_vals(&y_test_iris);

    */

    //let mut file = std::fs::File::create("output.pickle").unwrap();
    //serde_pickle::to_writer(&mut file, &rf, serde_pickle::SerOptions::new()).unwrap();
}
