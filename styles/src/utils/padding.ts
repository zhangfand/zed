import { Padding } from "../types";

export function padding(padding: number): Padding;
export function padding(vertical: number, horizontal: number): Padding;
export function padding(
  top: number,
  right: number,
  bottom: number,
  left: number
): Padding;
export function padding(
  a: number,
  b: number = a,
  c: number = b,
  d: number = c
): Padding {
  return {
    top: a,
    right: b,
    bottom: c,
    left: d,
  };
}
