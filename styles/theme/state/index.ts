export interface Interactive<T = Element> {
  default: T
  hovered: T
  pressed: T
  dragged?: T
  disabled?: T
}

export interface Toggleable<T = Interactive> {
  inactive: T
  active: T
}

export type ElementState = "default" | "hovered" | "pressed" | "dragged" | "disabled"
