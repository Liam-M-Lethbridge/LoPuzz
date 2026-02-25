<script setup>
import StarIcon from "./numbers/Five.vue";
</script>
<template>
  <div class="numbersSquare">
    <svg viewBox="0 0 100 100" style="height: 60%; width: 60%" y="1em">
      <StarIcon />
    </svg>
  </div>
</template>

<style scoped>
.numbersSquare {
  display: flex;
  justify-content: center;
  align-items: center;
  background-color: #d7e6ff;
  border: 4px solid #000000;
  border-radius: 20%;
  top: 4px;
  left: 30px;
}
</style>
