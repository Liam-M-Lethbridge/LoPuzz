<template>
  <polygon points="1 1,1 5, 2 5, 2 2, 5 2, 5 1" :fill="colour" />
  <polygon points="19 1, 19 5, 18 5, 18 2, 15 2, 15 1" :fill="colour" />
  <polygon points="1 19, 1 15, 2 15, 2 18, 5 18, 5 19" :fill="colour" />
  <polygon points="19 19,19 15, 18 15, 18 18, 15 18, 15 19" :fill="colour" />
</template>

<script setup>
defineProps({
  colour: {
    type: String,
    default: "white",
  },
});
</script>
