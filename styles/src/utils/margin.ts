import { Margin } from "../types";

export function margin(margin: number): Margin;
export function margin(horizontal: number, vertical: number): Margin;
export function margin(top: number, right: number, bottom: number, left: number): Margin;
export function margin(a: number, b: number = a, c: number = b, d: number = a): Margin {
    return {
        top: a,
        left: d,
        bottom: c,
        right: b,
    };
}
