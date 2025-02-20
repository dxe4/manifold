use pyo3::prelude::*;
use pyo3::types::PyAny;
use pyo3::types::PyBool;
use rug::Integer;
use std::str::FromStr;

mod math;
use math::collatz::{collatz_sequence_impl, Collatz};
use math::fib_calc::fib_matrix;
use math::inneficient::sum_of_factors_from_pentagonal_numbers;
use math::padic::{x_pow_y_pow_z_mod_k, NumberConfig};
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
fn collatz_sequence(a: &PyAny) -> Collatz {
    let num_a = to_rug_integer(a).unwrap();
    collatz_sequence_impl(num_a)
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

#[pyfunction]
fn power_of_two_exponent_10n_py(start: usize, end: usize) -> PyResult<Vec<String>> {
    let config = NumberConfig {
        start: start,
        end: end,
        ..Default::default()
    };
    Ok(x_pow_y_pow_z_mod_k(config))
}

#[pymodule]
fn manifold_rs(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(miller_rabin_bool, m)?)?;
    m.add_function(wrap_pyfunction!(add_numbers, m)?)?;
    m.add_function(wrap_pyfunction!(miller_rabin_bool_multiple, m)?)?;
    m.add_function(wrap_pyfunction!(power_of_two_exponent_10n_py, m)?)?;
    m.add_function(wrap_pyfunction!(collatz_sequence, m)?)?;
    m.add_class::<Collatz>()?;
    Ok(())
}
