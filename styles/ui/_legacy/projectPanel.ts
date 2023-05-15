import { labelButton } from "@components/button"
import { useSurfaceIntensity, useSurfaceStyle } from "@components/surface"
import { Theme } from "@theme"
import { interactiveContainerStyle } from "@theme/container/interactiveContainer"
import { iconStyle } from "@theme/icon/icon"
import { addToIntensitySet, intensity } from "@theme/intensity/intensity"
import { margin, padding } from "@theme/properties"
import { interactiveTextStyle, size, textStyle } from "@theme/text/text"

export default function projectPanel(theme: Theme) {
  const ICON_SPACING: Readonly<number> = 8
  const ITEM_HEIGHT: Readonly<number> = 24
  const INDENT_WIDTH: Readonly<number> = 16

  const panelStyle = useSurfaceStyle(theme, "panel")
  const panelIntensity = useSurfaceIntensity(theme, "panel")

  const itemContainer = interactiveContainerStyle({
    theme,
    options: {
      intensitySet: panelIntensity,
    },
  })

  const itemText = interactiveTextStyle(theme, {
    fontSize: size.md,
    intensity: intensity.secondary,
  })

  const ignoredItemText = textStyle(theme, {
    fontSize: size.md,
    intensity: intensity.hidden,
  })

  const itemIcon = iconStyle({
    theme,
    iconSize: "sm",
    intensity: intensity.secondary,
  })

  const activeItemContainer = interactiveContainerStyle({
    theme,
    options: {
      themeColor: "accent",
      intensitySet: addToIntensitySet(panelIntensity, 10),
    },
  })

  const activeItemText = interactiveTextStyle(theme, {
    fontSize: size.md,
    intensity: intensity.primary,
  })

  const openProjectButton = labelButton({
    theme,
  })

  const legacy_entry = {
    height: ITEM_HEIGHT,
    icon_color: itemIcon.color,
    icon_size: itemIcon.size,
    icon_spacing: ICON_SPACING,
    background: itemContainer.default.background,
    ...itemText.default,
    hover: {
      background: itemContainer.hovered.background,
    },
    active: {
      background: activeItemContainer.default.background,
      ...activeItemText.default,
    },
    active_hover: {
      background: activeItemContainer.hovered.background,
      ...activeItemText.hovered,
    },
  }

  const legacy_properties = {
    background: panelStyle.background,
    padding: padding(12, 6),
    indent_width: INDENT_WIDTH,
    entry: legacy_entry,
    ignored_entry: {
      ...legacy_entry,
      text: ignoredItemText,
    },
    openProjectButton: {
      background: openProjectButton.default.container.background,
      border: openProjectButton.default.container.border,
      corner_radius: openProjectButton.default.container.borderRadius,
      margin: margin(16),
      padding: padding(7, 3),
      ...openProjectButton.default.text,
      hover: {
        ...openProjectButton.hovered.text,
        background: openProjectButton.hovered.container.background,
        border: openProjectButton.hovered.container.border,
      },
    },
  }

  const static_properties = {
    dragged_entry: {
      height: 24,
      icon_color: "#838994",
      icon_size: 8,
      icon_spacing: 8,
      text: {
        family: "Zed Mono",
        size: 14,
        color: "#c8ccd4",
      },
      background: "#282c34e6",
      border: {
        color: "#363c46",
        width: 1,
      },
    },
    cut_entry: {
      height: 24,
      icon_color: "#838994",
      icon_size: 8,
      icon_spacing: 8,
      text: {
        family: "Zed Mono",
        color: "#545862",
        size: 14,
      },
      hover: {
        background: "#363c46",
      },
      active: {
        background: "#454a56",
        text: {
          family: "Zed Mono",
          size: 14,
          color: "#545862",
        },
      },
      active_hover: {
        background: "#454a56",
        text: {
          family: "Zed Mono",
          size: 14,
          color: "#c8ccd4",
        },
      },
    },
    filename_editor: {
      background: "#282c34",
      text: {
        family: "Zed Mono",
        size: 14,
        color: "#c8ccd4",
      },
      selection: {
        selection: "#74ade83d",
        cursor: "#74ade8",
      },
    },
  }

  return {
    ...static_properties,
    ...legacy_properties,
  }
}
