<script setup lang="ts">
const color = ref({ hue: 0, saturation: 0 });
watch(color, async (newColor) => {
    let color: SetColor = {
        type: "SetColor",
        hue: newColor.hue,
        saturation: newColor.saturation,
    };

    await $fetch("/api/update", {
        method: "POST",
        body: JSON.stringify(color),
    });
});

const brightness = ref(0);
watch(brightness, async (newBrightness) => {
    let brightness: SetBrightness = {
        type: "SetBrightness",
        brightness: newBrightness,
    };

    await $fetch("/api/update", {
        method: "POST",
        body: JSON.stringify(brightness),
    });
});

const active = ref(false);
watch(active, async (newActive) => {
    let activation: Activate = { type: "Activate", on: newActive };

    await $fetch("/api/update", {
        method: "POST",
        body: JSON.stringify(activation),
    });
});

const colorHex = computed(() => {
    return hsbToHex(color.value.hue, color.value.saturation, brightness.value);
});
</script>
<template>
    <div>
        <div :style="{ backgroundColor: colorHex }" class="h-20"></div>
        <div class="bg-neutral-800 h-dvh p-4 flex flex-col gap-4">
            <SettingsColor @update="(values) => (color = values)" />
            <SettingsBrightness @update="(value) => (brightness = value)" />
            <SettingsActivation @update="(value) => (active = value)" />
        </div>
    </div>
</template>
