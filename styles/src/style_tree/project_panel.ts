import {
    Border,
    TextStyle,
    background,
    border,
    text,
} from "./components"
import { interactive, toggleable } from "../element"
import merge from "ts-deepmerge"
import { useTheme } from "../theme"
import { neutral } from "../color"
export default function project_panel(): any {
    const theme = useTheme()

    const { is_light } = theme

    type EntryStateProps = {
        background?: string
        border?: Border
        text?: TextStyle
        icon_color?: string
    }

    type EntryState = {
        default: EntryStateProps
        hovered?: EntryStateProps
        clicked?: EntryStateProps
    }

    const entry = (unselected?: EntryState, selected?: EntryState) => {
        const git_status = {
            git: {
                modified: is_light
                    ? theme.ramps.yellow(0.6).hex()
                    : theme.ramps.yellow(0.5).hex(),
                inserted: is_light
                    ? theme.ramps.green(0.45).hex()
                    : theme.ramps.green(0.5).hex(),
                conflict: is_light
                    ? theme.ramps.red(0.6).hex()
                    : theme.ramps.red(0.5).hex(),
            },
        }

        const base_properties = {
            height: 22,
            background: neutral.surface,
            chevron_color: neutral.foreground_variant,
            icon_color: neutral.foreground_variant,
            chevron_size: 7,
            icon_size: 14,
            icon_spacing: 6,
            text: text(theme.middle, "sans", "variant", { size: "sm", color: neutral.foreground_variant }),
            status: {
                ...git_status,
            },
        }

        const selected_style: EntryState | undefined = selected
            ? selected
            : unselected

        const unselected_default_style = merge(
            base_properties,
            unselected?.default ?? {},
            {},
        )
        const unselected_hovered_style = merge(
            base_properties,
            { background: neutral.hover },
            unselected?.hovered ?? {},
        )
        const unselected_clicked_style = merge(
            base_properties,
            { background: neutral.pressed },
            unselected?.clicked ?? {},
        )
        const selected_default_style = merge(
            base_properties,
            {
                background: neutral.selected,
                text: text(theme.lowest, "sans", { size: "sm", color: neutral.foreground }),
            },
            selected_style?.default ?? {},
        )
        const selected_hovered_style = merge(
            base_properties,
            {
                background: neutral.hover,
                text: text(theme.lowest, "sans", { size: "sm", color: neutral.foreground }),
            },
            selected_style?.hovered ?? {},
        )
        const selected_clicked_style = merge(
            base_properties,
            {
                background: neutral.pressed,
                text: text(theme.lowest, "sans", { size: "sm", color: neutral.foreground }),
            },
            selected_style?.clicked ?? {},
        )

        return toggleable({
            state: {
                inactive: interactive({
                    state: {
                        default: unselected_default_style,
                        hovered: unselected_hovered_style,
                        clicked: unselected_clicked_style,
                    },
                }),
                active: interactive({
                    state: {
                        default: selected_default_style,
                        hovered: selected_hovered_style,
                        clicked: selected_clicked_style,
                    },
                }),
            },
        })
    }

    const default_entry = entry()

    return {
        open_project_button: interactive({
            base: {
                background: neutral.surface,
                border: {
                    color: neutral.border,
                    width: 1,
                    top: true,
                    bottom: true,
                    left: true,
                    right: true,
                },
                corner_radius: 4,
                margin: {
                    top: 16,
                    left: 16,
                    right: 16,
                },
                padding: {
                    top: 3,
                    bottom: 3,
                    left: 7,
                    right: 7,
                },
                ...text(theme.middle, "sans", "default", { size: "sm", color: neutral.foreground }),
            },
            state: {
                hovered: {
                    ...text(theme.middle, "sans", "default", { size: "sm" }),
                    background: background(theme.middle, "hovered"),
                    border: border(theme.middle, "active"),
                },
                clicked: {
                    ...text(theme.middle, "sans", "default", { size: "sm" }),
                    background: background(theme.middle, "pressed"),
                    border: border(theme.middle, "active"),
                },
            },
        }),
        background: neutral.surface,
        padding: { left: 6, right: 6, top: 0, bottom: 6 },
        indent_width: 20,
        entry: default_entry,
        dragged_entry: {
            ...default_entry.inactive.default,
            text: text(theme.middle, "sans", "on", { size: "sm" }),
            background: neutral.background_inverse,
            border: {
                color: neutral.border,
                width: 1,
                top: true,
                bottom: true,
                left: true,
                right: true,
            }
        },
        ignored_entry: entry(
            {
                default: {
                    text: text(theme.middle, "sans", "disabled", { color: neutral.foreground_hidden }),
                    icon_color: neutral.foreground_hidden,
                },
                hovered: {
                    text: text(theme.middle, "sans", "disabled", { color: neutral.foreground_hidden }),
                    icon_color: neutral.foreground_hidden,

                    background: neutral.hover,
                },
                clicked: {
                    text: text(theme.middle, "sans", "disabled", { color: neutral.foreground_hidden }),
                    icon_color: neutral.foreground_hidden,
                    background: neutral.pressed,
                },
            },
            {
                default: {
                    text: text(theme.middle, "sans", "disabled", { color: neutral.foreground_muted }),
                    icon_color: neutral.foreground_muted,
                },
                hovered: {
                    text: text(theme.middle, "sans", "disabled", { color: neutral.foreground_muted }),
                    icon_color: neutral.foreground_muted,
                    background: neutral.hover,
                },
                clicked: {
                    text: text(theme.middle, "sans", "disabled", { color: neutral.foreground_muted }),
                    icon_color: neutral.foreground_muted,
                    background: neutral.pressed,
                },
            },
        ),
        cut_entry: entry(
            {
                default: {
                    text: text(theme.middle, "sans", "disabled"),
                },
            },
            {
                default: {
                    background: background(theme.middle, "active"),
                    text: text(theme.middle, "sans", "disabled", {
                        size: "sm",
                    }),
                },
            },
        ),
        filename_editor: {
            background: background(theme.middle, "on"),
            text: text(theme.middle, "sans", "on", { size: "sm" }),
            selection: theme.players[0],
        },
    }
}
