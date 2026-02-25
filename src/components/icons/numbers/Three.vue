<template>
  <polygon
    points="
    20 10,
    80 10,
    80 90,
    20 90,
    20 75,
    65 75,
    65 57,
    30 57,
    30 42,
    65 42,
    65 25,
    20 25,
    "
    :fill="colour"
    stroke="black"
    stroke-width="4"
  />
</template>

<script setup>
defineProps({
  colour: {
    type: String,
    default: "oklch(0.9816 0.0921 109.3)",
  },
});
</script>
