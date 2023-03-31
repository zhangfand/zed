import { background, border, text } from "../styleTree/components"
import { Layer } from "../themes/common/colorScheme"
import { ContainedText, InteractiveContainer } from "../types"

export function button(layer: Layer): InteractiveContainer {
    const base: ContainedText = {
        corner_radius: 4,
        background: background(layer, "on"),
        border: border(layer, "on", { "overlay": true }),
        padding: {
            top: 5,
            bottom: 5,
            left: 6,
            right: 6
        },
        ...text(layer, "sans", "on", "default", {
            size: "xs",
        })
    }

    const hovered: ContainedText = {
        ...base,
        background: background(layer, "on", "hovered"),
    }

    const clicked: ContainedText = {
        ...base,
        background: background(layer, "on", "pressed"),
    }

    const el: InteractiveContainer = {
        ...base,
        hover: hovered,
        clicked: clicked,
    }

    return el
}
