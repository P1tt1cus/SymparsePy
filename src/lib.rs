use pdb::{FallibleIterator, SymbolData, PDB};
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use regex::Regex;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

fn map_pdb_error(err: pdb::Error) -> PyErr {
    PyRuntimeError::new_err(err.to_string())
}

#[pyfunction]
fn parse_pdb(pdb_path: &str) -> PyResult<Vec<HashMap<String, String>>> {
    let file =
        File::open(&Path::new(pdb_path)).map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    let mut pdb = PDB::open(file).map_err(map_pdb_error)?;
    let symbol_table = pdb.global_symbols().map_err(map_pdb_error)?;
    let mut results = Vec::new();

    let mut symbols = symbol_table.iter();
    while let Some(symbol) = symbols.next().map_err(map_pdb_error)? {
        if let SymbolData::Public(data) = symbol.parse().map_err(map_pdb_error)? {
            let mut result = HashMap::new();
            result.insert("offset".to_string(), data.offset.offset.to_string());
            result.insert("section".to_string(), data.offset.section.to_string());
            result.insert("name".to_string(), data.name.to_string().into());
            result.insert("code".to_string(), data.code.to_string());
            result.insert("msil".to_string(), data.msil.to_string());
            result.insert("function".to_string(), data.function.to_string());
            results.push(result);
        }
    }
    Ok(results)
}

// Function to search symbols using regex
#[pyfunction]
fn search_symbols(pdb_path: &str, pattern: &str) -> PyResult<Vec<HashMap<String, String>>> {
    let file =
        File::open(&Path::new(pdb_path)).map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    let mut pdb = PDB::open(file).map_err(map_pdb_error)?;
    let symbol_table = pdb.global_symbols().map_err(map_pdb_error)?;
    let mut results = Vec::new();

    let regex = Regex::new(pattern).map_err(|e| PyRuntimeError::new_err(e.to_string()))?;

    let mut symbols = symbol_table.iter();
    while let Some(symbol) = symbols.next().map_err(map_pdb_error)? {
        if let SymbolData::Public(data) = symbol.parse().map_err(map_pdb_error)? {
            if regex.is_match(&data.name.to_string()) {
                let mut result = HashMap::new();
                result.insert("offset".to_string(), data.offset.offset.to_string());
                result.insert("section".to_string(), data.offset.section.to_string());
                result.insert("name".to_string(), data.name.to_string().into());
                result.insert("code".to_string(), data.code.to_string());
                result.insert("msil".to_string(), data.msil.to_string());
                result.insert("function".to_string(), data.function.to_string());
                results.push(result);
            }
        }
    }

    Ok(results)
}

// Define the Python module
#[pymodule]
fn symparsepy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_pdb, m)?)?;
    m.add_function(wrap_pyfunction!(search_symbols, m)?)?;
    Ok(())
}
