import { Theme } from "@theme";
import commandPalette from "./commandPalette";
import contactNotification from "./contactNotification";
import feedback from "./feedback";
import hoverPopover from "./hoverPopover";
import incomingCallNotification from "./incomingCallNotification";
import picker from "./picker";
import projectSharedNotification from "./projectSharedNotification";
import sharedScreen from "./sharedScreen";
import simpleMessageNotification from "./simpleMessageNotification";
import tooltip from "./tooltip";
import updateNotification from "./updateNotification";
import { players } from "@theme/players";

export default function app(theme: Theme) {
  const legacy_properties = {
    colorScheme: {
      players: players(theme)
    }
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

    // Misc
    hoverPopover: hoverPopover(theme),
    tooltip: tooltip(theme),
    sharedScreen: sharedScreen(theme),

    ...legacy_properties
  }
}
