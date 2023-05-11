import { useColors } from "./colors"
import { Theme, ThemeColor } from "./config"
import { transparentColor } from "./properties/opacity"

interface Player {
  color: string
  selection: string
  border: string
  cursor: string
}

const buildPlayer = (theme: Theme, color: ThemeColor): Player => {
  const colors = useColors(theme)

  const selection = transparentColor({
    theme,
    color,
    intensity: 50,
    opacity: 0.12,
  })

  const player = {
    color: colors[color](50),
    border: colors[color](20),
    cursor: colors[color](50),
    selection,
  }

  return player
}

// TODO: Add other player colors
const players = (theme: Theme): Player[] => {
  const host = buildPlayer(theme, "accent")

  return [host]
}

export { Player, players }
