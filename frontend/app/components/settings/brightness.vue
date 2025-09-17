<script setup lang="ts">
import { SliderRoot, SliderThumb, SliderTrack } from "reka-ui";
import type { SetBrightness } from "~~/shared/updates";

async function setBrightness() {
    let brightness: SetBrightness = {
        type: "SetBrightness",
        brightness: sliderValue.value[0]!,
    };

    await $fetch("/api/update", {
        method: "POST",
        body: JSON.stringify(brightness),
    });
}

const sliderValue = ref([0]);
</script>
<template>
    <div>
        <SliderRoot
            v-model="sliderValue"
            class="relative flex items-center select-none touch-none h-5"
            :max="1"
            :step="0.01"
        >
            <SliderTrack class="grow rounded-full h-2 bg-gray-600" />
            <SliderThumb
                class="w-4 h-4 bg-white rounded-full focus:outline-none"
                aria-label="Volume"
            />
        </SliderRoot>
        <button
            @click="setBrightness"
            class="bg-neutral-500 hover:bg-neutral-700 text-white font-bold py-1 px-2 mt-2 rounded"
        >
            update Brightness
        </button>
    </div>
</template>
