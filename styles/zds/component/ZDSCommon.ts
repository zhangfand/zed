import { Appearance, useSystem } from "../system";
import { ZDSIcon, ZDSShadow, ZDSText } from "../zdsComponent";

const system = useSystem(Appearance.Dark)

export const defaultShadow: ZDSShadow = {
  x: 0,
  y: 2,
  blur: 8,
  color: system.color.shadow,
  opacity: 0.16
}

export const defaultText: ZDSText = {
  text: "text",
  family: system.font.sans,
  color: system.color.primary,
  lineHeight: 1,
  size: 14,
  tracking: 0,
  weight: "Regular"
}

export const defaultIcon: ZDSIcon = {
  color: system.color.primary,
  size: 16,
  style: "outline"
}