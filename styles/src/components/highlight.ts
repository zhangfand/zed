import { background } from "../styleTree/components"
import { ColorScheme } from "../themes/common/colorScheme"
import { withOpacity } from "../utils/color"

export const highlight = (colorScheme: ColorScheme) => {
    const layer = colorScheme.highest

    return {
        match: {
            backgroundColor: withOpacity(
                background(layer, "active"),
                colorScheme.isLight ? 0.75 : 1
            ),
        },
    }
}
