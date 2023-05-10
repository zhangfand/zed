import { Border, Theme } from "@/theme"
import {
    Intensity,
    calculateIntensity,
    resolveElementIntensities,
    IntensitySet,
} from "../intensity"
import { Padding, Margin } from "@theme/properties"
import {
    ContainedText,
    ContainedTextProps,
    containedText,
} from "./containedText"
import { FlexStyle } from "@theme/element/flex"
import { IconStyle } from "@theme/icon"
import { Shadow } from "@theme/shadow"
import { Prettify } from "@theme/types/utility"

export interface ContainerStyle {
    background?: string
    margin?: Margin
    padding?: Padding
    borderRadius?: number
    border?: Border
    width: number | "auto"
    height: number | "auto"
    shadow?: Shadow
}

const blankContainer: ContainerStyle = {
    width: "auto",
    height: "auto",
}

export const container: Record<string, ContainerStyle> = {
    blank: blankContainer,
}

// TODO: Move to a const, move to properties
export enum BorderRadius {
    "Medium" = 4,
}

export type ContainerOptions = Partial<ContainerStyle>

export const DEFAULT_CONTAINER_OPTIONS: ContainerOptions = {
    borderRadius: 0,
    width: "auto",
    height: "auto",
} as const

export function containerStyle(options: ContainerOptions): ContainerStyle {
    const mergedOptions = {
        ...DEFAULT_CONTAINER_OPTIONS,
        ...options,
    }

    return {
        width: mergedOptions.width,
        height: mergedOptions.height,
        ...mergedOptions,
    }
}

export interface ContainedIcon {
    container: ContainerStyle
    icon: IconStyle
}

export type ContainedTextAndIcon = Prettify<ContainedText & ContainedIcon>

export type Element =
    | FlexStyle
    | ContainerStyle
    | ContainedIcon
    | ContainedText
    | ContainedTextAndIcon

export interface Interactive<T = Element> {
    default: T
    hovered: T
    pressed: T
    dragged?: T
    disabled?: T
}

export interface Toggleable<T = Interactive> {
    inactive: T
    active: T
}

export type State = "default" | "hovered" | "pressed" | "dragged" | "disabled"

export type StateIntensities = Partial<Record<State, IntensitySet>>

/** Returns a StateIntensitySet for each default state */
export function buildIntensitiesForStates(
    theme: Theme,
    name: string,
    startingIntensity: IntensitySet
): StateIntensities {
    const light = theme.appearance === "light"
    const multiplier = light ? 1 : 1.2
    const stepSize = 5
    const startingOffset = light ? 5 : 12
    const intensitySteps = [0, 1, 2, 3].map(
        (step) => multiplier * stepSize * step + startingOffset
    )

    const scaleFactor = theme.intensity.scaleFactor

    const scaledIntensitySteps = intensitySteps.map(
        (intensity) => intensity * scaleFactor
    )

    const resolvedIntensity = resolveElementIntensities(
        theme,
        startingIntensity
    )

    const defaultState = {
        bg: resolvedIntensity.bg,
        border: resolvedIntensity.border,
        fg: resolvedIntensity.fg,
    }

    const disabledState: IntensitySet = {
        bg: 5 as Intensity,
        border: 10 as Intensity,
        fg: 35 as Intensity,
    }

    const elementStates = {
        default: buildStateIntensity(name, "default", defaultState),
        hovered: buildStateIntensity(
            name,
            "hovered",
            defaultState,
            scaledIntensitySteps[1]
        ),
        pressed: buildStateIntensity(
            name,
            "pressed",
            defaultState,
            scaledIntensitySteps[2]
        ),
        disabled: disabledState,
    }

    return elementStates
}

export function buildStateIntensity(
    componentName: string,
    name: string,
    startingIntensity: IntensitySet,
    change?: number
): IntensitySet {
    if (!change) {
        return startingIntensity
    }

    const stateIntensity: IntensitySet = {
        bg: calculateIntensity(startingIntensity.bg, change),
        border: calculateIntensity(startingIntensity.border, change),
        fg: calculateIntensity(startingIntensity.fg, change),
    }

    const nameForCheck = `${componentName} ${name}`

    checkContrast(nameForCheck, startingIntensity.bg, stateIntensity.fg)

    return stateIntensity
}

export const checkContrast = (
    name: string,
    background: Intensity,
    foreground: Intensity,
    debug?: boolean
) => {
    const foregroundIntensity = Math.max(foreground, background) + 0.05
    const backgroundIntensity = Math.min(foreground, background) + 0.05
    const contrastRatio = foregroundIntensity / backgroundIntensity

    // Return a contrast with 2 decimal places
    const contrast = +contrastRatio.toFixed(2)

    debug &&
        console.log(
            `Contrast on ${name}: ${contrast}. Foreground: ${foreground}, Background: ${background}`
        )

    if (contrast < 4.5) {
        console.log(`Constrast on ${name} may be too low: ${contrast}`)
    }

    if (contrast < 3) {
        throw new Error(`Constrast on ${name} is too low: ${contrast}`)
    }
}

export { ContainedText, ContainedTextProps, containedText }
