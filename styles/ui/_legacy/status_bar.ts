import { useSurfaceStyle } from "@components/surface"
import { Theme } from "@theme"
import { padding } from "@theme/properties"
import { interactiveTextStyle, textStyle } from "@theme/text/text"
import json from "./static_json/workspace.json"
import { iconSize, iconStyle } from "@theme/icon/icon"
import { interactiveContainerStyle } from "@theme/container/interactiveContainer"

export function statusBar(theme: Theme) {
    const STATUS_BAR_HEIGHT: Readonly<number> = 30
    const STATUS_BAR_ITEM_SPACING: Readonly<number> = 8

    const surfaceStyle = useSurfaceStyle(theme, "background")

    const defaultTextStyle = textStyle(theme)

    const activeLanguageStyle = interactiveTextStyle(theme, {
        padding: padding(6, 0),
    })

    const diagnostic = {
        error: iconStyle({ theme, iconSize: "lg", themeColor: "error" }),
        warning: iconStyle({ theme, iconSize: "lg", themeColor: "warning" }),
        success: iconStyle({ theme, iconSize: "lg", themeColor: "success" }),
    }

    // Should be interactiveIconStyle
    const statusBarIcon = interactiveTextStyle(theme, {
        padding: padding(6, 3),
        borderRadius: 6,
    })

    const status_bar = {
        ...json.status_bar,
        height: STATUS_BAR_HEIGHT,
        item_spacing: STATUS_BAR_ITEM_SPACING,
        padding: padding(6, 1),
        border: {
            ...surfaceStyle.border,
            top: true,
            overlay: true,
        },
        cursor_position: textStyle(theme),
        active_language: {
            ...activeLanguageStyle.default.text,
            hover: activeLanguageStyle.hovered.text,
        },
        auto_update_progress_message: defaultTextStyle,
        auto_update_done_message: defaultTextStyle,
        // lsp_status
        // diagnostic_message
        diagnostic_summary: {
            ...json.status_bar.diagnostic_summary,
            height: 20,
            text: defaultTextStyle,
            icon_width: diagnostic.error.size,
            summary_spacing: 6,
            icon_color_ok: diagnostic.success.color,
            icon_color_warning: diagnostic.warning.color,
            icon_color_error: diagnostic.error.color,
        },
        sidebar_buttons: {
            ...json.status_bar.sidebar_buttons,
            group_left: {},
            group_right: {},
            item: {
                corner_radius: statusBarIcon.default.container.borderRadius,
                padding: statusBarIcon.default.container.padding,
                icon_size: iconSize.lg,
                icon_color: statusBarIcon.default.text.color,
                label: {
                    margin: {
                        left: 6,
                    },
                    ...statusBarIcon.default.text,
                },
                hover: {
                    icon_color: statusBarIcon.hovered.text.color,
                    background: statusBarIcon.hovered.container.background,
                },
                // This is the wrong state, should use active on toggle
                active: {
                    icon_color: statusBarIcon.pressed.text.color,
                    background: statusBarIcon.pressed.container.background,
                },
            },
            badge: {
                corner_radius: 3,
                padding: 2,
                margin: {
                    bottom: -1,
                    right: -1,
                },
                border: {
                    color: "#464b57",
                    width: 1,
                },
                background: "#18243d",
            },
        },
    }

    return status_bar
}
