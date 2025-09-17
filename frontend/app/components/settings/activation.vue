<script setup lang="ts">
import type { Activate } from "~~/shared/updates";

async function setActivation() {
    let activation: Activate = { type: "Activate", on: on.value };

    await $fetch("/api/update", {
        method: "POST",
        body: JSON.stringify(activation),
    });
}

const on = ref(false);
</script>
<template>
    <div>
        <input type="checkbox" v-model="on" />
        <button
            @click="setActivation"
            class="bg-neutral-500 hover:bg-neutral-700 text-white font-bold py-1 px-2 mt-2 rounded"
        >
            update Activation
        </button>
    </div>
</template>
