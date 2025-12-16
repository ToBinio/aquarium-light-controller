export function hsbToHex(
  hue: number,
  saturation: number,
  brightness: number,
): string {
  const c = brightness * saturation;
  const x = c * (1 - Math.abs(((hue * 6) % 2) - 1));
  const m = brightness - c;

  let r = 0,
    g = 0,
    b = 0;

  if (hue >= 0 && hue < 1 / 6) {
    r = c;
    g = x;
    b = 0;
  } else if (hue >= 1 / 6 && hue < 2 / 6) {
    r = x;
    g = c;
    b = 0;
  } else if (hue >= 2 / 6 && hue < 3 / 6) {
    r = 0;
    g = c;
    b = x;
  } else if (hue >= 3 / 6 && hue < 4 / 6) {
    r = 0;
    g = x;
    b = c;
  } else if (hue >= 4 / 6 && hue < 5 / 6) {
    r = x;
    g = 0;
    b = c;
  } else if (hue >= 5 / 6 && hue <= 1) {
    r = c;
    g = 0;
    b = x;
  }

  const red = Math.round((r + m) * 255);
  const green = Math.round((g + m) * 255);
  const blue = Math.round((b + m) * 255);

  return `#${red.toString(16).padStart(2, "0")}${green.toString(16).padStart(2, "0")}${blue.toString(16).padStart(2, "0")}`;
}
