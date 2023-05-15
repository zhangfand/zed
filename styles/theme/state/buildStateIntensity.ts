import { checkContrast } from "@theme/contrast/contrast"
import { IntensitySet, calculateIntensity } from "@theme/intensity/intensity"

export function buildStateIntensity(
  componentName: string,
  name: string,
  startingIntensity: IntensitySet,
  change?: number
): IntensitySet {
  if (!change) {
    return startingIntensity
  }

  const stateIntensity: IntensitySet = {
    bg: calculateIntensity(startingIntensity.bg, change),
    border: calculateIntensity(startingIntensity.border, change),
    fg: calculateIntensity(startingIntensity.fg, change),
  }

  const nameForCheck = `${componentName} ${name}`

  checkContrast(nameForCheck, startingIntensity.bg, stateIntensity.fg)

  return stateIntensity
}
