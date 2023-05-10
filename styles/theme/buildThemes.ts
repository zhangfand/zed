import * as themeConfigs from "@/themes"
import { buildUI } from "@/ui"
import { buildTheme } from "./buildTheme"
import { EXPORT_PATH, exportData } from "@/lib/export"
import legacy_ui from "@theme/legacy_ui"
import snakeCaseTree from "@lib/snakeCase"

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

    // Format the styles the way Zed will consume them
    const formattedStyles = snakeCaseTree(styles)

    themes.push({ name: theme.name, json: formattedStyles, path: EXPORT_PATH })
  }
  // Pass the array of themes to the export function
  // Which will clear the target directory and write the themes
  exportData(themes);
}
