<script setup lang="ts">
const hexColor = ref("#000000");

async function updateLight() {
    await $fetch("/api/update", {
        method: "POST",
        body: JSON.stringify({ color: hexColor.value, brightness: 0.5 }),
    });
}
</script>
<template>
    <div class="bg-neutral-800 h-dvh p-4">
        <div class="text-white">
            {{ hexColor }}
        </div>
        <HueSlider v-model:hex-color="hexColor" />
        <button
            @click="updateLight"
            class="bg-neutral-500 hover:bg-neutral-700 text-white font-bold py-1 px-2 mt-2 rounded"
        >
            update
        </button>
    </div>
</template>
