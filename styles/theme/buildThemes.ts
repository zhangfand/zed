import * as themeConfigs from "@/themes"
import { buildUI } from "@/ui"
import { buildTheme } from "./buildTheme"
import { EXPORT_PATH, writeToDisk } from "@/lib/export"
import { writeTokens } from "./tokens"
import legacy_ui from "@theme/legacy_ui"

export function buildThemes(): void {
    for (const themeConfig of Object.values(themeConfigs)) {
        // ThemeConfig => Theme
        const theme = buildTheme(themeConfig)

        const ui = buildUI(theme)

        // Write outputs
        writeTokens(theme.name)

        const styles = {
            ...legacy_ui(theme),
            ui: ui,
        }

        const json = JSON.stringify(styles)
        writeToDisk(theme.name, json, EXPORT_PATH)
    }
}
