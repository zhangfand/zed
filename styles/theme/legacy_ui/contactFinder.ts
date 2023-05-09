import { iconButton } from "@components/button";
import { Theme } from "@theme";
import { popoverContainerStyle } from "@theme/container/popover";

export default function contactFinder(theme: Theme) {
  const SIDE_MARGIN: Readonly<number> = 6;
  const ICON_WIDTH: Readonly<number> = 8;
  const BUTTON_WIDTH: Readonly<number> = 16;

  const container = popoverContainerStyle({ theme })

  const contactButton = iconButton(theme)

  const legacy_properties = {

  }

  return {
    ...legacy_properties,
  }
}
