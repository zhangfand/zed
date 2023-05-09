import { iconButton } from "@components/button";
import { Theme } from "@theme";
import picker from "./picker";
import { inputStyle } from "@components/input";
import { margin, padding } from "@theme/properties";
import { containerStyle } from "@theme/container";

export default function contactFinder(theme: Theme) {
  const SIDE_MARGIN: Readonly<number> = 6;
  const ROW_HEIGHT: Readonly<number> = 28;
  const CONTACT_AVATAR_SIZE: Readonly<number> = 18;


  // TODO: picker -> palette, move to components
  const pickerStyle = picker(theme)
  const pickerInput = inputStyle({
    theme, options: {
      margin: margin(SIDE_MARGIN, 0)
    }
  })

  const addContactButton = iconButton({
    theme, options: {
      width: 16,
      iconSize: "sm",
    }
  })

  const legacyContactButton = {
    background: addContactButton.default.container.background,
    color: addContactButton.default.icon.color,
    iconWidth: addContactButton.default.icon.size,
    buttonWidth: addContactButton.default.container.width,
    cornerRadius: addContactButton.default.container.borderRadius,
    hover: {
      background: addContactButton.hovered.container.background,
    }
  }

  const legacy_properties = {
    picker: {
      emptyContainer: {},
      item: {
        ...pickerStyle.item,
        margin: { left: SIDE_MARGIN, right: SIDE_MARGIN },
      },
      noMatches: pickerStyle.noMatches,
      inputEditor: pickerInput,
      emptyInputEditor: pickerInput,
    },
    // Convert to a true Interactive<Container> in Rust
    contactButton: legacyContactButton,
    diabledContactButton: {
      ...legacyContactButton,
      background: addContactButton.disabled.container.background,
      color: addContactButton.disabled.icon.color,
    },
    rowHeight: ROW_HEIGHT,
  }

  return {
    ...legacy_properties,
    contactAvatar: containerStyle({
      width: CONTACT_AVATAR_SIZE,
      height: CONTACT_AVATAR_SIZE,
      borderRadius: CONTACT_AVATAR_SIZE / 2,
    }),
    contactUsername: containerStyle({
      padding: padding(0, 0, 0, 8),
    })
  }
}
