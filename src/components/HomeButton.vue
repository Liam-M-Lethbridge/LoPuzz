<script setup lang="ts">
import { ref } from 'vue';
let hover = ref(false);
</script>

<template>
    <div class="button" @mouseover="hover = true" @mouseleave="hover = false">
        <svg viewBox="0 0 100 100">
            <path d=" 
            M50 25 
            L80 48 
            L75 48 
            L75 80 
            L45 80 
            L45 55 
            L33 55  
            L33 80 
            L25 80 
            L25 48 
            L20 48
            Z

            M53 57
            L68 57
            L68 72
            L53 72
            Z
            " fill="white" stroke="black" stroke-width="1" fill-rule="evenodd"></path>
        </svg>
    </div>

</template>


<style scoped>
.button {
  background-color: #f46767;
  border: 4px solid black;

  border-radius: 8px;
    width: 10vh;
    height: 10vh;
}

.button:hover{
    background-color:rgb(236, 156, 156);
}
</style>