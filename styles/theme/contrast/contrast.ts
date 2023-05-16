import { Intensity } from "@theme"

export const MINIMUM_CONTRAST: Readonly<number> = 3
export const LOW_CONTRAST: Readonly<number> = 4.5

export const checkContrast = (
  name: string,
  background: Intensity,
  foreground: Intensity,
  debug?: boolean
) => {
  const foregroundIntensity = Math.max(foreground, background) + 0.05
  const backgroundIntensity = Math.min(foreground, background) + 0.05
  const contrastRatio = foregroundIntensity / backgroundIntensity

  // Return a contrast with 2 decimal places
  const contrast = +contrastRatio.toFixed(2)

  debug &&
    console.log(
      `Contrast on ${name}: ${contrast}. Foreground: ${foreground}, Background: ${background}`
    )

  if (contrast < LOW_CONTRAST) {
    console.log(`Constrast on ${name} may be too low: ${contrast}`)
  }

  if (contrast < MINIMUM_CONTRAST) {
    throw new Error(`Constrast on ${name} is too low: ${contrast}`)
  }
}
