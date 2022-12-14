export interface StringToken {
  value: string
}

export interface OtherToken extends StringToken {
  type: "other";
}

export interface BorderRadiusToken extends StringToken {
  type: "borderRadius";
}

export interface ColorToken extends StringToken {
  type: "color";
}

function colorToken(value: string): ColorToken {
  return {
    value: value,
    type: "color",
  };
}

export interface FontFamilyToken extends StringToken {
  type: "fontFamilies";
}

export interface FontWeightToken extends StringToken {
  type: "fontWeights";
}

export interface FontSizeToken extends StringToken {
  type: "fontSizes";
}

export interface LineHeightToken extends StringToken {
  type: "lineHeights";
}

export interface TextDecorationToken {
  value: "none" | "underline" | "strikethrough";
  type: "textDecoration";
}

export interface TypographyToken {
  value: {
    fontFamily: FontFamilyToken;
    fontWeight: FontWeightToken;
    fontSize: FontSizeToken;
    lineHeight: LineHeightToken; // "27px", "130%"
    letterSpacing: string; // Hard coded for now
    paragraphSpacing: string; // Hard coded for now
    textDecoration: TextDecorationToken;
  };
  type: "typography";
}