use crate::matrix::solve_matrix;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct LinearEquation {
    variables: HashMap<String, f64>,
    constant: f64,
}

pub fn solve_linear_equations(equations: &[LinearEquation]) -> Option<HashMap<String, f64>> {
    let mut num_variables = 0;
    let mut variable_index_map: HashMap<String, usize> = HashMap::new();
    let mut rows: Vec<Vec<f64>> = Vec::new();
    let mut vector: Vec<Vec<f64>> = Vec::new();

    // Populate index map
    for equation in equations {
        for key in equation.variables.keys() {
            let index = variable_index_map.get(key).cloned();
            if index.is_none() {
                variable_index_map.insert(key.clone(), num_variables);
                num_variables += 1;
            }
        }
    }

    // Transform to array of ordered variable keys
    let variables_array: Vec<String> = variable_index_map
        .into_iter()
        .sorted_by_key(|(_, index)| *index)
        .map(|(key, _)| key)
        .collect();

    // Populate matrix and vector
    for equation in equations {
        let row: Vec<f64> = variables_array
            .iter()
            .map(|key| *equation.variables.get(key).unwrap_or(&0.0))
            .collect();
        let constant = equation.constant;
        rows.push(row);
        vector.push(vec![constant]);
    }

    let x = solve_matrix(rows, vector);

    match x {
        None => None,
        Some(x) => {
            let mut solution: HashMap<String, f64> = HashMap::new();
            for (i, key) in variables_array.iter().enumerate() {
                solution.insert(key.clone(), x[i][0]);
            }
            Some(solution)
        }
    }
}

pub fn solve_linear_equation_clusters(clusters: &[Vec<LinearEquation>]) -> HashMap<String, f64> {
    let mut solution: HashMap<String, f64> = HashMap::new();

    for equations in clusters {
        let cluster_solution = solve_linear_equations(equations);
        if let Some(cluster_solution) = cluster_solution {
            solution.extend(cluster_solution);
        }
    }

    solution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_linear_equation() {
        let equations = vec![
            LinearEquation {
                variables: [("a".to_string(), 1.0)].iter().cloned().collect(),
                constant: 0.0,
            },
            LinearEquation {
                variables: [("a".to_string(), 1.0)].iter().cloned().collect(),
                constant: 0.1,
            },
        ];
        let solution = solve_linear_equations(&equations);
        assert!(solution.is_none());
    }

    #[test]
    fn test_solve_linear_equation_clusters() {
        let equations1 = vec![
            LinearEquation {
                variables: [("a".to_string(), 1.0)].iter().cloned().collect(),
                constant: 10.0,
            },
            LinearEquation {
                variables: [("a".to_string(), 1.0), ("b".to_string(), 2.0)]
                    .iter()
                    .cloned()
                    .collect(),
                constant: 0.0,
            },
            LinearEquation {
                variables: [("b".to_string(), 1.0), ("c".to_string(), 1.0)]
                    .iter()
                    .cloned()
                    .collect(),
                constant: 0.0,
            },
        ];
        let equations2 = vec![LinearEquation {
            variables: [("d".to_string(), 1.0), ("e".to_string(), 2.0)]
                .iter()
                .cloned()
                .collect(),
            constant: 0.0,
        }];
        let equations3 = vec![LinearEquation {
            variables: [("z".to_string(), 1.0)].iter().cloned().collect(),
            constant: 42.0,
        }];
        let equations4 = vec![
            LinearEquation {
                variables: [("conflict".to_string(), 1.0)].iter().cloned().collect(),
                constant: -21.0,
            },
            LinearEquation {
                variables: [("conflict".to_string(), 1.0)].iter().cloned().collect(),
                constant: -1.0,
            },
        ];
        let clusters = vec![equations1, equations2, equations3, equations4];

        let solution = solve_linear_equation_clusters(&clusters);

        assert_eq!(solution["a"], 10.0);
        assert_eq!(solution["b"], -5.0);
        assert_eq!(solution["c"], 5.0);
        assert_eq!(solution["z"], 42.0);
        assert!(solution.get("conflict").is_none());
        assert!(solution.get("d").is_none());
        assert!(solution.get("e").is_none());

        let equations1 = vec![
            LinearEquation {
                variables: [("Q (044f1290-c386-42ac-a531-e6334029fd7c)".to_string(), 1.0)]
                    .iter()
                    .cloned()
                    .collect(),
                constant: 0.0,
            },
            LinearEquation {
                variables: [("Q (044f1290-c386-42ac-a531-e6334029fd7c)".to_string(), 1.0)]
                    .iter()
                    .cloned()
                    .collect(),
                constant: 0.2,
            },
        ];
        let equations2 = vec![
            LinearEquation {
                variables: [("Q (864a016f-542e-4254-87f3-59dde11539b6)".to_string(), 1.0), ("Q (8446d267-c847-43d0-9dca-5d9a88d0b35b)->(864a016f-542e-4254-87f3-59dde11539b6)".to_string(), 1.0)]
                    .iter()
                    .cloned()
                    .collect(),
                constant: 0.0,
            },
            LinearEquation {
                variables: [("P (864a016f-542e-4254-87f3-59dde11539b6)".to_string(), 1.0)]
                    .iter()
                    .cloned()
                    .collect(),
                constant: 0.0,
            },
            LinearEquation {
                variables: [("Q (864a016f-542e-4254-87f3-59dde11539b6)->(8446d267-c847-43d0-9dca-5d9a88d0b35b)".to_string(), 1.0)]
                    .iter()
                    .cloned()
                    .collect(),
                constant: 0.0,
            },
            LinearEquation {
                variables: [("Q (8446d267-c847-43d0-9dca-5d9a88d0b35b)->(864a016f-542e-4254-87f3-59dde11539b6)".to_string(), 1.0), ("Q (864a016f-542e-4254-87f3-59dde11539b6)->(8446d267-c847-43d0-9dca-5d9a88d0b35b)".to_string(), 1.0)]
                    .iter()
                    .cloned()
                    .collect(),
                constant: 0.0,
            },
            LinearEquation {
                variables: [("P (8446d267-c847-43d0-9dca-5d9a88d0b35b)".to_string(), 1.0), ("P (864a016f-542e-4254-87f3-59dde11539b6)".to_string(), -1.0), ("Q (8446d267-c847-43d0-9dca-5d9a88d0b35b)->(864a016f-542e-4254-87f3-59dde11539b6)".to_string(), -0.000013558774142754068)]
                    .iter()
                    .cloned()
                    .collect(),
                constant: 0.0,
            }
        ];
        let clusters = vec![equations1, equations2];
        let solution = solve_linear_equation_clusters(&clusters);
        for (key, value) in &solution {
            println!("{}: {}", key, value);
        }
    }
}
