/**
 * A 8 value hexidecimal color
 */
type Color = string

/**
 * Describes the style of a border for an element.
 * @property {number} width - The width of the border.
 * @property {boolean} top - Whether or not the top border should be rendered.
 * @property {boolean} left - Whether or not the left border should be rendered.
 * @property {boolean} bottom - Whether or not the bottom border should be rendered.
 * @property {boolean} right - Whether or not the right border should be rendered.
 * @property {boolean} overlay - Whether or not the border should be rendered inside the element instead of outside it. Roughly equivalent to CSS' inset property.
 * @property {Color} color - The color of the border.
 */
interface Border {
    width: number
    top: boolean
    left: boolean
    bottom: boolean
    right: boolean
    overlay: boolean
    color: Color
}

/**
 * An x,y offset, currently used only for shadows
 */
interface Offset {
    x: number
    y: number
}

/**
 * Represents the shadow of an element, including its offset, blur amount, and color.
 */
interface Shadow {
    offset: Offset
    blur: number
    color: Color
}

/**
 * Describes the margin around an element, specifying the size of the margin on each side.
 */
interface Margin {
    top: number
    left: number
    bottom: number
    right: number
}

/**
 * Describes the padding inside an element, specifying the size of the padding on each side.
 */
interface Padding {
    top: number
    left: number
    bottom: number
    right: number
}

/**
 * @typedef {Object} Container
 * @property {Margin} margin - Describes the margin around an element, specifying the size of the margin on each side.
 * @property {Padding} padding - Describes the padding inside an element, specifying the size of the padding on each side.
 * @property {Color} [background_color] - The background color of the container.
 * @property {Color} [overlay_color] - The overlay color of the container.
 * @property {Border} border - Describes the style of a border for an element.
 * @property {number} corner_radius - The corner radius of the container.
 * @property {Shadow} [shadow] - Represents the shadow of an element, including its offset, blur amount, and color.
 * @property {string} [cursor] - The cursor style of the container.
 */
interface Container {
    margin: Margin
    padding: Padding
    background_color?: Color
    overlay_color?: Color
    border: Border
    corner_radius: number
    shadow?: Shadow
    cursor?: string
}
