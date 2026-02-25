<script setup>
import QueenIcon from "./QueenIcon.vue";
</script>
<template>
  <div class="QueenSquare">
    <svg viewBox="0 0 32 22" style="height: 60%; width: 60%" y="1em">
      <QueenIcon />
    </svg>
  </div>
</template>

<style scoped>
.QueenSquare {
  display: flex;
  justify-content: center;
  align-items: center;
  background-color: #fdd7ff;
  border: 4px solid #000000;
  border-radius: 20%;
  top: 4px;
  left: 30px;
}
</style>
