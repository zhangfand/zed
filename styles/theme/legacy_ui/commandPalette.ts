import {
    relativeIntensityToSurface,
    useSurfaceIntensity,
} from "@components/surface"
import { Theme } from "@theme/config"
import { containedText, container } from "@theme/container"
import { ContainerStyle } from "@theme/container"
import { intensity } from "@theme/intensity"
import { padding } from "@theme/padding"
import * as text from "@theme/text"

export default function commandPalette(theme: Theme) {
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
            size: text.size.xs,
            intensity: intensity.secondary,
            ...keyContainer,
        },
    })

    const active_key_text = text.textStyle(theme, {
        intensity: intensity.primary,
        size: text.size.xs,
    })

    const active_key_background = relativeIntensityToSurface(surface.bg, 10)

    const legacy_properties = {
        keystrokeSpacing: 8,
        // Should be key, active_key
        key: {
            ...key,
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
