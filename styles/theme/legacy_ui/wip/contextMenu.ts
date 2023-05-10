import { Theme } from "@theme"
import { containedText, containerStyle } from "@theme/container"
import { popoverContainerStyle } from "@theme/container/popover"
import { margin, padding } from "@theme/properties"
import { interactiveTextStyle, weight } from "@theme/text"

export default function contextMenu(theme: Theme) {
    const KEYSTROKE_MARGIN: Readonly<number> = 30
    const ICON_SIZE: Readonly<number> = 8
    const ICON_SPACING: Readonly<number> = 14

    const container = popoverContainerStyle({ theme })

    const item = interactiveTextStyle(theme, {
        padding: padding(6, 2),
        borderRadius: 6,
    })

    const keystroke = containedText({
        theme,
        options: {
            weight: weight.bold,
        },
    })

    const separator = containerStyle({
        background: container.border.color,
        margin: margin(0, 2),
    })

    const legacy_properties = {
        keystrokeMargin: KEYSTROKE_MARGIN,
        iconWidth: ICON_SIZE,
        iconSpacing: ICON_SPACING,
        item: {
            ...item,
            keystroke,
            active: {
                background: item.hovered.container.background,
            },
            activeHover: {
                background: item.hovered.container.background,
            },
        },
    }

    return {
        ...legacy_properties,
        ...container,
        separator,
    }
}
