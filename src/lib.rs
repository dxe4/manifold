use pyo3::prelude::*;
use pyo3::types::PyAny;
use pyo3::types::PyBool;
use rug::Integer;
use std::str::FromStr;

mod math;
use math::inneficient::sum_of_factors_from_pentagonal_numbers;
use math::primes::miller_rabin_impl;

fn to_rug_integer(obj: &PyAny) -> PyResult<Integer> {
    let str_val = obj.str()?.to_string();
    Integer::from_str(&str_val)
        .map_err(|_| pyo3::exceptions::PyValueError::new_err("Invalid integer value."))
}

#[pyfunction]
fn miller_rabin_bool_multiple(a: &PyAny, b: &PyAny) -> PyResult<Vec<Py<PyBool>>> {
    let num_a = to_rug_integer(a)?;
    let num_b = to_rug_integer(b)?;

    let result = miller_rabin_impl(&num_a, &num_b);

    Python::with_gil(|py| {
        Ok(result
            .into_iter()
            .map(|b| PyBool::new(py, b).into_py(py))
            .collect())
    })
}

#[pyfunction]
fn miller_rabin_bool(a: &PyAny) -> PyResult<Py<PyBool>> {
    let num_a = to_rug_integer(a).unwrap();

    let result = miller_rabin_impl(&num_a, &num_a);
    Python::with_gil(|py| Ok(PyBool::new(py, result[0]).into_py(py)))
}

#[pyfunction]
fn add_numbers(a: &PyAny, b: &PyAny) -> PyResult<PyObject> {
    fn to_rug_integer(obj: &PyAny) -> PyResult<Integer> {
        if let Ok(int_val) = obj.extract::<i64>() {
            Ok(Integer::from(int_val))
        } else if let Ok(str_val) = obj.extract::<String>() {
            Integer::from_str(&str_val).map_err(|_| {
                pyo3::exceptions::PyValueError::new_err("Invalid string for integer conversion.")
            })
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(
                "Argument must be an integer or a string.",
            ))
        }
    }

    let num_a = to_rug_integer(a)?;
    let num_b = to_rug_integer(b)?;

    let sum = num_a + num_b;

    Python::with_gil(|py| {
        if let Some(i64_val) = sum.to_i64() {
            Ok(i64_val.into_py(py))
        } else if let Some(u64_val) = sum.to_u64() {
            Ok(u64_val.into_py(py))
        } else {
            Ok(sum.to_string().into_py(py))
        }
    })
}

#[pymodule]
fn manifold_rs(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(miller_rabin_bool, m)?)?;
    m.add_function(wrap_pyfunction!(add_numbers, m)?)?;
    m.add_function(wrap_pyfunction!(miller_rabin_bool_multiple, m)?)?;
    // ;
    Ok(())
}
