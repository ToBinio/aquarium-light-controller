<script setup lang="ts">
import { SliderRoot, SliderThumb, SliderTrack } from "reka-ui";

let emits = defineEmits<{
    (e: "update", color: { hue: number; saturation: number }): void;
}>();

const hueValue = ref([0]);
const saturationValue = ref([0]);

const colorHex = computed(() => {
    return hsbToHex(hueValue.value[0]!, saturationValue.value[0]!, 0.5);
});

let timeoutId: NodeJS.Timeout | null = null;
watch([hueValue, saturationValue], async () => {
    if (timeoutId) clearTimeout(timeoutId);
    timeoutId = setTimeout(() => {
        emits("update", {
            hue: hueValue.value[0]!,
            saturation: saturationValue.value[0]!,
        });
    }, 500);
});
</script>
<template>
    <div class="text-gray-400">
        Hue
        <SliderRoot
            v-model="hueValue"
            class="relative flex items-center select-none touch-none h-5"
            :max="1"
            :step="0.01"
        >
            <SliderTrack id="hue-slider-track" class="grow rounded-full h-2" />
            <SliderThumb
                class="w-4 h-4 bg-white rounded-full focus:outline-none"
                aria-label="Volume"
            />
        </SliderRoot>
        Saturation
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
