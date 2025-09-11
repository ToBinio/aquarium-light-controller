<script setup lang="ts">
import { SliderRoot, SliderThumb, SliderTrack } from "reka-ui";

let hexColor = defineModel("hexColor");

const sliderValue = ref([0]);

function hueToHex(hue: number): string {
    hue = ((hue % 360) + 360) % 360;

    const saturation = 1;
    const lightness = 0.5;

    const c = (1 - Math.abs(2 * lightness - 1)) * saturation;
    const x = c * (1 - Math.abs(((hue / 60) % 2) - 1));
    const m = lightness - c / 2;

    let r = 0,
        g = 0,
        b = 0;

    if (hue < 60) [r, g, b] = [c, x, 0];
    else if (hue < 120) [r, g, b] = [x, c, 0];
    else if (hue < 180) [r, g, b] = [0, c, x];
    else if (hue < 240) [r, g, b] = [0, x, c];
    else if (hue < 300) [r, g, b] = [x, 0, c];
    else [r, g, b] = [c, 0, x];

    const toHex = (n: number) =>
        Math.round((n + m) * 255)
            .toString(16)
            .padStart(2, "0");

    return `#${toHex(r)}${toHex(g)}${toHex(b)}`;
}

watch(
    sliderValue,
    (sliderValue) => {
        const [hue] = sliderValue;
        hexColor.value = hueToHex(hue ?? 0);
    },
    { deep: true },
);
</script>
<template>
    <SliderRoot
        v-model="sliderValue"
        class="relative flex items-center select-none touch-none h-5"
        :max="360"
        :step="1"
    >
        <SliderTrack id="hue-slider-track" class="grow rounded-full h-2" />
        <SliderThumb
            class="w-4 h-4 bg-white rounded-full focus:outline-none"
            aria-label="Volume"
        />
    </SliderRoot>
</template>
<style lang="css" scoped>
#hue-slider-track {
    background: linear-gradient(
        to right,
        rgb(255, 0, 0),
        rgb(255, 255, 0),
        rgb(0, 255, 0),
        rgb(0, 255, 255),
        rgb(0, 0, 255),
        rgb(255, 0, 255),
        rgb(255, 0, 0)
    );
}
</style>
