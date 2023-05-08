import { Theme } from "@theme"
import notification from "@components/notification"

export default function simpleMessageNotification(theme: Theme) {
  const n = notification(theme)

  const legacy_properties = {
    message: n.message,
    dismissButton: {
      color: n.close.default.icon.color,
      iconWidth: n.close.default.icon.size,
      iconHeight: n.close.default.icon.size,
      buttonWidth: n.close.default.icon.size,
      buttonHeight: n.close.default.icon.size,
      hover: {
        color: n.close.hovered.icon.color,
      },
    },
    actionMessage: {
      ...n.cta.default.text,
      hover: {
        color: n.cta.hovered.text.color,
      },
    },
  }
  return {
    ...legacy_properties,
  }
}
