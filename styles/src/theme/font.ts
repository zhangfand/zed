// The StyleisticSet type represents the stylistic set feature tags in OpenType fonts (ss01 to ss20)
interface StyleisticSets {
    ss01: boolean;
    ss02: boolean;
    ss03: boolean;
    ss04: boolean;
    ss05: boolean;
    ss06: boolean;
    ss07: boolean;
    ss08: boolean;
    ss09: boolean;
    ss10: boolean;
    ss11: boolean;
    ss12: boolean;
    ss13: boolean;
    ss14: boolean;
    ss15: boolean;
    ss16: boolean;
    ss17: boolean;
    ss18: boolean;
    ss19: boolean;
    ss20: boolean;
}

interface OpenTypeFeatures extends StyleisticSets {
    /** Access All Alternates: Selects a glyph from a set of alternate glyphs */
    aalt: boolean;
    /** Contextual Alternates: Applies a second substitution feature based on a match of a character pattern within a context of surrounding patterns */
    calt: boolean;
    /** Glyph Composition/Decomposition: Replaces a sequence of glyphs with a single glyph which is preferred for typographic purposes */
    ccmp: boolean;
    /** Case-Sensitive Forms: Shifts various punctuation marks up to a position that works better with all-capital sequences */
    case: boolean;
    /** Capital Spacing: Adjusts inter-glyph spacing for all-capital text */
    cpsp: boolean;
    /** Fractions: Replaces figures separated by a slash with diagonal fractions */
    frac: boolean;
    /** Kerning: Adjusts amount of space between glyphs, generally to provide optically consistent spacing between glyphs */
    kern: boolean;
    /** Standard Ligatures: Replaces a sequence of glyphs with a single glyph which is preferred for typographic purposes */
    liga: boolean;
    /** Mark Positioning: Positions mark glyphs in relation to a base glyph */
    mark: boolean;
    /** Mark to Mark Positioning: Positions one mark glyph in relation to another mark glyph */
    mkmk: boolean;
    /** Ordinals: Replaces default alphabetic glyphs with the corresponding ordinal forms for use after figures */
    ordn: boolean;
    /** Proportional Figures: Replaces figure glyphs set on uniform (tabular) widths with corresponding glyphs set on proportional widths */
    pnum: boolean;
    /** Stylistic Alternates: Replaces a glyph with an alternate version for stylistic purposes */
    salt: boolean;
    /** Subscript: Replaces default glyphs with subscript glyphs */
    subs: boolean;
    /** Superscript: Replaces default glyphs with superscript glyphs */
    sups: boolean;
    /** Swash: Replaces default glyphs with swash glyphs for stylistic purposes */
    swsh: boolean;
    /** Titling: Replaces default glyphs with titling glyphs for use in large-size settings */
    titl: boolean;
    /** Tabular Figures: Replaces figure glyphs set on proportional widths with corresponding glyphs set on uniform (tabular) widths */
    tnum: boolean;
    /** Slashed Zero: Replaces default zero with a slashed zero for better distinction between "0" and "O" */
    zero: boolean;
}

interface FontConfig extends Partial<OpenTypeFeatures> {
    /** The font family, without a weight.
    *
    * Correct: Fira Code, Inter, Garamond
    *
    * Incorrect: Fira Code Regular, Inter Medium, Garamond Bold
    */
    family: string;
    /** Font size in pixels */
    size?: number;
    /** Numeric font weight.
    *
    * `100`: Thin
    *
    * `200`: Extra Light / Ultra Light
    *
    * `300`: Light
    *
    * `400`: Normal / Regular
    *
    * `500`: Medium
    *
    * `600`: Semi Bold / Demi Bold
    *
    * `700`: Bold
    *
    * `800`: Extra Bold / Ultra Bold
    *
    * `900`: Black / Heavy
    */
    weight?: number;
    italic?: boolean;
    /** The height from one baseline to the next. This determines the space between two lines. */
    lineHeight?: number;
    /** Letter spacing or Tracking in %.
    *
    * A modifier that changes the space between letters.
    *
    * Example: `-1.8` would reduce the letter spacing by 1.8 percent of the default */
    letterSpacing?: number;
}

// Ensure a font read by the Rust code always has all properties
type Font = Required<FontConfig>

interface DefaultFonts {
    ui: Font,
    buffer: Font,
    terminal: Font,
    prose: Font
}

const fontDefault: Omit<Font, "family"> = {
    size: 15,
    weight: 400,
    italic: false,
    lineHeight: 17,
    letterSpacing: 0,
    aalt: false,
    calt: false,
    ccmp: false,
    case: false,
    cpsp: false,
    frac: false,
    kern: false,
    liga: false,
    mark: false,
    mkmk: false,
    ordn: false,
    pnum: false,
    salt: false,
    subs: false,
    sups: false,
    swsh: false,
    titl: false,
    tnum: false,
    zero: false,
    ss01: false,
    ss02: false,
    ss03: false,
    ss04: false,
    ss05: false,
    ss06: false,
    ss07: false,
    ss08: false,
    ss09: false,
    ss10: false,
    ss11: false,
    ss12: false,
    ss13: false,
    ss14: false,
    ss15: false,
    ss16: false,
    ss17: false,
    ss18: false,
    ss19: false,
    ss20: false,
}

export const defaultFont: DefaultFonts = {
    ui: {
        ...fontDefault,
        family: "Inter",
        size: 14,
        lineHeight: 15,
        letterSpacing: -1.8,
        zero: true
    },
    buffer: {
        ...fontDefault,
        family: "Fira Code",
    },
    prose: {
        ...fontDefault,
        family: "Inter",
        size: 16,
        lineHeight: 18,
        case: true,
        frac: true,
        tnum: true,
        zero: true
    },
    terminal: {
        ...fontDefault,
        family: "Fira Code",
        calt: false
    }
}
