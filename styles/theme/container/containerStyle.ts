import { Border, borderStyle } from "@theme/properties/border"
import { Margin, Padding } from "@theme/properties"
import { Shadow } from "@theme/properties/shadow"
import { Theme, ThemeColor, useColors } from "@theme"
import { IntensitySet } from "@theme/intensity/intensity"

export interface ContainerStyle {
  background?: string
  margin?: Margin
  padding?: Padding
  borderRadius?: number
  border?: Border
  width?: number
  height?: number
  shadow?: Shadow
}

export interface ContainerOptions extends Partial<ContainerStyle> {
  themeColor?: ThemeColor
  intensitySet?: IntensitySet
}

export const DEFAULT_CONTAINER_INTENSITY_SET: Readonly<IntensitySet> = {
  bg: 1,
  border: 15,
  fg: 100,
}

export const DEFAULT_CONTAINER_OPTIONS: ContainerOptions = {
  themeColor: "neutral",
  intensitySet: DEFAULT_CONTAINER_INTENSITY_SET,
} as const

interface ContainerProps {
  theme: Theme,
  options?: Partial<ContainerOptions>
}

export function containerStyle({ theme, options }: ContainerProps): ContainerStyle {
  const color = useColors(theme)

  const mergedOptions = {
    ...DEFAULT_CONTAINER_OPTIONS,
    ...options,
  }

  let background

  if (options.background) {
    background = mergedOptions.background
  } else {
    background = color[mergedOptions.themeColor](mergedOptions.intensitySet.bg)
  }

  let border

  // TODO: This isn't done
  // Border needs to be extended to allow a theme color to be specified
  if (options.themeColor) {
    border = borderStyle({
      theme,
      intensity: mergedOptions.intensitySet.border,
    })
  } else {
    border = mergedOptions.border
  }

  // TODO: This was a spread of merged options
  // But text options from containedText were leaking in
  // Need to figure out what was happening there
  return {
    width: mergedOptions.width,
    height: mergedOptions.height,
    background,
    margin: mergedOptions.margin,
    padding: mergedOptions.padding,
    borderRadius: mergedOptions.borderRadius,
    border,
    shadow: mergedOptions.shadow,
  }
}
