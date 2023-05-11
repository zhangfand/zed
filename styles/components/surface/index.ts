import {
  Intensity,
  addToIntensitySet,
  addToIntensity,
  IntensitySet,
} from "@theme/intensity/intensity"
import { ContainerStyle } from "@theme/container/containerStyle"
import { Theme } from "@theme/config"
import { useColors } from "@theme/color/colors"
import { borderStyle } from "@theme/properties/border"
import { ElementState } from "@theme/state"

type SurfaceLevel = 0 | 1 | 2
type SurfaceName =
  | "background"
  | "panel"
  | "pane"
  | "popover"
  | "palette"
  | "tooltip"

type SurfaceLevels = Record<SurfaceName, SurfaceLevel>

type Surface = keyof SurfaceLevels

const surfaceLevel: SurfaceLevels = {
  background: 0,
  panel: 1,
  pane: 1,
  popover: 2,
  palette: 2,
  tooltip: 2,
}

type SurfaceStyle = Pick<Required<ContainerStyle>, "background" | "border">

type InteractiveSurfaceStyles = Record<ElementState, SurfaceStyle>

function useSurfaceIntensity(theme: Theme, surface: Surface): IntensitySet {
  const level = surfaceLevel[surface]

  const BASE_SURFACE_INTENSITIES: IntensitySet = {
    bg: 1,
    border: 12,
    fg: 100,
  } as const

  const intensity = BASE_SURFACE_INTENSITIES

  switch (level) {
    case 1:
      return addToIntensitySet(intensity, 10)
    case 2:
      return addToIntensitySet(intensity, 20)
    default:
      return intensity
  }
}

function buildSurfaceStyle(theme: Theme, surface: Surface): SurfaceStyle {
  const color = useColors(theme)
  const intensity = useSurfaceIntensity(theme, surface)

  const borderIntensity = intensity.border as Intensity

  return {
    background: color.neutral(intensity.bg),
    border: borderStyle({ theme, intensity: borderIntensity }),
  }
}

function buildSurfaceLevels(theme: Theme) {
  const surface = {
    background: buildSurfaceStyle(theme, "background"),
    panel: buildSurfaceStyle(theme, "panel"),
    pane: buildSurfaceStyle(theme, "pane"),
    popover: buildSurfaceStyle(theme, "popover"),
  }

  return surface
}

const useSurfaceStyle = buildSurfaceStyle

const surface = (theme: Theme) => {
  return {
    level: surfaceLevel,
    style: buildSurfaceLevels(theme),
  }
}

// Placeholder for defining element background intensity relative to surface logic
// TODO: You should be able to specific adding or subtracting intensity
function relativeIntensityToSurface(
  surfaceIntensity: Intensity,
  intensityChange: Intensity
): Intensity {
  // adjust background color based on the relative difference between surface intensity and intensityChange
  const newIntensity: Intensity = addToIntensity(
    surfaceIntensity,
    intensityChange
  )

  return newIntensity
}

export {
  Surface,
  SurfaceStyle,
  InteractiveSurfaceStyles,
  useSurfaceIntensity,
  useSurfaceStyle,
  relativeIntensityToSurface,
  surface,
}
