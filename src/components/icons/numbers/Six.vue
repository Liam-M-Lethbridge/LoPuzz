<template>
    <path d="
    M20 10 
    L80 10
    L80 25
    L35 25
    L35 42
    L80 42
    L80 90
    L20 90
    Z

    M65 57
    L65 75
    L35 75
    L35 57
    Z
    "
    :fill="colour" stroke="black" stroke-width="4" fill-rule="evenodd"  />
</template>

<script setup>
defineProps({
  colour: {
    type: String,
    default: "oklch(0.8156 0.0925 260.13)"
  }
})
</script>