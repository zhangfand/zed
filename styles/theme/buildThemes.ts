import * as themeConfigs from "@/themes"
import { buildUI } from "@/ui"
import { buildTheme } from "./buildTheme"
import { EXPORT_PATH, exportData } from "@/lib/export"
import legacy_ui from "@theme/legacy_ui"

export function buildThemes(): void {
  // Make an array of themes
  const themes = [];
  for (const themeConfig of Object.values(themeConfigs)) {
    // ThemeConfig => Theme
    const theme = buildTheme(themeConfig)

    const ui = buildUI(theme)

    const styles = {
      ...legacy_ui(theme),
      ui: ui,
    }

    const json = JSON.stringify(styles)
    themes.push({ name: theme.name, json: json, path: EXPORT_PATH })
  }
  // Pass the array of themes to the export function
  // Which will clear the target directory and write the themes
  exportData(themes);
}
