import { labelButton } from "@components/button"
import notificationStyle from "@components/notification"
import { Theme } from "@theme"
import { intensity } from "@theme/intensity/intensity"
import { textStyle } from "@theme/text/text"

export default function incomingCallNotification(theme: Theme) {
    const AVATAR_SIZE: Readonly<number> = 48
    const WINDOW_WIDTH: Readonly<number> = 380
    const WINDOW_HEIGHT: Readonly<number> = 74
    const CALLER_CONTAINER_PADDING: Readonly<number> = 12
    const CALLER_USERNAME_MARGIN_TOP: Readonly<number> = -3
    const CALLER_METADATA_MARGIN_LEFT: Readonly<number> = 10
    const CALLER_MESSAGE_MARGIN_TOP: Readonly<number> = -3
    const WORKTREE_ROOTS_MARGIN_TOP: Readonly<number> = -3
    const BUTTON_WIDTH: Readonly<number> = 96

    const notification = notificationStyle(theme)
    const primaryText = textStyle(theme, {
        weight: "bold",
    })
    const secondaryText = textStyle(theme, { intensity: intensity.secondary })
    // TODO: Extend label button to allow theming these
    const acceptButton = labelButton({
        theme,
        componentName: "incomingCallAcceptButton",
    })
    const declineButton = labelButton({
        theme,
        componentName: "incomingCallDeclineButton",
    })

    const legacy_properties = {
        windowHeight: WINDOW_HEIGHT,
        windowWidth: WINDOW_WIDTH,
        background: notification.container.background,
        callerContainer: {
            padding: CALLER_CONTAINER_PADDING,
        },
        callerAvatar: {
            height: AVATAR_SIZE,
            width: AVATAR_SIZE,
            cornerRadius: AVATAR_SIZE / 2,
        },
        callerMetadata: {
            margin: { left: CALLER_METADATA_MARGIN_LEFT },
        },
        callerUsername: {
            ...primaryText,
            margin: { top: CALLER_USERNAME_MARGIN_TOP },
        },
        callerMessage: {
            ...secondaryText,
            margin: { top: CALLER_MESSAGE_MARGIN_TOP },
        },
        worktreeRoots: {
            ...secondaryText,
            margin: { top: WORKTREE_ROOTS_MARGIN_TOP },
        },
        buttonWidth: BUTTON_WIDTH,
        acceptButton: {
            background: acceptButton.default.container.background,
            border: acceptButton.default.container.border,
            ...acceptButton.default.text,
        },
        declineButton: {
            background: declineButton.default.container.background,
            border: declineButton.default.container.border,
            ...declineButton.default.text,
        },
    }

    return {
        ...legacy_properties,
        message: notification.message,
    }
}
