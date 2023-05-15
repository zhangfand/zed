import { Surface, useSurfaceIntensity } from "@components/surface"
import { Theme } from "../config"
import chroma from "chroma-js"

export interface Shadow {
    blur: number
    color: string
    offset: number[]
}

export function shadow(theme: Theme, surface: Surface): Shadow {
    const DEFAULT_SHADOW_BLUR = 16 as const

    const intensity = useSurfaceIntensity(theme, surface)

    let shadowAlpha: number
    let offsetX: number
    let offsetY: number

    switch (surface) {
        case "popover":
            shadowAlpha = 0.12 as const
            offsetX = 1
            offsetY = 2
            break
        default:
            shadowAlpha = 0.12 as const
            offsetX = 1
            offsetY = 4
            break
    }

    const blur = DEFAULT_SHADOW_BLUR
    const color = chroma(theme.color.neutral[intensity.bg])
        .alpha(shadowAlpha)
        .hex()
    const offset = [offsetX, offsetY]

    return {
        blur,
        color,
        offset,
    }
}
