<template>
  <polygon
    points="
    20 10, 
    80 10,
    80 25,
    50 80,
    32 80,
    62 25,
    20 25"
    :fill="colour"
    stroke="black"
    stroke-width="4"
  />
</template>

<script setup>
defineProps({
  colour: {
    type: String,
    default: "oklch(0.8133 0.1332 319.76)",
  },
});
</script>
