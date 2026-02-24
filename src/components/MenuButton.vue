<script setup lang="ts">
import { ref } from 'vue';

let hover = ref(false);
</script>

<template>
    <div class="button" @mouseover="hover = true" @mouseleave="hover = false" :style="{ active: hover }">
        <svg viewBox="0 0 100 100">
            <polygon points="20 20, 80 20 80 30, 20 30" fill="#FDFDFD"></polygon>
            <polygon points="20 45, 80 45 80 55, 20 55" fill="#FDFDFD"></polygon>
            <polygon points="20 70, 80 70 80 80, 20 80" fill="#FDFDFD"></polygon>
        </svg>
    </div>

</template>


<style scoped>

.button{
    background-color:#DDDDDD;
    border-bottom: 4px solid black ;
    border-right: 4px solid black ;
    border-top: 4px solid black ;
    border-bottom-right-radius: 8px;
    border-top-right-radius: 8px;
    position: absolute;
    width: min(5vw, 5vh);
    height: min(5vw, 5vh);
    top: min(2vw, 2vh);
    left: 0;

}
.button:hover{
    background-color:#EEEEEE;
}
</style>