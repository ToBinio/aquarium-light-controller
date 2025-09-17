export type Updates = Activate | SetColor | SetBrightness;

export type Activate = {
  type: "Activate";
  on: boolean;
};

export type SetColor = {
  type: "SetColor";
  hue: number;
  saturation: number;
};

export type SetBrightness = {
  type: "SetBrightness";
  brightness: number;
};
