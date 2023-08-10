#[macro_use]
extern crate cpython;

use rayon::prelude::*;

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
fn edges(matrix : &Vec<Vec<bool>>, ii :usize) ->  Vec<usize> {
    let mut ret = matrix[ii].iter().enumerate().filter(|(i,b)| **b && *i > ii).map(|(i,b)| i).collect::<Vec<_>>();
    ret.push(matrix.len());
    ret
}

fn _no_rec(matrix : &Vec<Vec<bool>>,i:usize) -> Vec<Vec<usize>> {
    let mut rets = vec![];
    let target = matrix.len();
    let cutoff = matrix.len() + 1;
    let mut visited = vec![i];
    let mut stack : Vec<Vec<usize>> = vec![edges(matrix,i)];
    let mut good_nodes : Vec<Vec<usize>> = vec![edges(matrix,i)];
    let mut iters = 0;
    while stack.len()> 0 {
        //iters += 1;
        //if iters > 5 {
        //    break
        //}

        //println!("stacklen {:?}",stack.len());
        //println!("stack {:?}",stack);
        let mut children : &mut Vec<usize> = stack.last_mut().unwrap();
        
        //println!("visited {:?}",visited);
        //println!("good_nodes {:?}",good_nodes);
        if children.len() ==0 {
            // if child exists/is not none
            stack.pop();
            good_nodes.pop();
            visited.pop();
        }
        else {
            let child = children.remove(0);
            //println!("child {:?}",child);
            if visited.len()< cutoff {
                // if child in visited
                if visited.contains(&child) {
                    continue
                }
                // if child is target
                if child == target {
                    // yield visited
                    let mut tmp = visited.clone();
                    tmp.push(child);
                    rets.push(tmp);
                }
                // add child to visited
                visited.push(child);
                //println!("add child to visited {:?} ",child);
                //if target not in visited
                if !visited.contains(&target) {
                    //println!("target not in visited");
                    let good_children : Vec<usize> = edges(matrix,child);
                    //println!("good_children {:?}",good_children);
                    //matrix[child].iter().enumerate().filter(|(i,b)| **b).map(|(i,b)| i).collect::<Vec<_>>();
                    // intersection of good_children and good_nodes
                    let inter = good_children.into_iter().filter(|x| good_nodes.len()>0 && good_nodes.last().unwrap().contains(x)).collect::<Vec<_>>();
                    //if inter.len() != 0 {
                        good_nodes.push(inter);
                    //}
                    // append inter to stack
                    //println!("adding good_ndoes to stack {:?} ",good_nodes);
                    stack.push(good_nodes.last().unwrap().clone());
                }
                else {
                    //println!("target in visited");
                    // popitem from hashmap visited
                    visited.pop();
                    //println!("popitem from hashmap visited {:?} ",visited);
                }
            } else {
                // yield visited
                let mut tmp = visited.clone();
                tmp.push(child);
                rets.push(tmp);
                // popitem from hashmap visited
                stack.pop();
                good_nodes.pop();
                visited.pop();
            }
        }
    }
    return rets
}


fn _rec(matrix : &Vec<Vec<bool>>,i:usize, check : &Vec<usize>) -> Vec<Vec<usize>> {
    let mut rets= vec![];
    let mut track = vec![];
    for j in check {
        if matrix[i][*j] {
            let recs : Vec<Vec<usize>> = _rec(matrix,*j,&track);
            if recs.len() > 0 {
                for p in recs {
                    let mut tmp = vec![i];
                    tmp.extend(p);
                    rets.push(tmp);
                }
            } else {
                rets.push(vec![i,*j]);
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
        let recs : Vec<Vec<usize>>= _rec(matrix,i,&vec);
        rets.extend(recs);
    }
    return rets
}

pub fn rec_par(matrix : &Vec<Vec<bool>>) -> Vec<Vec<usize>> {
    let vecit : Vec<usize> = (0..matrix.len()).collect::<Vec<_>>();
    let rets : Vec<Vec<usize>> = vecit.par_iter().map(|i| {
        let vec : Vec<usize> = (*i+1..matrix.len()).collect::<Vec<_>>().into_iter().rev().collect();
        let recs : Vec<Vec<usize>>= _rec(matrix,*i,&vec);
        recs
    }).flatten().collect();
    return rets
}

pub fn no_rec_par(matrix : &Vec<Vec<bool>>) -> Vec<Vec<usize>> {
    let vecit : Vec<usize> = (0..matrix.len()).collect::<Vec<_>>();
    vecit.par_iter().map(|i| {
        _no_rec(matrix,*i)
    }).flatten().collect()
}


fn no_rec_py(_py: Python) -> PyResult<Vec<Vec<usize>>> {
    let weights = load_vec_file("weights.csv");
    let mat = load_mat_file("pseudo.csv");
    let out = no_rec_par(&mat);
    Ok(out)
}

fn rec_py(_py: Python) -> PyResult<Vec<Vec<usize>>> {
    let weights = load_vec_file("weights.csv");
    let mat = load_mat_file("pseudo.csv");
    let out = rec(&mat);
    Ok(out)
}


fn rec_par_py(_py: Python) -> PyResult<Vec<Vec<usize>>> {
    let weights = load_vec_file("weights.csv");
    let mat = load_mat_file("pseudo.csv");
    let out = rec_par(&mat);
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
    m.add(py, "no_rec", py_fn!(py, no_rec_py()))?;
    m.add(py, "rec", py_fn!(py, rec_py()))?;
    m.add(py, "rec_par", py_fn!(py, rec_par_py()))?;
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