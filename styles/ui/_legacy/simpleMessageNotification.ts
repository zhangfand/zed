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
      ...notification.cta.default.container,
      hover: {
        ...notification.cta.hovered.text,
        ...notification.cta.hovered.container,
      },
    },
    message: {
      ...notification.message.text,
      ...notification.message.container,
    }
  }
  return {
    ...legacy_properties,
  }
}
