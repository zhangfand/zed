import { useSurfaceStyle } from "@components/surface"
import { Theme } from "@theme"
import { textStyle } from "@theme/text/text"

export default function terminal(theme: Theme) {
  const pane = useSurfaceStyle(theme, "pane")
  const text = textStyle(theme)

  /**
   * Colors are controlled per-cell in the terminal grid.
   * Cells can be set to any of these more 'theme-capable' colors
   * or can be set directly with RGB values.
   * Here are the common interpretations of these names:
   * https://en.wikipedia.org/wiki/ANSI_escape_code#Colors
   */
  return {
    black: "#0f172a",
    red: "#b91c1c",
    green: "#15803d",
    yellow: "#ca8a04",
    blue: "#1e40af",
    magenta: "#5b21b6",
    cyan: "#0891b2",
    white: "#e2e8f0",
    brightBlack: "#020617",
    brightRed: "#ef4444",
    brightGreen: "#22c55e",
    brightYellow: "#facc15",
    brightBlue: "#2563eb",
    brightMagenta: "#7c3aed",
    brightCyan: "#22d3ee",
    brightWhite: "#f8fafc",
    /**
     * Default color for characters
     */
    foreground: text.color,
    /**
     * Default color for the rectangle background of a cell
     */
    background: pane.background,
    modalBackground: pane.background,
    /**
     * Default color for the cursor
     */
    cursor: "#2563eb",
    dimBlack: "#020617",
    dimRed: "#450a0a",
    dimGreen: "#166534",
    dimYellow: "#854d0e",
    dimBlue: "#1e40af",
    dimMagenta: "#5b21b6",
    dimCyan: "#155e75",
    dimWhite: "#cbd5e1",
    brightForeground: "#e2e8f0",
    dimForeground: "#334155",
  }
}
