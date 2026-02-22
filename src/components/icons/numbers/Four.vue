<template>
    <path d="
    M50 10 
    L65 10
    L65 55
    L80 55
    L80 70
    L65 70
    L65 90
    L50 90
    L50 70
    L20 70
    L20 55
    Z

    M50 32
    L50 55
    L37 55
    Z
    " :fill="colour" stroke="black" stroke-width="4" fill-rule="evenodd" />
</template>


<script setup>
defineProps({
  colour: {
    type: String,
    default: "oklch(0.8411 0.1487 139.76)"
  }
})
</script>