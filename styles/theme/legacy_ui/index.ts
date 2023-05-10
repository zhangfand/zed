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
import { players } from "@theme/players"
import contactsPopover from "./contactsPopover"
import contactFinder from "./contactFinder"

// Static JSON from unported styleTrees
import * as staticRamps from "./static_json/ramps.json"
import * as color_scheme from "./static_json/color_scheme.json"
import * as syntax from "./static_json/syntax.json"

export default function app(theme: Theme) {
    const legacy_properties = {
        colorScheme: {
            ...color_scheme,
            players: players(theme),
            ramps: staticRamps,
            ...syntax,
        },
    }

    return {
        meta: {
            name: theme.name,
            isLight: theme.appearance === "light",
        },

        // Palette
        picker: picker(theme),
        commandPalette: commandPalette(theme),

        // Notifcations
        contactNotification: contactNotification(theme),
        incomingCallNotification: incomingCallNotification(theme),
        simpleMessageNotification: simpleMessageNotification(theme),
        projectSharedNotification: projectSharedNotification(theme),
        updateNotification: updateNotification(theme),

        // Pane
        feedback: feedback(theme),

        // Sort
        hoverPopover: hoverPopover(theme),
        tooltip: tooltip(theme),
        sharedScreen: sharedScreen(theme),
        contactsPopover: contactsPopover(theme),
        contactFinder: contactFinder(theme),

        // TODO
        // workspace: workspace(theme),
        // copilot: copilot(theme),
        // welcome: welcome(theme),
        // editor: editor(theme),
        // projectDiagnostics: projectDiagnostics(theme),
        // projectPanel: projectPanel(theme),
        // contactList: contactList(theme),
        // search: search(theme),
        // terminal: terminal(theme),

        ...legacy_properties,
    }
}
