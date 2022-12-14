import Theme from "../src/themes/common/theme";

interface TokenSet {

}

function buildTokenSet(theme: Theme) {
  return {
    meta: {
      themeName: {
        value: theme.name,
        type: "other",
      },
    },
    shadowAlpha: {
      value: "0.24",
      type: "number",
    },
  };
}


const tokens: TokenSet = {}

export default tokens