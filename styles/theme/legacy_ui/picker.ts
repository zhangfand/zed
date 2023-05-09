import { Theme } from "@theme"
import { containedText } from "@theme/container"
import { popoverContainerStyle } from "@theme/container/popover"
import { intensity } from "@theme/intensity"
import { margin, padding } from "@theme/properties"
import { interactiveTextStyle, textStyle } from "@theme/text"

// TODO: picker -> palette, move to components
export default function picker(theme: Theme) {
  const container = popoverContainerStyle({ theme })

  // TODO: You shouldn't need to specify the color here
  const itemStyle = interactiveTextStyle(
    theme,
    {
      ...textStyle(theme, { intensity: intensity.secondary }),
      color: "neutral",
      padding: padding(4, 12),
      margin: margin(1, 4, 0, 4),
      borderRadius: 8
    }
  )

  const activeItem = interactiveTextStyle(
    theme,
    {
      ...textStyle(theme),
      color: "accent",
      weight: 700,
      padding: padding(4, 12),
      margin: margin(1, 4, 0, 4),
      borderRadius: 8
    }
  )

  // TODO: Make an input component with placeholder text
  const input = interactiveTextStyle(theme, {
    padding: padding(16, 8),
    margin: margin(0, 0, 4, 0),
  })

  const highlightText = textStyle(theme, {
    weight: 700,
    color: "accent",
  })

  const noMatches = containedText({
    theme,
    options: {
      intensity: intensity.secondary,
      padding: padding(16, 8)
    }
  })

  const legacy_properties = {
    inputEditor: {
      ...input,
      placeholderText: textStyle(theme, { intensity: intensity.inactive }),
      // TODO: Add player selection color
      selection: '#FF0000',
    },
    emptyInputEditor: {
      ...input,
      placeholderText: textStyle(theme, { intensity: intensity.inactive }),
      // TODO: Add player selection color
      selection: '#FF0000',
    },
    emptyContainer: {
      ...container,
      padding: padding(0),
    },
    item: {
      ...itemStyle,
      highlightText,
      active: activeItem,
    }

  }
  return {
    ...legacy_properties,
    container,
    noMatches,
  }
}
