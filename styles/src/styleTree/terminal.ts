import Theme from "../themes/common/theme";
import { backgroundColor } from "./components";

export default function terminal(theme: Theme) {
  return {
    container: {
      background: backgroundColor(theme, 100),
      padding: {
        left: 8,
        right: 8,
        top: 4,
        bottom: 4
      },
    },
  }
}