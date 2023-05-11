import { useColors } from "./colors"
import { Theme, ThemeColor } from "./config"
import { Intensity } from "./intensity"

export type Size = "sm" | "md" | "lg"

type Sizes = Record<Size, number>

export const iconSize: Sizes = {
  sm: 7,
  md: 11,
  lg: 15,
} as const

interface IconProps {
  theme: Theme
  iconSize: Size
  intensity?: Intensity
  themeColor?: ThemeColor
}

const DEFAULT_ICON_INTENSITY: Intensity = 100 as const

/**
 * Get an iconStyle from an icon size and intensity.
 *
 * Optionally, a color can be specified.
 *
 * If no color is specified, neutral is used.
 */
export const iconStyle = ({
  theme,
  iconSize: size,
  intensity = DEFAULT_ICON_INTENSITY,
  themeColor = "neutral",
}: IconProps): IconStyle => {
  const color = useColors(theme)

  return {
    color: color[themeColor](intensity),
    size: iconSize[size]
  }
}

export interface IconStyle {
  color: string
  size: number
}
