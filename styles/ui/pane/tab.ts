import { iconButton } from "@components/button"
import { Button } from "@components/button/build"
import { useSurfaceIntensity } from "@components/surface"
import { Theme } from "@theme"
import { borderStyle } from "@theme/properties/border"
import { ContainedIcon, ContainedText, container } from "@theme/container"
import { buildIntensitiesForStates } from "@theme/state/buildIntensitiesForStates"
import { ContainerStyle } from "@theme/container/containerStyle"
import { FlexStyle, flex } from "@theme/element/flex"
import { IconStyle, iconStyle } from "@theme/icon/icon"
import {
    addToIntensitySet,
    resolveIntensitySet,
} from "@theme/intensity/intensity"
import { padding } from "@theme/properties"
import { background } from "@theme/properties/background"
import { TextStyle, textStyle } from "@theme/text/text"
import { Interactive, ElementState } from "@theme/state"

interface TabProps {
    theme: Theme
    active?: boolean
    state: ElementState
}

interface Indicators {
    dirty: IconStyle
    conflict: IconStyle
}

interface Tab {
    flex: FlexStyle
    container: ContainerStyle
    // Indicates the type of tab, e.g. "Project Search", "Feedback"
    icon: IconStyle
    // Indicates the status of the tab, e.g. "Dirty", "Conflict"
    indicator: Indicators
    label: TextStyle
    // When two tabs of the same name are open, a description appears next to the label
    description: ContainedText
    close: Button<ContainedIcon>
}

function tabState({ theme, active = false, state }: TabProps): Tab {
    const name = active ? "active_tab" : "tab"
    const TAB_HEIGHT = 32

    const intensities = active
        ? useSurfaceIntensity(theme, "pane")
        : addToIntensitySet(useSurfaceIntensity(theme, "pane"), 20)

    const resolvedIntensities = resolveIntensitySet(theme, intensities)

    const interactiveIntensities = buildIntensitiesForStates(
        theme,
        name,
        resolvedIntensities
    )

    const containerStyle = (state: ElementState): ContainerStyle => {
        return {
            height: TAB_HEIGHT,
            background: background(theme, interactiveIntensities[state].bg),
            border: borderStyle({
                theme,
                intensity: interactiveIntensities[state].border,
            }),
            padding: padding(0, 12, 0, 8),
        }
    }

    const text = textStyle(theme, {
        intensity: 70,
    })

    return {
        container: containerStyle(state),
        flex: flex(8, {
            alignItems: "center",
        }),
        icon: iconStyle({
            theme,
            iconSize: "md",
            intensity: 70,
        }),
        indicator: {
            dirty: iconStyle({
                theme,
                iconSize: "sm",
                themeColor: "accent",
            }),
            conflict: iconStyle({
                theme,
                iconSize: "sm",
                themeColor: "warning",
            }),
        },
        label: text,
        description: {
            container: {
                ...container.blank,
            },
            text: textStyle(theme, {
                intensity: 50,
            }),
        },
        close: iconButton({ theme }),
    }
}

export function activeTab(theme: Theme): Interactive<Tab> {
    return {
        default: tabState({
            theme,
            active: false,
            state: "default",
        }),
        hovered: tabState({
            theme,
            active: false,
            state: "hovered",
        }),
        pressed: tabState({
            theme,
            active: false,
            state: "pressed",
        }),
    }
}

export function inactiveTab(theme: Theme): Interactive<Tab> {
    return {
        default: tabState({
            theme,
            active: false,
            state: "default",
        }),
        hovered: tabState({
            theme,
            active: false,
            state: "hovered",
        }),
        pressed: tabState({
            theme,
            active: false,
            state: "pressed",
        }),
    }
}
