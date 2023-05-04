import { Theme } from "@theme*"
import { size } from "@theme/text"
import { margin } from "@theme/properties"
import { containedText } from "@theme/container"
import { iconButton } from "@components/button"

export default function updateNotification(theme: Theme) {
    const HEADER_PADDING = 8 as const

    const message = containedText({
        theme,
        options: {
            size: size.xs,
            margin: margin(0, HEADER_PADDING)
        }
    })

    const close = iconButton(theme)


    const legacy_properties = {
        dismissButton: {
            color: close.default.icon.color,
            iconWidth: close.default.icon.size,
            iconHeight: close.default.icon.size,
            buttonWidth: close.default.icon.size,
            buttonHeight: close.default.icon.size,
            hover: {
                color: close.hovered.icon.color,
            },
        },
    }

    return {
        ...legacy_properties,
        message,
    }
}


function legacyupdateNotification(colorScheme: ColorScheme): Object {
    let layer = colorScheme.middle
    return {
        actionMessage: {
            ...text(layer, "sans", { size: "xs" }),
            margin: { left: headerPadding, top: 6, bottom: 6 },
            hover: {
                color: foreground(layer, "hovered"),
            },
        },
    }
}
