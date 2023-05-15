import { Intensity, Theme, ThemeColor, ThemeColors, ThemeConfig } from "@theme"
import chroma, { Color, Scale } from "chroma-js"

export { chroma, Color, Scale }

/** Builds the Theme's colors from the each of the ThemeConfig's InputColors */
export function buildThemeColors(
  themeConfig: ThemeConfig
): Readonly<ThemeColors> {
  const scales: ThemeColors = {} as ThemeColors

  for (const [colorName, colorValue] of Object.entries(themeConfig.colors)) {
    const name = colorName as ThemeColor
    scales[name] = chroma.scale(colorValue).colors(100)
  }

  // Ensure the theme colors can't be mutated
  return Object.freeze(scales) as Readonly<ThemeColors>
}

export function useIntensityColor(
  theme: Theme,
  themeColor: ThemeColor,
  intensity: Intensity
): string {
  if (!theme.color) {
    throw new Error("useIntensityColor: Theme has no colors provided")
  }

  if (intensity < 1 || intensity > 100) {
    throw new Error(
      `useIntensityColor: Intensity must be between 1 and 100, received ${intensity}`
    )
  }

  const scale = theme.color[themeColor]
  // scale is 0 to 100, intensity is 1 to 100
  // so we need to subtract 1 from intensity
  const c = scale[intensity - 1]
  return c
}
