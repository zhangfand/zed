import { useColors } from "@theme/color/colors"
import { Theme } from "@theme/config"
import { containedText } from "@theme/container"
import { popoverContainerStyle } from "@components/popover"
import { intensity } from "@theme/intensity/intensity"
import { margin, padding } from "@theme/properties"
import { size, textStyle, weight } from "@theme/text/text"

export default function tooltip(theme: Theme) {
  // TODO: Remove this
  const color = useColors(theme)

  const text = textStyle(theme, {
    fontSize: size.sm,
  })

  const container = popoverContainerStyle({
    theme,
    options: {
      padding: padding(8, 4),
      margin: margin(6),
      borderRadius: 6,
    },
  })

  const keystroke = containedText({
    theme,
    options: {
      intensity: intensity.secondary,
      weight: weight.bold,
      margin: margin(0, 6),
      padding: padding(4, 4),
      borderRadius: 4,
      // TODO: Don't manually assigned a color like this
      background: color.neutral(10),
    },
  })

  const legacy_properties = {
    maxTextWidth: 200,
    keystroke: {
      ...keystroke.container,
      ...keystroke.text,
      corner_radius: keystroke.container.borderRadius,
    },
  }

  return {
    ...legacy_properties,
    ...container,
    corner_radius: container.borderRadius,
    text,
  }
}
