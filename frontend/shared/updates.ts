export type Updates = Activate | SetColor | SetBrightness;

export type Activate = {
  type: "Activate";
  on: boolean;
};

export type SetColor = {
  type: "SetColor";
  hex: string;
};

export type SetBrightness = {
  type: "SetBrightness";
  value: number;
};
