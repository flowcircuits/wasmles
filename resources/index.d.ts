export interface LinearEquation<
    Variables extends Record<string, number> = Record<string, number>
> {
    constant: number;
    variables: Variables;
}
