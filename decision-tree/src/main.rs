use std::fs::File;
use std::io::prelude::*;
use rand::prelude::*;
use csv::Error;

mod dtree;
mod utils;
mod random_forest;


fn main()  {
    
    let mut file = File::open("./iris.csv").expect("No se pudo abrir el arcivo");

    let mut content = String::new();

    file.read_to_string(&mut content).expect("No se pudo leer el archivo");


    let mut _reader = csv::Reader::from_reader(content.as_bytes());


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

    /*  ----------------------------  |*/
    /*                                |*/
    /*     Random Forest Section      |*/
    /*                                |*/
    /*  ----------------------------  |*/

    let (X_train, Y_train, X_test, Y_test) = utils::split_dataset(&mut x, &mut y, 0.8);

    println!("{}", X_train.clone().len());

    // for i in 0..X_train.clone().len() {
    //     println!("{:?} - {:?}", X_train[i].clone(), Y_train[i].clone());
    //     println!("{:?}", Y_train[i].clone());
    // }
    
    // Parameters n_trees, min_samples_split, max_depth, n_feats
    let mut rf = random_forest::RandomForest::new(1, 3, 3, 4);
    //let mut tree = dtree::DecisionTreeClassifier::new(3, 3);
    rf.fit(&X_train, &Y_train);
    //println!("{:?}", X_train);
    let y_pred = rf.predict(&X_test);

    //println!("{:?}", y_pred);
    //println!("{:?}", Y_test);

    println!("Accuracy: {}", random_forest::accuracy(&Y_test, &y_pred));

    //println!("aaa: {}", random_forest::count_val("hola".to_string(), &vector));

   
}

