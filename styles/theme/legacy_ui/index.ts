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

    meta: {
      name: theme.name,
      isLight: theme.appearance === "light",
    },
    commandPalette: commandPalette(theme),
    contactFinder: {
      comment: "Not added yet"
    },
    contactList: {
      comment: "Not added yet"
    },
    contactNotification: contactNotification(theme),
    contactsPopover: contactsPopover(theme),
    copilot: {
      comment: "Not added yet"
    },
    editor: {
      comment: "Not added yet"
    },
    feedback: feedback(theme),
    hoverPopover: hoverPopover(theme),
    incomingCallNotification: incomingCallNotification(theme),
    picker: picker(theme),
    projectDiagnostics: {
      comment: "Not added yet"
    },
    projectPanel: {
      comment: "Not added yet"
    },
    projectSharedNotification: projectSharedNotification(theme),
    search: {
      comment: "Not added yet"
    },
    sharedScreen: sharedScreen(theme),
    simpleMessageNotification: simpleMessageNotification(theme),
    terminal: {
      comment: "Not added yet"
    },
    tooltip: tooltip(theme),
    updateNotification: updateNotification(theme),
    welcome: {
      comment: "Not added yet"
    },
    workspace: {
      comment: "Not added yet"
    },
    ...legacy_properties,
  }
}
