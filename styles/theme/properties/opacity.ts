import { Intensity, Theme, useColors } from "@theme"
import { ThemeColor } from "@theme/config"
import { resolveThemeColorIntensity } from "@theme/intensity/intensity"
import chroma from "chroma-js"

interface TransparentColorProps {
    theme: Theme
    themeColor?: ThemeColor
    intensity: Intensity
    opacity: number
}

export function transparentColor({
    theme,
    themeColor,
    intensity,
    opacity,
}: TransparentColorProps): string {
    const color = useColors(theme)

    const resolvedColorIntensity = resolveThemeColorIntensity(theme, intensity)

    const c = chroma(
        color[themeColor ? themeColor : "neutral"](resolvedColorIntensity)
    )
        .alpha(opacity)
        .hex()

    return c
}
