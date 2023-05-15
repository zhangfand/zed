import { labelButton } from "@components/button"
import notificationStyle from "@components/notification"
import { Theme } from "@theme"
import { intensity } from "@theme/intensity/intensity"
import { textStyle } from "@theme/text/text"

export default function projectSharedNotification(theme: Theme) {
  const AVATAR_SIZE: Readonly<number> = 48
  const WINDOW_WIDTH: Readonly<number> = 380
  const WINDOW_HEIGHT: Readonly<number> = 74
  const OWNER_CONTAINER_PADDING: Readonly<number> = 12
  const OWNER_USERNAME_MARGIN_TOP: Readonly<number> = -3
  const OWNER_METADATA_MARGIN_LEFT: Readonly<number> = 10
  const WORKTREE_ROOTS_MARGIN_TOP: Readonly<number> = -3
  const BUTTON_WIDTH: Readonly<number> = 96

  const notification = notificationStyle(theme)
  const primaryText = textStyle(theme, {
    weight: "bold",
  })
  const secondaryText = textStyle(theme, { intensity: intensity.secondary })
  const openButton = labelButton({
    theme,
    componentName: "projectSharedOpenButton",
  })
  const dismissButton = labelButton({
    theme,
    componentName: "projectSharedDismissButton",
  })

  const legacy_properties = {
    windowHeight: WINDOW_HEIGHT,
    windowWidth: WINDOW_WIDTH,
    background: notification.container.background,
    ownerContainer: {
      padding: OWNER_CONTAINER_PADDING,
    },
    ownerAvatar: {
      height: AVATAR_SIZE,
      width: AVATAR_SIZE,
      cornerRadius: AVATAR_SIZE / 2,
    },
    ownerMetadata: {
      margin: { left: OWNER_METADATA_MARGIN_LEFT },
    },
    ownerUsername: {
      ...primaryText,
      margin: { top: OWNER_USERNAME_MARGIN_TOP },
    },
    worktreeRoots: {
      ...secondaryText,
      margin: { top: WORKTREE_ROOTS_MARGIN_TOP },
    },
    buttonWidth: BUTTON_WIDTH,
    openButton: {
      background: openButton.default.container.background,
      border: openButton.default.container.border,
      ...openButton.default.text,
    },
    dismissButton: {
      // TODO: Clean this up
      ...dismissButton.default.text,
      color: notification.close.default.icon.color,
      iconWidth: notification.close.default.icon.size,
      iconHeight: notification.close.default.icon.size,
      buttonWidth: notification.close.default.icon.size,
      buttonHeight: notification.close.default.icon.size,
      hover: {
        color: notification.close.hovered.icon.color,
      },
    },
    message: {
      ...notification.message.text,
      ...notification.message.container,
    },
  }

  return {
    ...legacy_properties,
  }
}
