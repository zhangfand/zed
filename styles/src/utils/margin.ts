import { Margin } from "../types"

export function margin(margin: number): Margin
export function margin(vertical: number, horizontal: number): Margin
export function margin(
    top: number,
    bottom: number,
    right: number,
    left: number
): Margin
export function margin(
    a: number,
    b: number = a,
    c: number = b,
    d: number = a
): Margin {
    return {
        top: a,
        bottom: d,
        left: c,
        right: b,
    }
}
