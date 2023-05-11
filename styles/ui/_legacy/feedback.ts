import { labelButton } from "@components/button"
import { Theme } from "@theme/config"
import { interactiveTextStyle, textStyle } from "@theme/text/text"

export default function feedback(theme: Theme) {
  const link_text = interactiveTextStyle(theme)
  const info_text = textStyle(theme)
  const submitButtonStyle = labelButton({
    theme,
    componentName: "feedbackSubmitButton",
  })

  const legacy_properties = {
    button_margin: 8,
    // Should be info_text
    info_text_default: info_text,
    // Should be link_text

    link_text_default: {
      ...link_text.default.text,
      ...link_text.default.container,
    },
    link_text_hover: {
      ...link_text.hovered.text,
      ...link_text.hovered.container,
    },
    submit_button: {
      ...submitButtonStyle.default.text,
      ...submitButtonStyle.default.container,
      hover: {
        ...submitButtonStyle.hovered.text,
        ...submitButtonStyle.hovered.container,
      },
      clicked: {
        ...submitButtonStyle.pressed.text,
        ...submitButtonStyle.pressed.container,
      },
    },
  }

  return {
    ...legacy_properties,
  }
}
