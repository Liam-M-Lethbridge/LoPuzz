<template>
  <polygon
    points="
    80 10, 
    20 10,
    20 25,
    65 25,
    65 42,
    20 42,
    20 90,
    80 90,
    80 75,
    35 75,
    35 57,
    80 57,"
    :fill="colour"
    stroke="black"
    stroke-width="4"
  />
</template>

<script setup>
defineProps({
  colour: {
    type: String,
    default: "oklch(0.8995 0.0782 71.55)",
  },
});
</script>
