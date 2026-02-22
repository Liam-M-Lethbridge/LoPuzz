<template>
    <polygon points="
    20 10, 
    58 10,
    58 75,
    80 75,
    80 90,
    20 90,
    20 75,
    43 75,
    43 25,
    20 25," 
    :fill="colour" stroke="black" stroke-width="4"  />
</template>


<script setup>
defineProps({
  colour: {
    type: String,
    default: "oklch(0.8266 0.0967 19.33)"
  }
})
</script>