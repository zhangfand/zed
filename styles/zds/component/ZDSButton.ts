import { Container } from "../mixin/Container";
import { Appearance, useSystem } from "../system";
import { ZDSIcon, ZDSText } from "../zdsComponent";
import { defaultIcon, defaultShadow, defaultText } from "./ZDSCommon";

const system = useSystem(Appearance.Dark)

interface ZDSButton {
  container: Partial<Container>
  label: ZDSText,
  icon: ZDSIcon
}

export function zdsButton(text: string) {
  const button: ZDSButton = {
    container: {
      fill: system.color.primary,
      border: {
        fill: system.color.border,
        width: 1,
        style: "SOLID"
      },
      cornerRadius: 4,
      shadow: {
        ...defaultShadow,
        color: system.color.shadow
      }
    },
    label: {
      ...defaultText,
      text: text,
      color: system.color.primary
    },
    icon: {
      ...defaultIcon,
      color: system.color.accent
    }
  }

  return button
}

export default console.log(zdsButton("Hello World"))
