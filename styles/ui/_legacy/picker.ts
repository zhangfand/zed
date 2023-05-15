import { Theme } from "@theme"
import { containedText } from "@theme/container"
import { popoverContainerStyle } from "@components/popover"
import { addToIntensitySet, intensity } from "@theme/intensity/intensity"
import { margin, padding } from "@theme/properties"
import { selectionStyle } from "@theme/properties/selection"
import { interactiveTextStyle, textStyle } from "@theme/text/text"
import { useSurfaceIntensity } from "@components/surface"

// TODO: picker -> palette, move to components
export default function picker(theme: Theme) {
  const container = popoverContainerStyle({ theme })
  const intensitySet = useSurfaceIntensity(theme, "pane")
  const activeIntensitySet = addToIntensitySet(intensitySet, 10)

  // TODO: You shouldn't need to specify the color here
  const itemStyle = interactiveTextStyle(theme, {
    ...textStyle(theme, { intensity: intensity.secondary }),
    background: container.background,
    themeColor: "neutral",
    padding: padding(12, 4),
    margin: margin(1, 4, 0, 4),
    borderRadius: 8,
  })

  const activeItem = interactiveTextStyle(theme, {
    ...textStyle(theme, { intensity: intensity.primary }),
    themeColor: "accent",
    intensitySet: activeIntensitySet,
    weight: "bold",
    padding: padding(12, 4),
    margin: margin(1, 4, 0, 4),
    borderRadius: 8,
  })

  // TODO: Make an input component with placeholder text
  const input = interactiveTextStyle(theme, {
    padding: padding(16, 8),
    margin: margin(0, 0, 4, 0),
  })

  const highlightText = textStyle(theme, {
    weight: "bold",
    themeColor: "accent",
  })

  const noMatches = containedText({
    theme,
    options: {
      intensity: intensity.secondary,
      padding: padding(16, 8),
    },
  })

  const paletteContainer = {
    ...container,
    cornderRadius: 12,
    padding: padding(0, 0, 4, 0),
  }

  const legacy_properties = {
    inputEditor: {
      ...input.default.container,
      text: input.default.text,
      placeholderText: textStyle(theme, {
        intensity: intensity.inactive,
      }),
      // TODO: Add player selection color
      selection: selectionStyle(theme),
    },
    emptyInputEditor: {
      ...input.default.container,
      text: input.default.text,
      placeholderText: textStyle(theme, {
        intensity: intensity.inactive,
      }),
      // TODO: Add player selection color
      selection: selectionStyle(theme),
    },
    emptyContainer: {
      ...paletteContainer,
      padding: padding(0),
    },
    item: {
      ...itemStyle.default.container,
      text: itemStyle.default.text,
      highlightText,
      active: activeItem,
    },
  }
  return {
    ...paletteContainer,
    ...legacy_properties,
    noMatches,
  }
}
