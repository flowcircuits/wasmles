mod linear_equations;
mod matrix;

use crate::linear_equations::{
    solve_linear_equation_clusters, solve_linear_equations, LinearEquation,
};
use serde_wasm_bindgen::Error;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn solveLinearEquations(equations_value: JsValue) -> Result<JsValue, Error> {
    console_error_panic_hook::set_once();
    let equations: Result<Vec<LinearEquation>, Error> =
        serde_wasm_bindgen::from_value(equations_value);
    let solution: Option<HashMap<String, f64>> = match equations {
        Ok(equations) => solve_linear_equations(&equations),
        Err(_) => Some(HashMap::new()),
    };
    serde_wasm_bindgen::to_value(&solution)
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn solveLinearEquationClusters(clusters_value: JsValue) -> Result<JsValue, Error> {
    console_error_panic_hook::set_once();
    let clusters: Result<Vec<Vec<LinearEquation>>, Error> =
        serde_wasm_bindgen::from_value(clusters_value);
    let solution: HashMap<String, f64> = match clusters {
        Ok(clusters) => solve_linear_equation_clusters(&clusters),
        Err(_) => HashMap::new(),
    };
    let solution_value: Result<JsValue, Error> = serde_wasm_bindgen::to_value(&solution);
    solution_value
}
