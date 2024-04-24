use pyo3::prelude::*;
use pyo3::exceptions::PyFileNotFoundError;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// Function that returns a "hello $NAME" string.
#[pyfunction]
fn say_hello(name: String) -> PyResult<String> {
    Ok(format!("Hello {}, how are you?", name))
}

/// Checks if a text file contains the given name.
#[pyfunction]
fn check_reg(filename: String, name: String) -> PyResult<String> {
    let file_result = File::open(filename);
    match file_result {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            if contents.contains(&name) {
                Ok("You are registered!".to_string())
            } else {
                Ok("Sorry, you're not on the list!".to_string())
            }
        },
        Err(_) => {
            Err(PyFileNotFoundError::new_err("File doesn't exist!"))
        },
    }
}

/// Return length of str vector
#[pyfunction]
fn count_att(att_list: Vec<String>) -> PyResult<usize> {
    Ok(att_list.len())
}

/// Give a dictionary of travel budgets and calculate average
#[pyfunction]
fn travel_avg(budget_dict: HashMap<String, f32>) -> PyResult<f32> {
    let mut sum: f32 = 0.0;
    let mut count: f32 = 0.0;
    for (_, budget) in budget_dict {
        sum = sum + budget;
        count = count + 1.0;
    }
    Ok(sum/count)
}

/// A Python module implemented in Rust.
#[pymodule]
fn start_from_scratch_arne(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(say_hello, m)?)?;
    m.add_function(wrap_pyfunction!(check_reg, m)?)?;
    m.add_function(wrap_pyfunction!(count_att, m)?)?;
    m.add_function(wrap_pyfunction!(travel_avg, m)?)?;
    Ok(())
}
