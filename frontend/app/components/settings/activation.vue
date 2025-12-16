<script setup lang="ts">
import { SwitchRoot, SwitchThumb } from "reka-ui";
import type { Activate } from "~~/shared/updates";

let emits = defineEmits<{
    (e: "update", active: boolean): void;
}>();

const on = ref(false);

watch(on, async () => {
    emits("update", on.value);
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
