// TODO: Colors should be ZDSColor or Chroma color, not string

// interface LayoutMixin {

// }

export interface ZDSShadow {
  x: number,
  y: number,
  blur: number,
  opacity: number,
  color: string
}

export interface ZDSComponentElement {
  color: string,
  border: {
    radius: number,
    color: string
  }
  shadow: ZDSShadow
}

export interface ZDSText {
  text: string,
  color: string,
  family: string,
  size: number,
  weight: string,
  lineHeight: number,
  tracking: number,
}

export interface ZDSIcon {
  color: string,
  size: number,
  style: "filled" | "outline"
}