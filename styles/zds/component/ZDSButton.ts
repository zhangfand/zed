import { Appearance, useSystem } from "../system";
import { ZDSComponentElement, ZDSIcon, ZDSText } from "../zdsComponent";
import { defaultIcon, defaultShadow, defaultText } from "./ZDSCommon";

const system = useSystem(Appearance.Dark)

interface ZDSButton {
  container: ZDSComponentElement
  label: ZDSText,
  icon: ZDSIcon
}

export function zdsButton(text: string) {
  const button: ZDSButton = {
    container: {
      color: system.color.primary,
      border: {
        radius: 4, // 4 (All) | [4,0] (TL,BR) | [4,0,4,0] (TL, TR, BR, BL)
        color: system.color.border
      },
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
