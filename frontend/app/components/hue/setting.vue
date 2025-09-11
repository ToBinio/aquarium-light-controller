<script setup lang="ts">
const hexColor = ref("#000000");

async function setColor() {
    await $fetch("/api/update", {
        method: "POST",
        body: JSON.stringify({ type: "SetColor", hex: hexColor.value }),
    });
}
</script>
<template>
    <div>
        <div class="text-white">
            {{ hexColor }}
        </div>
        <HueSlider v-model:hex-color="hexColor" />
        <button
            @click="setColor"
            class="bg-neutral-500 hover:bg-neutral-700 text-white font-bold py-1 px-2 mt-2 rounded"
        >
            update Color
        </button>
    </div>
</template>
