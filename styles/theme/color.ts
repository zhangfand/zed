import chroma from "chroma-js"
import { Theme, ThemeColor, ThemeColors, ThemeConfig } from "./config"
import { Intensity } from "./intensity"

export type Color = chroma.Color
export type Scale = chroma.Scale
export type Scales = Record<keyof ThemeConfig["colors"], Scale>

export { chroma }

function buildScaleFromSingleColor(color: Color): Scale {
  // TODO: Don't allow single color scales
  const scale = chroma.scale([
    color.darken(1),
    color.darken(0.5),
    color,
    color.brighten(0.5),
    color.brighten(1),
  ])
  return scale
}

export function buildThemeScales(themeConfig: ThemeConfig): ThemeColors {
  const scales: Scales = {} as Scales
  for (const [colorName, colorValue] of Object.entries(themeConfig.colors)) {
    const name = colorName as keyof ThemeConfig["colors"]

    scales[name] = Array.isArray(colorValue)
      ? chroma.scale(colorValue)
      : buildScaleFromSingleColor(chroma(colorValue))
  }

  const scaleArrays: ThemeColors = {} as ThemeColors

  for (const [colorName, scale] of Object.entries(scales)) {
    const name = colorName as keyof ThemeConfig["colors"]
    scaleArrays[name] = scale.colors(100)
  }

  return scaleArrays
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
  console.log(scale, themeColor)
  const c = scale[intensity]
  return c
}
