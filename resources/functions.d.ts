import { LinearEquation } from "./index";

export function solveLinearEquations<T extends Record<string, number>>(
    equations_value: LinearEquation<T>[]
): T | undefined;

export function solveLinearEquationClusters<T extends Record<string, number>>(
    clusters_value: LinearEquation<T>[][]
): T;
