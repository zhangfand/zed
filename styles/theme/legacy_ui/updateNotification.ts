import { Theme } from "@theme*"
import { interactiveTextStyle, size } from "@theme/text"
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


    // This should be actionMessage when the legacy properties are removed
    // This should probably be renamed with a more descriptive name
    const actionMessageStyle = interactiveTextStyle(
        theme,
        {
            size: size.xs,
            margin: margin(6, 0, 6, HEADER_PADDING)
        }
    )

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
        actionMessage: {
            ...actionMessageStyle.default.text,
            hover: {
                color: actionMessageStyle.hovered.text.color,
            }
        }
    }

    return {
        ...legacy_properties,
        message,
    }
}
