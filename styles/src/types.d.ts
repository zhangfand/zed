export interface Border {
    width: number
    color: string
    overlay: boolean
    top: boolean
    right: boolean
    bottom: boolean
    left: boolean
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
    shadow: Shadow | null
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
    italic: boolean | null
    underline: Underline | null
    fade_out: number | null
}
export interface ImageStyle {
    border: Border
    corner_radius: number
    height: number | null
    width: number | null
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
    calt: boolean | null
    case: boolean | null
    cpsp: boolean | null
    frac: boolean | null
    liga: boolean | null
    onum: boolean | null
    ordn: boolean | null
    pnum: boolean | null
    ss01: boolean | null
    ss02: boolean | null
    ss03: boolean | null
    ss04: boolean | null
    ss05: boolean | null
    ss06: boolean | null
    ss07: boolean | null
    ss08: boolean | null
    ss09: boolean | null
    ss10: boolean | null
    ss11: boolean | null
    ss12: boolean | null
    ss13: boolean | null
    ss14: boolean | null
    ss15: boolean | null
    ss16: boolean | null
    ss17: boolean | null
    ss18: boolean | null
    ss19: boolean | null
    ss20: boolean | null
    subs: boolean | null
    sups: boolean | null
    swsh: boolean | null
    titl: boolean | null
    tnum: boolean | null
    zero: boolean | null
}

export interface TextStyle {
    color: string
    font_family_name: string
    font_family_id: string
    font_id: string
    font_size: number
    font_properties: { style: string; weight: number; stretch: number }
    underline: Underline
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
