import { Intensity } from "./intensity"

// The primary intensities that should be used for components and UI.
// Readonly is used to prevent the intensities from being modified.
interface SemanticIntensities {
    primary: Readonly<Intensity>
    secondary: Readonly<Intensity>
    inactive: Readonly<Intensity>
    hidden: Readonly<Intensity>
    disabled: Readonly<Intensity>
}
const PRIMARY_INTENSITY: Readonly<Intensity> = 100
const SECONDARY_INTENSITY: Readonly<Intensity> = 75
const INACTIVE_INTENSITY: Readonly<Intensity> = 50
const HIDDEN_INTENSITY: Readonly<Intensity> = 40
const DISABLED_INTENSITY: Readonly<Intensity> = 30

/**
 * Semantic intensities are used to define specific intensities, like the value of primary and secondary text, the color of a disabled button, etc.
 *
 * `default`: The default intensity for a given color. This is the intensity that should be used for most text and UI elements.
 *
 * Dos:
 * - Use default intensity for most text and UI elements.
 *
 * Don'ts:
 * - Use default intensity for indicating an element is active or selected. Instead, use a combonation of a background and border change, or a shift in the color.
 *
 * `secondary`: A lower intensity that should be used for secondary text and UI elements.
 *
 * Dos:
 * - Use for secondary labels, helper text, information that should be less visually prominent.
 * - Use for details, subheadings, secondary buttons, etc.
 *
 * Don'ts:
 * - Use for primary text or main UI elements that require more emphasis.
 * - Don't use for inactive elements,
 *
 * `inactive`: The lowest non-disabled intensity that should be used for UI elements.
 *
 * Dos:
 * - Use for placeholder text
 * - Use for inactive UI elements, like inactive tabs, inactive menu items, etc.
 *
 * Don'ts:
 * - Use for visual differentiation for text or UI elements that are not inactive.
 *
 * `disabled`: The intensity that should be used for disabled text and UI elements, indicating the user cannot interact with them.
 *
 * Dos:
 * - Use for disabled button text, disabled form elements, disabled menu items.
 * - Use to clearly signal a user cannot interact with an element.
 *
 * Don'ts:
 * - Use for anything a user can interact with.
 */
const semanticIntensities: SemanticIntensities = {
    primary: PRIMARY_INTENSITY,
    secondary: SECONDARY_INTENSITY,
    inactive: INACTIVE_INTENSITY,
    hidden: HIDDEN_INTENSITY,
    disabled: DISABLED_INTENSITY,
}

export { semanticIntensities }
