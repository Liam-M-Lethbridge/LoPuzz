<script setup lang="ts">
import { ref } from 'vue';
let hover = ref(false);
</script>

<template>
    <div class="button" @mouseover="hover = true" @mouseleave="hover = false">
        <svg viewBox="0 0 100 50">
            <polygon points=" 10 25, 30 45, 30 20, 80 20, 80 30, 30 30, 30 5" fill="white"></polygon>
        </svg>
    </div>

</template>


<style scoped>


</style>