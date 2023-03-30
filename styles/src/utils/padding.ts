import { Padding } from "../types"

export function padding(padding: number): Padding
export function padding(vertical: number, horizontal: number): Padding
export function padding(
    top: number,
    bottom: number,
    right: number,
    left: number
): Padding
export function padding(
    a: number,
    b: number = a,
    c: number = b,
    d: number = a
): Padding {
    return {
        top: a,
        bottom: d,
        left: c,
        right: b,
    }
}
