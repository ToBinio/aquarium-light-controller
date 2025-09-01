<script setup lang="ts">
const hexColor = ref("#000000");
</script>
<template>
    <div class="bg-neutral-800 h-dvh p-4">
        <div class="text-white">
            {{ hexColor }}
        </div>
        <HueSlider v-model:hex-color="hexColor" />
    </div>
</template>
