import { Theme } from "@theme"
import { iconSize, iconStyle } from "@theme/icon"
import { intensity } from "@theme/intensity"
import { interactiveTextStyle, size, textStyle } from "@theme/text"

export default function projectPanel(theme: Theme) {
  const ICON_SPACING: Readonly<number> = 8
  const ITEM_HEIGHT: Readonly<number> = 24

  const baseIcon = iconStyle({
    theme,
    iconSize: "sm",
    intensity: intensity.secondary
  })

  const defaultText = interactiveTextStyle(
    theme,
    {
      fontSize: size.sm
    })



  const itemMixin = {
    height: ITEM_HEIGHT,
    iconColor: baseIcon.color,
    iconSize: baseIcon.size,
    iconSpacing: ICON_SPACING,
  }

  const legacy_properties = {
    entry: {
      ...itemMixin,
      text: textStyle(theme, {
        fontSize: size.sm
      },
    }
  }

  let entry = {
    ...baseEntry,
    text: text(layer, "mono", "variant", { size: "sm" }),
    hover: {
      background: background(layer, "variant", "hovered"),
    },
    active: {
      background: colorScheme.isLight
        ? withOpacity(background(layer, "active"), 0.5)
        : background(layer, "active"),
      text: text(layer, "mono", "active", { size: "sm" }),
    },
    activeHover: {
      background: background(layer, "active"),
      text: text(layer, "mono", "active", { size: "sm" }),
    },
  }

  return {
    openProjectButton: {
      background: background(layer),
      border: border(layer, "active"),
      cornerRadius: 4,
      margin: {
        top: 16,
        left: 16,
        right: 16,
      },
      padding: {
        top: 3,
        bottom: 3,
        left: 7,
        right: 7,
      },
      ...text(layer, "sans", "default", { size: "sm" }),
      hover: {
        ...text(layer, "sans", "default", { size: "sm" }),
        background: background(layer, "hovered"),
        border: border(layer, "active"),
      },
    },
    background: background(layer),
    padding: { left: 12, right: 12, top: 6, bottom: 6 },
    indentWidth: 8,
    entry,
    draggedEntry: {
      ...baseEntry,
      text: text(layer, "mono", "on", { size: "sm" }),
      background: withOpacity(background(layer, "on"), 0.9),
      border: border(layer),
    },
    ignoredEntry: {
      ...entry,
      text: text(layer, "mono", "disabled"),
    },
    cutEntry: {
      ...entry,
      text: text(layer, "mono", "disabled"),
      active: {
        background: background(layer, "active"),
        text: text(layer, "mono", "disabled", { size: "sm" }),
      },
    },
    filenameEditor: {
      background: background(layer, "on"),
      text: text(layer, "mono", "on", { size: "sm" }),
      selection: colorScheme.players[0],
    },
  }
}
