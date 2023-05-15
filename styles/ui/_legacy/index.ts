import { Theme } from "@theme"
import commandPalette from "./commandPalette"
import contactNotification from "./contactNotification"
import feedback from "./feedback"
import hoverPopover from "./hoverPopover"
import incomingCallNotification from "./incomingCallNotification"
import picker from "./picker"
import projectSharedNotification from "./projectSharedNotification"
import sharedScreen from "./sharedScreen"
import simpleMessageNotification from "./simpleMessageNotification"
import tooltip from "./tooltip"
import updateNotification from "./updateNotification"
import { players } from "@theme/player/player"
import contactsPopover from "./contactsPopover"
import contactFinder from "./contactFinder"

// Static JSON from unported styleTrees
import * as staticRamps from "./static_json/ramps.json"
import * as color_scheme from "./static_json/color_scheme.json"
import * as syntax from "./static_json/syntax.json"

import * as contact_list from "./static_json/contact_list.json"
import * as context_menu from "./static_json/context_menu.json"
import * as copilot from "./static_json/copilot.json"
import * as project_diagnostics from "./static_json/project_diagnostics.json"
import * as search from "./static_json/search.json"
import * as terminal from "./static_json/terminal.json"
import * as welcome from "./static_json/welcome.json"
import projectPanel from "./projectPanel"
import workspace from "./workspace"
import editor from "./editor"

export default function app(theme: Theme) {
    const legacy_properties = {
        colorScheme: {
            ...color_scheme,
            // TODO: Remove these ASAP
            popover_shadow: {
                blur: 4,
                color: "#00000d33",
                offset: [1, 2],
            },
            modal_shadow: {
                blur: 16,
                color: "#00000d33",
                offset: [0, 2],
            },
            players: players(theme),
            ...staticRamps,
            ...syntax,
        },
    }

    return {
        meta: {
            name: theme.name,
            isLight: theme.appearance === "light",
        },
        commandPalette: commandPalette(theme),
        contactNotification: contactNotification(theme),
        projectSharedNotification: projectSharedNotification(theme),
        incomingCallNotification: incomingCallNotification(theme),
        picker: picker(theme),
        workspace: workspace(theme),
        ...copilot,
        ...welcome,
        ...context_menu,
        editor: {
            ...editor(theme),
            hoverPopover: hoverPopover(theme),
        },
        ...project_diagnostics,
        project_panel: projectPanel(theme),
        contactsPopover: contactsPopover(theme),
        contactFinder: contactFinder(theme),
        ...contact_list,
        ...search,
        sharedScreen: sharedScreen(theme),
        updateNotification: updateNotification(theme),
        simpleMessageNotification: simpleMessageNotification(theme),
        tooltip: tooltip(theme),
        ...terminal,
        feedback: feedback(theme),

        ...legacy_properties,
    }
}
