import { Color } from "chroma-js"
import { Effect } from "./Effect"


export interface Size {
  width: number,
  height: number
}

export interface Padding {
  top: number, bottom: number, left: number, right: number
}

export interface Margin {
  top: number, bottom: number, left: number, right: number
}

export interface Border {
  fill: Color,
  width: number,
  style: "SOLID" | "DASHED" | "WAVY"
}

export type CornerRadius = number | { top: number, bottom: number, left: number, right: number }

export interface Flex {
  direction: "VERTICAL" | "HORIZONTAL"
  /** The spacing between two elements in the primary flex direction */
  spacing: number,
}

/** A ring is used to show a focus state, or a follower */
export interface Ring {
  fill: Color,
  width: number,
  offset: number
}

export interface Container {
  size: Size,
  padding: Padding
  margin: Margin
  cornerRadius: CornerRadius
  border: Border
  ring: Ring
  flex: Flex
  effect: Effect
}
