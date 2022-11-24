const zds = {
  theme: {
    meta: {
      name: "Theme Name",
    },
    colors: {
      // Theme Specific Colors
    },
    // Elevations
    level0: {
      lowest: {
        base: {
          default: {
            background: {},
            foreground: {},
            border: {}
          }
        },
        variant: {},
        on: {},
        accent: {},
        positive: {},
        warning: {},
        negative: {}
      },
      middle: {

      },
      highest: {
      }
    },
    syntax: {
      // Default syntax values
      markdown: {
        url: {
          color: "orange",
          weight: "bold", // Normal, Bold...
          decoration: "underline", // underline, strikethrough
        }
      },
      unreachable: {
        dectoration: "strikethrough"
      }
    },
    players: {
      // players 1-8
      player1: {
        color: {},
        border: {},
        selection: {}
      }
    },
  },
  // Default Theme Colors
  colors: {
    neutral: [
      // 10 colors
    ],
    red: [
      // some colors
    ]
  },
  // Default Theme Shadows
  shadows: {
    modal: {},
    popover: {},
    drag: {}
  },
  borderRadius: {
    // xs-xx
  }
  // spacing
}
