import { Intensity, Theme, useColors } from "@theme"
import { ThemeColor } from "@theme/config"
import { resolveThemeColorIntensity } from "@theme/intensity"
import chroma from "chroma-js"

interface TransparentColorProps {
    theme: Theme
    color?: ThemeColor
    intensity: Intensity
    opacity: number
}

export function transparentColor({
    theme,
    color,
    intensity,
    opacity,
}: TransparentColorProps): string {
    const themeColor = useColors(theme)

    const resolvedColorIntensity = resolveThemeColorIntensity(theme, intensity)

    const c = chroma(
        themeColor[color ? color : "neutral"](resolvedColorIntensity)
    )
        .alpha(opacity)
        .hex()

    return c
}
