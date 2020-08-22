use std::sync::Arc;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rayon::prelude::*;
use numpy::{IntoPyArray, PyArrayDyn, PyArray2};

use phys;

#[pyclass]
struct FDMRunner {
    runner: Arc<phys::finite_difference_method::FDMRunner>,
}

#[pymethods]
impl FDMRunner {
    #[new]
    pub fn new(
        size: (usize, usize),
        speed: f64,
        delta_x: f64,
        delta_t: f64,
        source_pos: (usize, usize),
    ) -> Self {
        let runner = phys::finite_difference_method::FDMRunner::new(
            size, speed, delta_x, delta_t, source_pos,
        );
        FDMRunner {
            runner: Arc::new(runner),
        }
    }

    pub fn run<'py>(&self, py: Python<'py>, steps: usize) -> Vec<Py<PyArray2<f64>>>{
        let phys_ret = self.runner.run(steps);
        let mut ret = vec![];

        for res in phys_ret.iter() {
            let res = res.clone().into_pyarray(py);
            ret.push(res.to_owned());
        }
        ret
    }
}

#[pymodule]
fn physpy(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<FDMRunner>()?;

    Ok(())
}
