interface TextProperties {
    style: Style
    weight: Weight
    stretch: Stretch
}

type Style = "Normal" | "Italic" | "Oblique"

interface Weights {
    thin: number
    extraLight: number
    light: number
    normal: number
    medium: number
    semibold: number
    bold: number
    extraBold: number
    black: number
}

export type Weight = keyof Weights

interface FontStretchTypes {
    ultraCondensed: number
    extraCondensed: number
    condensed: number
    semiCondensed: number
    normal: number
    semiExpanded: number
    expanded: number
    extraExpanded: number
    ultraExpanded: number
}

export type Stretch = keyof FontStretchTypes

export interface Font {
    font_family_name: string
    font_size: number
    font_properties: TextProperties
}

export interface TextStyle extends Font {
    color: Color
    underline: Underline
}

export interface TextLabelWithHighlight {
    text: TextStyle
    text_highlight: HighlightStyle
}

export interface HighlightStyle {
    color?: Color
    weight?: Weight
    italic?: boolean
    underline?: Underline
    fade_out?: number
}

export interface Underline {
    color?: Color
    thickness: number
    squiggly: boolean
}

export const fontWeight: Weights = {
    thin: 100,
    extraLight: 200,
    light: 300,
    normal: 400,
    medium: 500,
    semibold: 600,
    bold: 700,
    extraBold: 800,
    black: 900,
}

export const fontStretch: FontStretchTypes = {
    ultraCondensed: 0.5,
    extraCondensed: 0.625,
    condensed: 0.75,
    semiCondensed: 0.875,
    normal: 1.0,
    semiExpanded: 1.125,
    expanded: 1.25,
    extraExpanded: 1.5,
    ultraExpanded: 2.0,
}
