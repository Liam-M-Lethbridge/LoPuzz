<template>
    <polygon points="
    20 10, 
    80 10,
    80 25,
    35 25,
    35 42,
    80 42,
    80 90,
    20 90,
    20 75,
    65 75,
    65 57,
    20 57," 
    :fill="colour" stroke="black" stroke-width="4"  />
</template>

<script setup>
defineProps({
  colour: {
    type: String,
    default: "oklch(0.8412 0.0839 203.78)"
  }
})
</script>