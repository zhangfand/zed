import { Theme } from "@theme"

import json from "./static_json/workspace.json"
import { useSurfaceIntensity, useSurfaceStyle } from "@components/surface"
import { borderStyle } from "@theme/properties/border"
import { padding } from "@theme/properties"
import { statusBar } from "./status_bar"

// VERY WIP
export default function workspace(theme: Theme) {
  // const LOGO_SIZE: Readonly<number> = 256

  const surface = useSurfaceStyle(theme, "background")
  const pane = useSurfaceStyle(theme, "pane")

  // Move to tab_bar.ts
  const TAB_BAR_HEIGHT: Readonly<number> = 32

  const tab_bar = {
    ...json.tab_bar,
    height: TAB_BAR_HEIGHT,
    background: pane.background,
  }

  // Move to title_bar.ts
  const ITEM_SPACING: Readonly<number> = 8
  const FACEPILE_SPACING: Readonly<number> = 2

  const title_bar_surface_style = useSurfaceStyle(theme, "background")
  const title_bar_surface_intensity = useSurfaceIntensity(theme, "background")

  const title_bar = {
    ...json.titlebar,
    item_spacing: ITEM_SPACING,
    face_pile_spacing: FACEPILE_SPACING,
    // Account for the bottom border
    height: TAB_BAR_HEIGHT + 1,
    background: title_bar_surface_style.background,
    border: borderStyle({
      theme,
      intensity: title_bar_surface_intensity.border,
      options: {
        position: "bottom",
      },
    }),
    padding: padding(0, 8, 0, 80),
  }

  const legacy_properties = {
    background: surface.background,
    blank_pane: json.blank_pane,
    tab_bar,
    titlebar: title_bar,
    status_bar: statusBar(theme),
  }

  return {
    ...json,
    ...legacy_properties,
    pane_divider: surface.border,
  }
}
