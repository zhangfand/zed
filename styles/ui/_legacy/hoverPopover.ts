import { Theme } from "@theme"
import { popoverContainerStyle } from "@components/popover"
import { margin, padding } from "@theme/properties"
import { shadow } from "@theme/properties/shadow"
import { textStyle } from "@theme/text/text"
import { transparentColor } from "@theme/properties/opacity"

export default function HoverPopover(theme: Theme) {
  const popverOptions = {
    borderRadius: 8,
    padding: padding(8, 4),
    margin: margin(0, 0, 0, -8),
    shadow: shadow(theme, "popover"),
  }

  const diagnosticSourceHighlight = textStyle(theme, {
    themeColor: "accent",
  })

  const legacy_properties = {
    blockStyle: {
      padding: { top: 4 },
    },
    // Should be a full text style
    // This is the prefix that shows which language server the diagnostic is from
    diagnosticSourceHighlight: {
      color: diagnosticSourceHighlight.color,
    },
    // This is the highlight color of the item you are hovering over
    highlight: transparentColor({
      theme,
      themeColor: "accent",
      intensity: 50,
      opacity: 0.4,
    }),
  }

  return {
    ...legacy_properties,
    container: popoverContainerStyle({
      theme,
      options: popverOptions,
    }),
    infoContainer: popoverContainerStyle({
      theme,
      color: "accent",
      options: popverOptions,
    }),
    warningContainer: popoverContainerStyle({
      theme,
      color: "warning",
      options: popverOptions,
    }),
    errorContainer: popoverContainerStyle({
      theme,
      color: "error",
      options: popverOptions,
    }),
    prose: textStyle(theme),
  }
}
