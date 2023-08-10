#[macro_use]
extern crate cpython;

use cpython::{Python, PyResult};
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub fn load_mat_file(file_path: &str) -> Vec<Vec<bool>> {
    let f = BufReader::new(File::open(file_path).unwrap());


    //let mut s = String::new();
    //f.read_line(&mut s).unwrap();

    let arr: Vec<Vec<bool>> = f.lines()
        .map(|l| l.unwrap().split(',')
             .map(|number| number.parse::<f64>().unwrap() > 0.5)
             .collect())
        .collect();
    return arr
}

fn load_vec_file(file_path: &str) -> Vec<f64> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let numbers: Vec<f64> = reader
        .lines()
        .map(|line| line.unwrap().parse::<f64>().unwrap())
        .collect();
    return numbers
}


fn _rec(matrix : &Vec<Vec<bool>>,i:usize, check : &Vec<usize>) -> Vec<Vec<usize>> {
    let mut rets= vec![];
    let mut track = vec![];
    for j in check {
        if matrix[i][*j] {
            rets.push(vec![*j]);  // TODO can be skipped if in any latter list!
            for p in _rec(matrix,*j,&track) {
                let mut tmp = vec![*j];
                tmp.extend(p);
                rets.push(tmp);
            }
            track.push(*j)
        }
    }
    return rets
}

pub fn rec(matrix : &Vec<Vec<bool>>) -> Vec<Vec<usize>> {
    let mut rets = vec![];
    for i in 0..matrix.len() {
        let vec : Vec<usize> = (i+1..matrix.len()).collect::<Vec<_>>().into_iter().rev().collect();
        rets.push(vec![i]); // TODO can be skipped if in any latter list!
        for p in _rec(matrix,i,&vec) {
            let mut tmp = vec![i];
            tmp.extend(p);
            rets.push(tmp);
        }
    }
    return rets
}


fn rec_py(_py: Python) -> PyResult<Vec<Vec<usize>>> {
    let weights = load_vec_file("weights.csv");
    let mat = load_mat_file("pseudo.csv");
    let out = rec(&mat);
    Ok(out)
}

fn load_vec_file_py(_py: Python, val: &str) -> PyResult<Vec<f64>> {
    let out = load_vec_file(val);
    Ok(out)
}

fn load_mat_file_py(_py: Python, val: &str) -> PyResult<Vec<Vec<bool>>> {
    let out = load_mat_file(val);
    Ok(out)
}

py_module_initializer!(rustypathfinder, |py, m | {
    m.add(py, "__doc__", "This module is implemented in Rust")?;
    m.add(py, "rec", py_fn!(py, rec_py()))?;
    m.add(py, "load_vec_file", py_fn!(py, load_vec_file_py(val: &str)))?;
    m.add(py, "load_mat_file", py_fn!(py, load_mat_file_py(val: &str)))?;
    Ok(())
});

#[test]
fn testit() {
    let mat = load_mat_file("pseudo.csv");
    let out = rec(&mat,0);
    println!("{:?}", out);
}