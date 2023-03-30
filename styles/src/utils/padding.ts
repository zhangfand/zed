import { Padding } from "../types";

export function padding(margin: number): Padding;
export function padding(horizontal: number, vertical: number): Padding;
export function padding(top: number, right: number, bottom: number, left: number): Padding;
export function padding(a: number, b: number = a, c: number = b, d: number = a): Padding {
    return {
        top: a,
        left: d,
        bottom: c,
        right: b,
    };
}
