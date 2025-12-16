<script setup lang="ts">
import { SliderRoot, SliderThumb, SliderTrack } from "reka-ui";

let emits = defineEmits<{
    (e: "update", brightness: number): void;
}>();

const sliderValue = ref([0]);

let timeoutId: NodeJS.Timeout | null = null;
watch(sliderValue, async () => {
    if (timeoutId) clearTimeout(timeoutId);
    timeoutId = setTimeout(() => {
        emits("update", sliderValue.value[0]!);
    }, 500);
});
</script>
<template>
    <div class="text-gray-400">
        Brightness
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
    </div>
</template>
