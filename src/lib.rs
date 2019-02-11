#![feature(specialization)]

use pyo3::prelude::*;

mod py_channel_id;
use crate::py_channel_id::PyChannelIdentifier;

#[pymodule]
fn py_disc_model(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyChannelIdentifier>()?;

    Ok(())
}
