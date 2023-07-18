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

    let mut tree = dtree::DecisionTreeClassifier::new(2, 0);

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

    /*
           let X_test: Vec<f32> = vec![
               65.0, 0.0, 2.0, 140.0, 417.0, 1.0, 0.0, 157.0, 0.0, 0.8, 2.0, 1.0, 2.0, 54.0, 1.0, 0.0,
               140.0, 239.0, 0.0, 1.0, 160.0, 0.0, 1.2, 2.0, 0.0, 2.0, 77.0, 1.0, 0.0, 125.0, 304.0, 0.0,
               0.0, 162.0, 1.0, 0.0, 2.0, 3.0, 2.0, 61.0, 1.0, 0.0, 138.0, 166.0, 0.0, 0.0, 125.0, 1.0,
               3.6, 1.0, 1.0, 2.0, 57.0, 1.0, 0.0, 110.0, 201.0, 0.0, 1.0, 126.0, 1.0, 1.5, 1.0, 0.0, 1.0,
               65.0, 1.0, 3.0, 138.0, 282.0, 1.0, 0.0, 174.0, 0.0, 1.4, 1.0, 1.0, 2.0, 54.0, 1.0, 2.0,
               125.0, 273.0, 0.0, 0.0, 152.0, 0.0, 0.5, 0.0, 1.0, 2.0, 41.0, 1.0, 0.0, 110.0, 172.0, 0.0,
               0.0, 158.0, 0.0, 0.0, 2.0, 0.0, 3.0, 59.0, 1.0, 0.0, 138.0, 271.0, 0.0, 0.0, 182.0, 0.0,
               0.0, 2.0, 0.0, 2.0, 54.0, 1.0, 0.0, 110.0, 239.0, 0.0, 1.0, 126.0, 1.0, 2.8, 1.0, 1.0, 3.0,
               70.0, 1.0, 1.0, 156.0, 245.0, 0.0, 0.0, 143.0, 0.0, 0.0, 2.0, 0.0, 2.0, 53.0, 1.0, 0.0,
               123.0, 282.0, 0.0, 1.0, 95.0, 1.0, 2.0, 1.0, 2.0, 3.0, 49.0, 0.0, 0.0, 130.0, 269.0, 0.0,
               1.0, 163.0, 0.0, 0.0, 2.0, 0.0, 2.0, 61.0, 1.0, 0.0, 140.0, 207.0, 0.0, 0.0, 138.0, 1.0,
               1.9, 2.0, 1.0, 3.0, 58.0, 1.0, 0.0, 128.0, 216.0, 0.0, 0.0, 131.0, 1.0, 2.2, 1.0, 3.0, 3.0,
               60.0, 1.0, 0.0, 145.0, 282.0, 0.0, 0.0, 142.0, 1.0, 2.8, 1.0, 2.0, 3.0, 58.0, 1.0, 1.0,
               125.0, 220.0, 0.0, 1.0, 144.0, 0.0, 0.4, 1.0, 4.0, 3.0, 62.0, 0.0, 2.0, 130.0, 263.0, 0.0,
               1.0, 97.0, 0.0, 1.2, 1.0, 1.0, 3.0, 63.0, 1.0, 0.0, 130.0, 330.0, 1.0, 0.0, 132.0, 1.0,
               1.8, 2.0, 3.0, 3.0, 41.0, 0.0, 2.0, 112.0, 268.0, 0.0, 0.0, 172.0, 1.0, 0.0, 2.0, 0.0, 2.0,
               43.0, 1.0, 0.0, 110.0, 211.0, 0.0, 1.0, 161.0, 0.0, 0.0, 2.0, 0.0, 3.0, 34.0, 0.0, 1.0,
               118.0, 210.0, 0.0, 1.0, 192.0, 0.0, 0.7, 2.0, 0.0, 2.0, 59.0, 1.0, 2.0, 126.0, 218.0, 1.0,
               1.0, 134.0, 0.0, 2.2, 1.0, 1.0, 1.0, 41.0, 1.0, 2.0, 112.0, 250.0, 0.0, 1.0, 179.0, 0.0,
               0.0, 2.0, 0.0, 2.0, 52.0, 1.0, 0.0, 125.0, 212.0, 0.0, 1.0, 168.0, 0.0, 1.0, 2.0, 2.0, 3.0,
               60.0, 1.0, 0.0, 130.0, 253.0, 0.0, 1.0, 144.0, 1.0, 1.4, 2.0, 1.0, 3.0, 35.0, 1.0, 0.0,
               126.0, 282.0, 0.0, 0.0, 156.0, 1.0, 0.0, 2.0, 0.0, 3.0, 54.0, 1.0, 0.0, 124.0, 266.0, 0.0,
               0.0, 109.0, 1.0, 2.2, 1.0, 1.0, 3.0, 67.0, 0.0, 0.0, 106.0, 223.0, 0.0, 1.0, 142.0, 0.0,
               0.3, 2.0, 2.0, 2.0, 40.0, 1.0, 0.0, 152.0, 223.0, 0.0, 1.0, 181.0, 0.0, 0.0, 2.0, 0.0, 3.0,
               45.0, 1.0, 1.0, 128.0, 308.0, 0.0, 0.0, 170.0, 0.0, 0.0, 2.0, 0.0, 2.0, 57.0, 0.0, 0.0,
               140.0, 241.0, 0.0, 1.0, 123.0, 1.0, 0.2, 1.0, 0.0, 3.0, 60.0, 1.0, 0.0, 125.0, 258.0, 0.0,
               0.0, 141.0, 1.0, 2.8, 1.0, 1.0, 3.0, 53.0, 0.0, 0.0, 138.0, 234.0, 0.0, 0.0, 160.0, 0.0,
               0.0, 2.0, 0.0, 2.0, 47.0, 1.0, 2.0, 108.0, 243.0, 0.0, 1.0, 152.0, 0.0, 0.0, 2.0, 0.0, 2.0,
               44.0, 1.0, 1.0, 120.0, 263.0, 0.0, 1.0, 173.0, 0.0, 0.0, 2.0, 0.0, 3.0, 57.0, 1.0, 2.0,
               128.0, 229.0, 0.0, 0.0, 150.0, 0.0, 0.4, 1.0, 1.0, 3.0, 65.0, 0.0, 0.0, 150.0, 225.0, 0.0,
               0.0, 114.0, 0.0, 1.0, 1.0, 3.0, 3.0, 63.0, 1.0, 0.0, 130.0, 254.0, 0.0, 0.0, 147.0, 0.0,
               1.4, 1.0, 1.0, 3.0, 57.0, 1.0, 1.0, 124.0, 261.0, 0.0, 1.0, 141.0, 0.0, 0.3, 2.0, 0.0, 3.0,
               67.0, 1.0, 0.0, 120.0, 229.0, 0.0, 0.0, 129.0, 1.0, 2.6, 1.0, 2.0, 3.0, 56.0, 0.0, 0.0,
               134.0, 409.0, 0.0, 0.0, 150.0, 1.0, 1.9, 1.0, 2.0, 3.0, 52.0, 1.0, 0.0, 112.0, 230.0, 0.0,
               1.0, 160.0, 0.0, 0.0, 2.0, 1.0, 2.0, 59.0, 1.0, 3.0, 134.0, 204.0, 0.0, 1.0, 162.0, 0.0,
               0.8, 2.0, 2.0, 2.0, 64.0, 1.0, 0.0, 128.0, 263.0, 0.0, 1.0, 105.0, 1.0, 0.2, 1.0, 1.0, 3.0,
               48.0, 0.0, 2.0, 130.0, 275.0, 0.0, 1.0, 139.0, 0.0, 0.2, 2.0, 0.0, 2.0, 47.0, 1.0, 0.0,
               110.0, 275.0, 0.0, 0.0, 118.0, 1.0, 1.0, 1.0, 1.0, 2.0, 61.0, 1.0, 3.0, 134.0, 234.0, 0.0,
               1.0, 145.0, 0.0, 2.6, 1.0, 2.0, 2.0, 51.0, 0.0, 0.0, 130.0, 305.0, 0.0, 1.0, 142.0, 1.0,
               1.2, 1.0, 0.0, 3.0, 47.0, 1.0, 2.0, 130.0, 253.0, 0.0, 1.0, 179.0, 0.0, 0.0, 2.0, 0.0, 2.0,
               58.0, 0.0, 3.0, 150.0, 283.0, 1.0, 0.0, 162.0, 0.0, 1.0, 2.0, 0.0, 2.0, 62.0, 1.0, 0.0,
               120.0, 267.0, 0.0, 1.0, 99.0, 1.0, 1.8, 1.0, 2.0, 3.0, 67.0, 0.0, 2.0, 115.0, 564.0, 0.0,
               0.0, 160.0, 0.0, 1.6, 1.0, 0.0, 3.0, 70.0, 1.0, 2.0, 160.0, 269.0, 0.0, 1.0, 112.0, 1.0,
               2.9, 1.0, 1.0, 3.0, 41.0, 0.0, 1.0, 105.0, 198.0, 0.0, 1.0, 168.0, 0.0, 0.0, 2.0, 1.0, 2.0,
               65.0, 0.0, 2.0, 160.0, 360.0, 0.0, 0.0, 151.0, 0.0, 0.8, 2.0, 0.0, 2.0, 74.0, 0.0, 1.0,
               120.0, 269.0, 0.0, 0.0, 121.0, 1.0, 0.2, 2.0, 1.0, 2.0, 58.0, 1.0, 0.0, 125.0, 300.0, 0.0,
               0.0, 171.0, 0.0, 0.0, 2.0, 2.0, 3.0, 40.0, 1.0, 0.0, 110.0, 167.0, 0.0, 0.0, 114.0, 1.0,
               2.0, 1.0, 0.0, 3.0, 59.0, 1.0, 0.0, 140.0, 177.0, 0.0, 1.0, 162.0, 1.0, 0.0, 2.0, 1.0, 3.0,
               54.0, 1.0, 0.0, 110.0, 206.0, 0.0, 0.0, 108.0, 1.0, 0.0, 1.0, 1.0, 2.0, 38.0, 1.0, 2.0,
               138.0, 175.0, 0.0, 1.0, 173.0, 0.0, 0.0, 2.0, 4.0, 2.0, 45.0, 0.0, 1.0, 130.0, 234.0, 0.0,
               0.0, 175.0, 0.0, 0.6, 1.0, 0.0, 2.0, 46.0, 1.0, 0.0, 140.0, 311.0, 0.0, 1.0, 120.0, 1.0,
               1.8, 1.0, 2.0, 3.0, 59.0, 1.0, 3.0, 170.0, 288.0, 0.0, 0.0, 159.0, 0.0, 0.2, 1.0, 0.0, 3.0,
               39.0, 1.0, 2.0, 140.0, 321.0, 0.0, 0.0, 182.0, 0.0, 0.0, 2.0, 0.0, 2.0, 56.0, 1.0, 0.0,
               132.0, 184.0, 0.0, 0.0, 105.0, 1.0, 2.1, 1.0, 1.0, 1.0, 41.0, 1.0, 2.0, 130.0, 214.0, 0.0,
               0.0, 168.0, 0.0, 2.0, 1.0, 0.0, 2.0, 68.0, 0.0, 2.0, 120.0, 211.0, 0.0, 0.0, 115.0, 0.0,
               1.5, 1.0, 0.0, 2.0, 44.0, 0.0, 2.0, 108.0, 141.0, 0.0, 1.0, 175.0, 0.0, 0.6, 1.0, 0.0, 2.0,
               43.0, 1.0, 0.0, 120.0, 177.0, 0.0, 0.0, 120.0, 1.0, 2.5, 1.0, 0.0, 3.0, 39.0, 0.0, 2.0,
               138.0, 220.0, 0.0, 1.0, 152.0, 0.0, 0.0, 1.0, 0.0, 2.0, 71.0, 0.0, 0.0, 112.0, 149.0, 0.0,
               1.0, 125.0, 0.0, 1.6, 1.0, 0.0, 2.0, 56.0, 0.0, 0.0, 200.0, 288.0, 1.0, 0.0, 133.0, 1.0,
               4.0, 0.0, 2.0, 3.0, 46.0, 0.0, 2.0, 142.0, 177.0, 0.0, 0.0, 160.0, 1.0, 1.4, 0.0, 0.0, 2.0,
           ];

            let y_test = vec![
                1, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 1, 0,
                1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 0, 0,
                0, 1, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1,
            ];

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

        let mut confusion_matrix = vec![vec![0; unique_labels.len()]; unique_labels.len()];

        let mut c: f32 = 0.;

        for i in 0..predictions.len() {
            if predictions[i] == y_test_iris[i] {
                c += 1.0;
            }

            confusion_matrix[predictions[i] as usize][y_test_iris[i] as usize] += 1;
        }
    */
    // println!("accuracy: {:?}", c / y_test_iris.len() as f32);
    // println!("{:?}", confusion_matrix);

    //let mut file = std::fs::File::create("output.pickle").unwrap();
    //serde_pickle::to_writer(&mut file, &rf, serde_pickle::SerOptions::new()).unwrap();
}
