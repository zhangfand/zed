import chroma from "chroma-js"
import { Theme } from "../config"
import { players } from "../player/player"

// NEXT: Get from TS-RS
export interface SelectionStyle {
  cursor: string
  selection: string
}

// TOOD: Allow use for players other than p1
export function selectionStyle(theme: Theme): SelectionStyle {
  const player = players(theme)[0]
  const selection = chroma(player.color).alpha(0.1).hex()

  return {
    cursor: player.color,
    selection,
  }
}
