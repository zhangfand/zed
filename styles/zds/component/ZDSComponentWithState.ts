import { Appearance, useSystem } from "../system";
import { ZDSComponentElement, ZDSIcon, ZDSShadow, ZDSText } from "../zdsComponent";

const system = useSystem(Appearance.Dark)

export interface ComponentState {
  state: string,
  component: any
}

export default function componentWithState(states: ComponentState[]) {
  let componentWithState = []

  states.map(state => {
    // Output each state and merge with componentWithState
    const componentState = {
      state: state.state,
      component: {
        color: "#FFF"
      }
    }

    return componentState
  })

  return componentWithState
}