import { Theme } from "@theme"
import notificationStyle from "@components/notification"

export default function simpleMessageNotification(theme: Theme) {
    const notification = notificationStyle(theme)

    const legacy_properties = {
        dismissButton: {
            color: notification.close.default.icon.color,
            iconWidth: notification.close.default.icon.size,
            iconHeight: notification.close.default.icon.size,
            buttonWidth: notification.close.default.icon.size,
            buttonHeight: notification.close.default.icon.size,
            hover: {
                color: notification.close.hovered.icon.color,
            },
        },
        actionMessage: {
            ...notification.cta.default.text,
            hover: {
                color: notification.cta.hovered.text.color,
            },
        },
    }
    return {
        ...legacy_properties,
        message: notification.message,
    }
}
