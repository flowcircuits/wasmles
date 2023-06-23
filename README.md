[![npm (tag)](https://img.shields.io/npm/v/@flowcircuits/wasmles?style=flat&colorA=000000&colorB=000000)](https://www.npmjs.com/package/@flowcircuits/wasmles) ![npm bundle size](https://img.shields.io/bundlephobia/minzip/@flowcircuits/wasmles?style=flat&colorA=000000&colorB=000000) ![NPM](https://img.shields.io/npm/l/@flowcircuits/wasmles?style=flat&colorA=000000&colorB=000000)

# @flowcircuits/wasmles

An easy to use Web Assembly linear equation solver built with Rust

## Installation

```bash
yarn add @flowcircuits/wasmles
```

## Usage

#### Browser

```ts
import wasmInit, {
    solveLinearEquations,
    LinearEquation,
} from "@flowcircuits/wasmles/browser";

const linearEquations: LinearEquation[] = [
    { variables: { a: 2, b: 8, c: -1 }, constant: 6 },
    { variables: { a: 1, b: -4, c: 2 }, constant: -6 },
    { variables: { a: 4, b: 2, c: 1 }, constant: 10 },
];

wasmInit().then(() => {
    const solutionMap = solveLinearEquations(linearEquations);
    const solution = Object.fromEntries(solutionMap);
    console.log(solution);
    // {
    //     a: 6,
    //     b: -2,
    //     c: -10,
    // }
});
```

#### Node

```ts
import {
    solveLinearEquations,
    LinearEquation,
} from "@flowcircuits/wasmles/node";

const linearEquations: LinearEquation[] = [
    { variables: { a: 2, b: 8, c: -1 }, constant: 6 },
    { variables: { a: 1, b: -4, c: 2 }, constant: -6 },
    { variables: { a: 4, b: 2, c: 1 }, constant: 10 },
];

const solutionMap = solveLinearEquations(linearEquations);
const solution = Object.fromEntries(solutionMap);
console.log(solution);
// {
//     a: 6,
//     b: -2,
//     c: -10,
// }
```

## Development

#### Build

```bash
yarn build
```

#### Test

```bash
yarn test
```
