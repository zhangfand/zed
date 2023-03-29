import * as text from "./text"

export interface ContainedText {
    container: Container,
    text: text.TextStyle
}

type ComponentElement = Container | ContainedText

export interface ComponentStates {
    hovered: ComponentElement
    clicked: ComponentElement
    active: ComponentElement
    disabled: ComponentElement
}
