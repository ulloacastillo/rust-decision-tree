use std::fs::File;
use std::io::prelude::*;

use csv::Error;

#[derive(Debug)]
struct Node <'a>{
    left: Option<Box<Node<'a>>>,
    right: Option<Box<Node<'a>>>,
    feature_index: i32,
    threshold: f32,
    //for leaf Nodes
    value: &'a str
}

impl <'a> Node <'a>{
    pub fn new(fi: i32, th: f32, v: &'a str) -> Self {
        Node {
            left: None,
            right: None,
            feature_index: fi,
            threshold: th,
            value: v
        }
    }

}

struct DecisionTreeClassifier <'a>{
    root: Option<Box<Node<'a>>>,

    // stopping conditions
    min_samples_split: usize,
    max_depth: usize
}

impl <'a> DecisionTreeClassifier <'a> {
    pub fn new(&mut self, mss: usize, md: usize) -> Self{
        DecisionTreeClassifier {
            root: None,
            min_samples_split: mss,
            max_depth: md
        }
    }

    pub fn build_tree(&mut self, X: Vec<Vec<f32>>, Y: Vec<f32>, curr_depth: usize) -> Node <'a>{
        let num_samples = X.len();
        let num_features = X[0].len();

        if num_samples >= self.min_samples_split && curr_depth <= self.max_depth {
            println!("");
        }

        Node {
            left: None,
            right: None,
            feature_index: 1,
            threshold: 1.0,
            value: ""
        }
    }

    pub fn get_best_split(&mut self, X: Vec<Vec<f32>>, Y: Vec<f32>, num_samples: usize, num_features: usize){
        println!("");
    }

    pub fn gini_index(&mut self, Y: &Vec<&str>) -> f32 {
        let class_labels = unique_vals(Y);
        let mut gini = 0.0;
        
        for cls in class_labels.iter() {
            let p_cls: f32 = (count_vals(&Y, &cls) / Y.len()) as f32;
            gini = gini + (p_cls * p_cls);

        }
        gini
    }
}

fn unique_vals<'a>(arr: &'a Vec<&str>) -> Vec<&'a str> {
    let mut u_vals: Vec<&str> = vec![];
    for el in arr.iter() {
        if !u_vals.contains(&el) {
            u_vals.push(el);
        }
    }
    u_vals
}

fn count_vals(arr: &Vec<&str>, label: &str) -> usize {
    let mut c = 0;
    for el in arr.iter() {
        if el == &label {
            c = c + 1;
        }
    }
    c
}

/*impl From<Node <'_>> for Option<Box<Node <'_>>> {
    fn from(node: Node<'_>) -> Self {
        Some(Box::new(node))
    }
}*/

fn main() -> Result<(), Error> {
    println!("Hello, world!");
    let mut file = File::open("./iris.csv").expect("No se pudo abrir el arcivo");

    let mut content = String::new();

    file.read_to_string(&mut content).expect("No se pudo leer el archivo");


    //println!("{}", content);


    let mut _reader = csv::Reader::from_reader(content.as_bytes());
    //let mut y = Vec::new();




    // Call split, and use collect() to get a string Vec.
    let values: Vec<&str> = content.split('\n').collect();
    let mut y: Vec<&str> = vec![];
    let mut x: Vec<Vec<f32>> = vec![];

    // Pretty-print the results.
    let _xs: Vec<&str> = values[0].split(',').collect();
    

    for _row in values.iter() {
        let xs: Vec<&str> = _row.split(',').collect();
        
        let mut aux: Vec<f32> = vec![];
        for i in 0..xs.len()-1{
            aux.push(xs[i].parse::<f32>().unwrap());
        }
        y.push(xs[xs.len() -1]);
        x.push(aux);
        

    }

    for _row in x.iter(){
        //println!("VALUES: {:?}", _row);
        continue;
    }
    
    let mut a = 4;
    let mut b = 2.0;

    let mut n = Node::new(a, b, "xd");
    println!("VALUES: {:?}", n);


    Ok(())
    
}

