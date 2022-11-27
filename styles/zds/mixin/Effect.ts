export interface Shadow {
  x: number,
  y: number,
  blur: number,
  opacity: number,
  color: string
}

export interface Effect {
  shadow: Shadow | Shadow[]
}