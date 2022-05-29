<template>
  <div>
    <button @click="a += 1">Click me with count {{ a }}</button>
  </div>
</template>

<script lang="ts" setup>
import { ref } from "vue";

let a = ref(1);
</script>
