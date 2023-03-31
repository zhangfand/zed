export interface Border {
    width: number
    color: string
    overlay?: boolean
    top?: boolean
    right?: boolean
    bottom?: boolean
    left?: boolean
}
export interface Margin {
    top: number
    left: number
    bottom: number
    right: number
}
export interface Padding {
    top: number
    left: number
    bottom: number
    right: number
}
export interface ContainerStyle {
    margin: Margin
    padding: Padding
    background: string
    overlay: string
    border: Border
    corner_radius: number
    shadow?: Shadow
    cursor:
    | "Arrow"
    | "ResizeLeftRight"
    | "ResizeUpDown"
    | "PointingHand"
    | "IBeam"
}

export interface HighlightStyle {
    color: string
    weight: string
    italic?: boolean
    underline?: Underline
    fade_out?: number
}
export interface ImageStyle {
    border: Border
    corner_radius: number
    height: number
    width: number
    grayscale: boolean
}

export interface Shadow {
    offset: [number, number]
    blur: number
    color: string
}
export type Spacing =
    | number
    | { top: number; left: number; bottom: number; right: number }

export interface Features {
    calt: boolean
    case: boolean
    cpsp: boolean
    frac: boolean
    liga: boolean
    onum: boolean
    ordn: boolean
    pnum: boolean
    ss01: boolean
    ss02: boolean
    ss03: boolean
    ss04: boolean
    ss05: boolean
    ss06: boolean
    ss07: boolean
    ss08: boolean
    ss09: boolean
    ss10: boolean
    ss11: boolean
    ss12: boolean
    ss13: boolean
    ss14: boolean
    ss15: boolean
    ss16: boolean
    ss17: boolean
    ss18: boolean
    ss19: boolean
    ss20: boolean
    subs: boolean
    sups: boolean
    swsh: boolean
    titl: boolean
    tnum: boolean
    zero: boolean
}

export interface TextStyle {
    color: string
    font_family_name: string
    font_family_id: string
    font_id: string
    font_size: number
    font_properties: { style: string; weight: number; stretch: number }
    underline: Underline | boolean
}
export interface Underline {
    color: string
    thickness: number
    squiggly: boolean
}

export type Container = Partial<ContainerStyle>
export type Text = Partial<TextStyle>
export type Image = Partial<ImageStyle>
export interface ContainedText extends Container, Text { }

export interface InteractiveContainer extends ContainedText {
    hover: ContainedText,
    clicked: ContainedText,
    disabled?: ContainedText
}
