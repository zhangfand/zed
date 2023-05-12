import { Theme } from "@/theme"
import { buildStateIntensity } from "@theme/state/buildStateIntensity"
import { ElementState } from "."
import {
  Intensity,
  IntensitySet,
  resolveIntensitySet,
} from "@theme/intensity/intensity"

export type StateIntensities = Partial<Record<ElementState, IntensitySet>>

export function buildIntensitiesForStates(
  theme: Theme,
  name: string,
  startingIntensity: IntensitySet
): StateIntensities {
  /** Returns a StateIntensitySet for each default state */
  const light = theme.appearance === "light"
  const multiplier = light ? 1 : 1.2
  const stepSize = 5
  const startingOffset = light ? 5 : 8
  const intensitySteps = [0, 1, 2, 3].map(
    (step) => multiplier * stepSize * step + startingOffset
  )

  const scaleFactor = theme.intensity.scaleFactor

  const scaledIntensitySteps = intensitySteps.map(
    (intensity) => intensity * scaleFactor
  )

  const resolvedIntensity = resolveIntensitySet(theme, startingIntensity)

  const defaultState = {
    bg: resolvedIntensity.bg,
    border: resolvedIntensity.border,
    fg: resolvedIntensity.fg,
  }

  const disabledState: IntensitySet = {
    bg: 5 as Intensity,
    border: 10 as Intensity,
    fg: 35 as Intensity,
  }

  const elementStates = {
    default: buildStateIntensity(name, "default", defaultState),
    hovered: buildStateIntensity(
      name,
      "hovered",
      defaultState,
      scaledIntensitySteps[1]
    ),
    pressed: buildStateIntensity(
      name,
      "pressed",
      defaultState,
      scaledIntensitySteps[2]
    ),
    disabled: disabledState,
  }

  return elementStates
}
