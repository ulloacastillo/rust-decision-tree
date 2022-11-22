use std::fs::File;
use std::io::prelude::*;
use rand::prelude::*;
use csv::Error;
use std::time::{Duration, Instant};

mod dtree;
mod utils;
mod random_forest;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    
    let mut file = File::open("./iris.csv").expect("No se pudo abrir el arcivo");

    let mut content = String::new();

    file.read_to_string(&mut content).expect("No se pudo leer el archivo");


    let mut _reader = csv::Reader::from_reader(content.as_bytes());


    let values: Vec<String> = content.split('\n').map(str::to_string).collect();
    let mut y: Vec<i32> = vec![];
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
        &y.push(b.parse::<i32>().unwrap());
        &x.push(aux);
    }

    

    //setosa = 1; versicolor = 2; virginica = 3
    
    // Parameters n_trees, min_samples_split, max_depth, n_feats
    let mut rf = random_forest::RandomForest::new(1, 2, 0, 4, 41);
        
    println!("{}", x.len());

    let now = Instant::now();
    
    
    
    rf.fit(&x, &y);
    
    
    let now2 = Instant::now();
    println!("{:?}", now2.duration_since(now));
    
    //let mut file = std::fs::File::create("output.pickle").unwrap();
    //serde_pickle::to_writer(&mut file, &rf, serde_pickle::SerOptions::new()).unwrap();

   
   
}
