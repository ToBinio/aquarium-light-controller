<script setup lang="ts">
import { SliderRoot, SliderThumb, SliderTrack } from "reka-ui";
import type { SetColor } from "~~/shared/updates";

async function setColor() {
    let color: SetColor = {
        type: "SetColor",
        hue: hueValue.value[0]!,
        saturation: saturationValue.value[0]!,
    };

    await $fetch("/api/update", {
        method: "POST",
        body: JSON.stringify(color),
    });
}

function hsbToHex(hue: number, saturation: number, brightness: number): string {
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

const hueValue = ref([0]);
const saturationValue = ref([0]);

const colorHex = computed(() => {
    return hsbToHex(hueValue.value[0]!, saturationValue.value[0]!, 0.5);
});
</script>
<template>
    <div>
        <div class="flex gap-2">
            <div :style="{ backgroundColor: colorHex }" class="w-10 h-10"></div>
            <div class="flex-1">
                <SliderRoot
                    v-model="hueValue"
                    class="relative flex items-center select-none touch-none h-5"
                    :max="1"
                    :step="0.01"
                >
                    <SliderTrack
                        id="hue-slider-track"
                        class="grow rounded-full h-2"
                    />
                    <SliderThumb
                        class="w-4 h-4 bg-white rounded-full focus:outline-none"
                        aria-label="Volume"
                    />
                </SliderRoot>
                <SliderRoot
                    v-model="saturationValue"
                    class="relative flex items-center select-none touch-none h-5"
                    :max="1"
                    :step="0.01"
                >
                    <SliderTrack
                        id="saturation-slider-track"
                        class="grow rounded-full h-2"
                    />
                    <SliderThumb
                        class="w-4 h-4 bg-white rounded-full focus:outline-none"
                        aria-label="Volume"
                    />
                </SliderRoot>
            </div>
        </div>
        <button
            @click="setColor"
            class="bg-neutral-500 hover:bg-neutral-700 text-white font-bold py-1 px-2 mt-2 rounded"
        >
            update Color
        </button>
    </div>
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

#saturation-slider-track {
    background: linear-gradient(to right, rgb(0, 0, 0), rgb(255, 255, 255));
}
</style>
