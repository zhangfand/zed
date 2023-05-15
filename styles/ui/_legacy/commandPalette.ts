import {
    relativeIntensityToSurface,
    useSurfaceIntensity,
} from "@components/surface"
import { useColors } from "@theme/color/colors"
import { Theme } from "@theme/config"
import { containedText, container } from "@theme/container"
import { ContainerStyle } from "@theme/container/containerStyle"
import { intensity } from "@theme/intensity/intensity"
import { padding } from "@theme/properties"
import * as text from "@theme/text/text"

export default function commandPalette(theme: Theme) {
    const color = useColors(theme)
    const surface = useSurfaceIntensity(theme, "pane")

    const keyContainer: ContainerStyle = {
        ...container.blank,
        borderRadius: 2,
        padding: padding(6, 1),
        margin: padding(1, 0, 1, 2),
    }

    const key = containedText({
        theme,
        options: {
            fontSize: text.size.xs,
            intensity: intensity.secondary,
            ...keyContainer,
        },
    })

    const active_key_text = text.textStyle(theme, {
        intensity: intensity.primary,
        fontSize: text.size.xs,
    })

    const active_key_background_intensity = relativeIntensityToSurface(
        surface.bg,
        10
    )

    // TODO: This shouldn't be a static color
    const active_key_background = color.neutral(active_key_background_intensity)

    const legacy_properties = {
        keystrokeSpacing: 8,
        // Should be key, active_key
        key: {
            text: key.text,
            ...key.container,
            cornerRadius: key.container.borderRadius,
            active: {
                text: active_key_text,
                background: active_key_background,
            },
        },
    }

    return {
        ...legacy_properties,
    }
}
