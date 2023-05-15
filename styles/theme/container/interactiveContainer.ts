import { ElementState, Interactive } from "@theme/state"
import {
    ContainerStyle,
    DEFAULT_CONTAINER_INTENSITY_SET,
    containerStyle,
} from "./containerStyle"
import { Theme, ThemeColor, useColors } from "@theme"
import deepmerge from "deepmerge"
import { buildIntensitiesForStates } from "@theme/state/buildIntensitiesForStates"
import { IntensitySet } from "@theme/intensity/intensity"

interface InteractiveContainerOptions {
    states?: ElementState[]
    intensitySet?: IntensitySet
    themeColor?: ThemeColor
}

interface InteractiveContainerProps {
    theme: Theme
    options?: InteractiveContainerOptions
}

const DEFAULT_OPTIONS: Readonly<InteractiveContainerOptions> = {
    states: ["default", "hovered", "pressed", "disabled"],
    intensitySet: DEFAULT_CONTAINER_INTENSITY_SET,
}

function interactiveContainerStyle({
    theme,
    options,
}: InteractiveContainerProps): Interactive<ContainerStyle> {
    const color = useColors(theme)

    const mergedOptions = options
        ? deepmerge(DEFAULT_OPTIONS, options)
        : DEFAULT_OPTIONS

    const stateIntensities = buildIntensitiesForStates(
        theme,
        "interactiveContainer",
        mergedOptions.intensitySet
    )

    const themeColor: ThemeColor = mergedOptions.themeColor ?? "neutral"

    const container: Readonly<Interactive<ContainerStyle>> = {
        default: containerStyle({
            theme,
            options: {
                background: color[themeColor](stateIntensities.default.bg),
            },
        }),
        hovered: containerStyle({
            theme,
            options: {
                background: color[themeColor](stateIntensities.hovered.bg),
            },
        }),
        pressed: containerStyle({
            theme,
            options: {
                background: color[themeColor](stateIntensities.pressed.bg),
            },
        }),
    }

    return container
}

export { interactiveContainerStyle, InteractiveContainerOptions }
