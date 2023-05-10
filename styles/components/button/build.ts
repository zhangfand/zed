import { Theme, useColors } from "@/theme"
import { border } from "@theme/border"
import {
  BorderRadius,
  ContainedIcon,
  ContainedText,
  ContainerStyle,
  Interactive,
  buildIntensitiesForStates,
} from "@theme/container"
import { TextStyle } from "@theme/text"
import { IntensitySet, resolveElementIntensities } from "@theme/intensity"
import { margin, padding } from "@theme/properties"
import { textStyle } from "@theme/text"
import { iconStyle } from "@theme/icon"

type ButtonSizes = "small" | "medium" | "large"
type ButtonSize = (typeof buttonSize)[keyof typeof buttonSize]

const buttonSize: Record<ButtonSizes, number> = {
  small: 15,
  medium: 21,
  large: 25,
}

const DEFAULT_BUTTON_INTENSITIES: IntensitySet = {
  bg: 5,
  border: 8,
  fg: 100,
}

interface ButtonProps {
  theme: Theme
  /** A unique name for the button
   *
   *  Used for debugging & contrast validation  */
  name: string
  kind: ButtonKind
  intensities?: IntensitySet
  size?: ButtonSize
}

export type Button<T = ContainedIcon | ContainedText> = Interactive<T>

type ButtonKind = "icon" | "label"

export function buildButton({
  theme,
  name,
  kind = "label",
  intensities = DEFAULT_BUTTON_INTENSITIES,
  size = buttonSize.medium,
}: ButtonProps): Button {
  const color = useColors(theme)
  const resolvedIntensities = resolveElementIntensities(theme, intensities)

  const container: ContainerStyle = {
    background: color.neutral(resolvedIntensities.bg),
    margin: margin(0, 0, 0, 0),
    padding: padding(6, 4),
    borderRadius: BorderRadius.Medium,
    border: border({ theme, intensity: resolvedIntensities.border }),
    height: size,
  }

  const icon = iconStyle({
    theme,
    intensity: resolvedIntensities.fg,
    size: "md",
  })

  const text: TextStyle = textStyle(theme, {
    intensity: resolvedIntensities.fg,
  })

  const states = buildIntensitiesForStates(theme, name, resolvedIntensities)

  const buildStates = (intensities: IntensitySet) => {
    const updatedContainer = {
      ...container,
      background: color.neutral(intensities.bg),
      border: border({ theme, intensity: intensities.border }),
    }

    const updatedIcon = {
      ...icon,
      color: color.neutral(intensities.fg),
    }

    const updatedText = {
      ...text,
      color: color.neutral(intensities.fg),
    }

    let stateStyle

    switch (kind) {
      case "icon":
        stateStyle = {
          container: updatedContainer,
          icon: updatedIcon,
        }

        return stateStyle as ContainedIcon
      case "label":
        stateStyle = {
          container: updatedContainer,
          text: updatedText,
        }
        return stateStyle as ContainedText
      default:
        throw new Error("Unhandled button kind")
    }
  }

  const button = {
    default: buildStates(states.default),
    hovered: buildStates(states.hovered),
    pressed: buildStates(states.pressed),
  }

  switch (kind) {
    case "icon":
      return button as Button<ContainedIcon>
    case "label":
      return button as Button<ContainedText>
    default:
      throw new Error("Unhandled button kind")
  }
}
