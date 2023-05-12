import { Theme } from "@theme"
import { interactiveTextStyle, size } from "@theme/text/text"
import { margin } from "@theme/properties"
import { containedText } from "@theme/container"
import { iconButton } from "@components/button"
import { popoverContainerStyle } from "@components/popover"

export default function notificationStyle(theme: Theme) {
    const HEADER_PADDING = 8 as const

    const container = popoverContainerStyle({ theme })

    const message = containedText({
        theme,
        options: {
            fontSize: size.xs,
            margin: margin(0, HEADER_PADDING),
        },
    })

    const close = iconButton({
        theme,
        componentName: "notificationCloseButton",
    })

    const cta = interactiveTextStyle(theme, {
        fontSize: size.xs,
        margin: margin(6, 0, 6, HEADER_PADDING),
    })

    return {
        container,
        message,
        close,
        cta,
    }
}
