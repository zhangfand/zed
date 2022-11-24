import { Appearance, useSystem } from "../system";
import { ZDSComponentElement, ZDSText } from "../zdsComponent";

const system = useSystem(Appearance.Dark)

interface ZDSButton {
  container: ZDSComponentElement
  label: ZDSText
}

export default function zdsButton() {
  const button = {
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
      // level: system.level.level0
    },
    label: {
      ...defaultText,
      color: system.color.primary
    },
    icon: {
      ...defaultIcon,
      color: system.color.primary
    }
  }
})