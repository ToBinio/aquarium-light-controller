<script setup lang="ts">
import { SwitchRoot, SwitchThumb } from "reka-ui";
import type { Activate } from "~~/shared/updates";

const on = ref(false);

watch(on, async () => {
    let activation: Activate = { type: "Activate", on: on.value };

    await $fetch("/api/update", {
        method: "POST",
        body: JSON.stringify(activation),
    });
});
</script>
<template>
    <div class="text-gray-400">
        Active
        <SwitchRoot
            id="airplane-mode"
            v-model="on"
            class="w-[32px] h-[20px] shadow-sm flex bg-stone-800 data-[state=checked]:bg-stone-600 border border-stone-600 rounded-full transition-all"
        >
            <SwitchThumb
                class="w-3.5 h-3.5 my-auto bg-white rounded-full transition-transform translate-x-0.5 data-[state=checked]:translate-x-full"
            />
        </SwitchRoot>
    </div>
</template>
