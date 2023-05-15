import { labelButton } from "@components/button"
import notificationStyle from "@components/notification"
import { Theme } from "@theme"
import { containedText } from "@theme/container"
import { containerStyle } from "@theme/container/containerStyle"
import { margin } from "@theme/properties"
import { size } from "@theme/text/text"

export default function contactNotification(theme: Theme) {
  const AVATAR_SIZE: Readonly<number> = 12
  const HEADER_PADDING: Readonly<number> = 8
  const HEADER_HEIGHT: Readonly<number> = 18
  const ICON_SIZE: Readonly<number> = 8
  const BUTTON_SIZE: Readonly<number> = 8

  const notification = notificationStyle(theme)

  const headerAvatar = containerStyle({
    theme,
    options: {
      height: AVATAR_SIZE,
      width: AVATAR_SIZE,
      borderRadius: 6,
    },
  })

  const headerMessage = containedText({
    theme,
    options: {
      fontSize: size.xs,
      margin: margin(HEADER_PADDING, 0),
    },
  })

  const button = labelButton({
    theme,
    componentName: "contactNotificationButton",
  })

  const bodyMessage = containedText({
    theme,
    options: {
      fontSize: size.xs,
      margin: margin(6, 0, 6, AVATAR_SIZE + HEADER_PADDING),
    },
  })
  const legacy_properties = {
    headerMessage: {
      ...headerMessage.text,
      margin: headerMessage.container.margin,
    },
    headerHeight: HEADER_HEIGHT,
    bodyMessage: {
      ...bodyMessage.text,
      margin: bodyMessage.container.margin,
    },
    dismissButton: {
      color: notification.close.default.icon.color,
      iconWidth: ICON_SIZE,
      iconHeight: ICON_SIZE,
      buttonWidth: BUTTON_SIZE,
      buttonHeight: BUTTON_SIZE,
      hover: {
        color: notification.close.hovered.icon.color,
      },
    },
    button: {
      ...button.default.text,
      ...button.default.container,
      margin: margin(0, 0, 0, 8),
      hover: {
        background: button.hovered.container.background,
      },
    },
  }

  return {
    ...legacy_properties,
    headerAvatar,
  }
}
