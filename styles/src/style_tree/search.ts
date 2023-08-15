import { with_opacity } from "../theme/color"
import { background, border, foreground, text } from "./components"
import { interactive, toggleable } from "../element"
import { useTheme } from "../theme"
import { icon_button, toggleable_icon_button } from "../component/icon_button"
import { text_button } from "../component/text_button"

export default function search(): any {
    const theme = useTheme()

    // Search input
    const editor = {
        background: background(theme.highest),
        corner_radius: 8,
        min_width: 200,
        max_width: 500,
        placeholder_text: text(theme.highest, "mono", "disabled"),
        selection: theme.players[0],
        text: text(theme.highest, "mono", "default"),
        border: border(theme.highest),
        margin: {
            right: 9,
        },
        padding: {
            top: 4,
            bottom: 4,
            left: 10,
            right: 4,
        },
    }

    const include_exclude_editor = {
        ...editor,
        min_width: 100,
        max_width: 250,
    }

    return {
        padding: { top: 16, bottom: 16, left: 16, right: 16 },
        // TODO: Add an activeMatchBackground on the rust side to differentiate between active and inactive
        match_background: with_opacity(
            foreground(theme.highest, "accent"),
            0.4
        ),
        option_button: toggleable_icon_button(theme, {
            active_color: "accent"
        }),
        action_button: text_button({}),
        editor,
        invalid_editor: {
            ...editor,
            border: border(theme.highest, "negative"),
        },
        include_exclude_editor,
        invalid_include_exclude_editor: {
            ...include_exclude_editor,
            border: border(theme.highest, "negative"),
        },
        match_index: {
            ...text(theme.highest, "mono", "variant"),
            padding: {
                left: 6,
            },
        },
        option_button_group: {
            padding: {
                left: 12,
                right: 12,
                // top: 3,
                // bottom: 3,
            },
        },
        include_exclude_inputs: {
            ...text(theme.highest, "mono", "variant"),
            padding: {
                right: 6,
            },
        },
        major_results_status: {
            ...text(theme.highest, "mono", "on"),
            size: 15,
        },
        minor_results_status: {
            ...text(theme.highest, "mono", "variant"),
            size: 13,
        },
        dismiss_button: icon_button({}),
        editor_icon: {
            icon: {
                color: foreground(theme.highest, "variant"),
                asset: "icons/magnifying_glass_12.svg",
                dimensions: {
                    width: 12,
                    height: 12,
                }
            },
            container: {
                margin: { right: 6 },
                padding: { left: 2, right: 2 },
            }
        },
        mode_button: toggleable({
            base: interactive({
                base: {
                    ...text(theme.highest, "mono", "variant"),
                    background: background(theme.highest, "variant"),

                    border: {
                        ...border(theme.highest, "on"),
                        left: false,
                        right: false
                    },

                    padding: {
                        // bottom: 4,
                        left: 10,
                        right: 10,
                        // top: 5,
                    },
                    corner_radius: 6,
                },
                state: {
                    hovered: {
                        ...text(theme.highest, "mono", "variant", "hovered"),
                        background: background(theme.highest, "variant", "hovered"),
                        border: border(theme.highest, "on", "hovered"),
                    },
                    clicked: {
                        ...text(theme.highest, "mono", "variant", "pressed"),
                        background: background(theme.highest, "variant", "pressed"),
                        border: border(theme.highest, "on", "pressed"),
                    },
                },
            }),
            state: {
                active: {
                    default: {
                        ...text(theme.highest, "mono", "on"),
                        background: background(theme.highest, "on")
                    },
                    hovered: {
                        ...text(theme.highest, "mono", "on", "hovered"),
                        background: background(theme.highest, "on", "hovered")
                    },
                    clicked: {
                        ...text(theme.highest, "mono", "on", "pressed"),
                        background: background(theme.highest, "on", "pressed")
                    },
                },
            },
        }),
        nav_button: icon_button({}),
        search_bar_row_height: 32,
        option_button_height: 22,
        modes_container: {
            margin: {
                right: 9
            }
        }

    }
}
