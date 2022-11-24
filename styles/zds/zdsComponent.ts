// TODO: Colors should be ZDSColor or Chroma color, not string

export interface ZDSComponentElement {
  color: string,
  border: {
    radius: number,
    color: string
  }
  shadow: {
    x: number,
    y: number,
    blur: number,
    opacity: number,
    color: string
  }
}

export interface ZDSText {
  color: string,
  family: string,
  size: number,
  weight: string,
  lineHeight: string,
  tracking: string,
}

export interface ZDSIcon {
  color: string,
  size: number,
  style: "filled" | "outline"
}