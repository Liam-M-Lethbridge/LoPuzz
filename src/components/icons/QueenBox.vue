<script setup>
  import QueenIcon from "./QueenIcon.vue";
</script>
<template>
    <div class="QueenSquare">
        <svg viewBox="0 0 32 22" height="min(60%, 60%)" width="min(60%, 60%)" y="1em">
            <QueenIcon></QueenIcon>
        </svg>
    </div>
</template>

<style scoped>
.QueenSquare{
  display: flex;
  justify-content: center;
  align-items: center;
  background-color: #FDD7FF;
  border: 4px solid #000000;
  border-radius: 20%;
  top:4px; 
  left:30px;
  animation-duration: 6s;
  animation-name: appear-and-dissappear;
  animation-iteration-count: infinite;
}
</style>