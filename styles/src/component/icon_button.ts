import { neutral } from "../color"
import { interactive, toggleable } from "../element"
import { Theme, Layer } from "../theme"
import { Button } from "./button"

export type Margin = {
    top: number
    bottom: number
    left: number
    right: number
}

interface IconButtonOptions {
    layer?:
    | Theme["lowest"]
    | Theme["middle"]
    | Theme["highest"]
    color?: keyof Theme["lowest"]
    margin?: Partial<Margin>
    variant?: Button.Variant
    size?: Button.Size
}

type ToggleableIconButtonOptions = IconButtonOptions & {
    active_color?: keyof Theme["lowest"]
    active_layer?: Layer
}

export function icon_button({ color, margin, size }: IconButtonOptions = {
    variant: Button.variant.Default,
    size: Button.size.Medium,
}) {
    // const theme = useTheme()

    if (!color) color = "base"

    // const background_color = variant === Button.variant.Ghost ? null : background(layer ?? theme.lowest, color)

    const m = {
        top: margin?.top ?? 0,
        bottom: margin?.bottom ?? 0,
        left: margin?.left ?? 0,
        right: margin?.right ?? 0,
    }

    const padding = {
        top: size === Button.size.Small ? 2 : 2,
        bottom: size === Button.size.Small ? 2 : 2,
        left: size === Button.size.Small ? 2 : 4,
        right: size === Button.size.Small ? 2 : 4,
    }

    return interactive({
        base: {
            corner_radius: 6,
            padding: padding,
            margin: m,
            icon_width: 12,
            icon_height: 14,
            button_width: size === Button.size.Small ? 16 : 20,
            button_height: 14,
        },
        state: {
            default: {
                background: neutral.surface,
                color: neutral.foreground,
            },
            hovered: {
                background: neutral.hover,
                color: neutral.foreground,
            },
            clicked: {
                background: neutral.pressed,
                color: neutral.foreground,
            },
        },
    })
}

export function toggleable_icon_button(
    theme: Theme,
    { color, active_color, margin, variant, size, active_layer }: ToggleableIconButtonOptions
) {
    if (!color) color = "base"

    return toggleable({
        state: {
            inactive: icon_button({ color, margin, variant, size }),
            active: icon_button({
                color: active_color ? active_color : color,
                margin,
                layer: active_layer,
                size
            }),
        },
    })
}
