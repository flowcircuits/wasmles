[![npm (tag)](https://img.shields.io/npm/v/@flowcircuits/wasmles?style=flat&colorA=000000&colorB=000000)](https://www.npmjs.com/package/@flowcircuits/wasmles) ![npm bundle size](https://img.shields.io/bundlephobia/minzip/@flowcircuits/wasmles?style=flat&colorA=000000&colorB=000000) ![NPM](https://img.shields.io/npm/l/@flowcircuits/wasmles?style=flat&colorA=000000&colorB=000000)

# @flowcircuits/wasmles

An easy to use Web Assembly linear equation solver built with Rust

## Installation

```bash
yarn add @flowcircuits/wasmles
```

## Usage

```ts
import wasmInit, { solveLinearEquations } from "@flowcircuits/wasmles";

interface LinearEquation {
    constant: number;
    variables: Record<string, number>;
}

const linearEquations: LinearEquation[] = [
    { variables: { a: 2, b: 3, c: -1 }, constant: 5 },
    { variables: { a: 1, b: -4, c: 2 }, constant: -6 },
    { variables: { a: 3, b: 2, c: 1 }, constant: 9 },
];

wasmInit().then(() => {
    const solutionMap = solveLinearEquations(linearEquations);
    const solution = Object.fromEntries(solutionMap);
    console.log(solution);
    // {
    //     a: 2,
    //     b: 1,
    //     c: 3,
    // }
});
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
