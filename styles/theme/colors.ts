import { useIntensityColor } from "./color"
import { Theme, ThemeColor } from "./config"
import { Intensity } from "./intensity"

interface ColorFunctions {
  [themeColor: string]: (intensity: Intensity) => string
}

/**
 * Returns a set of functions that can be used to get a color from the theme.
 *
 * Get a specific color using a theme color name and an intensity:
 *
 * ```ts
 * const color = useColors(theme)
 * const background = color.accent(80)
 * ```
 */
export function useColors(theme: Theme): ColorFunctions {
  const functions: ColorFunctions = {}
  for (const c in theme.colors) {
    const themeColor = c as ThemeColor
    functions[themeColor] = (intensity: Intensity) =>
      useIntensityColor(theme, themeColor, intensity)
  }
  return functions
}
