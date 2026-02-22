<script setup lang="ts">
    import CircleIcon from './components/icons/CircleIcon.vue';
    import StarIcon from './components/icons/StarIcon.vue';
    import SquareIcon from './components/icons/SquareIcon.vue';
    import TriangleIcon from './components/icons/TriangleIcon.vue';
    import { ref, onMounted } from 'vue';
    import { invoke } from '@tauri-apps/api/core';
import HexagonIcon from './components/icons/HexagonIcon.vue';
import SelectBoxx from './components/icons/SelectBoxx.vue';
    var gridSize = 5;
    var grid = ref<number[]>([]);
    var input = ref<number[]>([]);
    var invalids = ref<number[]>([]);
    var position = ref<number>(0);

    async function newGrid(){
        grid.value = await invoke("create_shapes_game", { "gridSize":gridSize, "difficulty":0 })
        input = ref<number[]>(new Array(gridSize**2).fill(0));
        position = ref<number>(gridSize*gridSize);
        invalids = ref<number[]>(new Array(gridSize**2).fill(0));
    }

    function toggle(i: number){
    input.value[i] = input.value[i]+1;
    if(input.value[i] > gridSize){
        input.value[i] =0;
    }

    
  }

    function move(dir: number){
    if(position.value == gridSize*gridSize){
        position.value = 0;
    }
    else if(position.value+dir < gridSize*gridSize && position.value+dir >=0){
        if (dir == -1 && position.value%gridSize == 0){
            return;
        }
        if (dir == 1 && position.value%gridSize == gridSize-1){
            return;
        }
        position.value = position.value + dir;
        }
    }
    onMounted(async () => {
    newGrid();
    window.addEventListener("keydown", e => {
      if (e.key === "ArrowUp") {
        e.preventDefault()
        move(-gridSize);
      }
      else if (e.key === "ArrowDown") {
        e.preventDefault()
        move(gridSize);
      }
      else if (e.key === "ArrowLeft") {
        e.preventDefault()
        move(-1);
      }
      else if (e.key === "ArrowRight") {
        e.preventDefault()
        move(1);
      }

      else if (e.key == " "){
        e.preventDefault()
        toggle(position.value);
    }

  
    })
});


function get_grid_value(index: number){
    if (grid.value[index] != 0){
        return grid.value[index];
    }
    return input.value[index];
}
</script>

<template>
    <div class="background">
    <div class="square">
      <div
        class="grid"
        :style="{ gridTemplateColumns: `repeat(${gridSize}, 1fr)` }"
      >
        <div
          v-for="(cell, i) in grid"
          :key="i"
          class="cell"
          @click.shift="position=i; toggle(i)"
          @click.exact="position=i; toggle(i)">
        <svg v-if="get_grid_value(i) === 1" viewBox="0 0 100 100" style="height:60%; width:60%; z-index: 2;">
            <CircleIcon/>
        </svg>
        <svg v-if="get_grid_value(i) === 2" viewBox="0 0 100 100" style="height:60%; width:60%; z-index: 2;">
            <TriangleIcon/>
        </svg>
          <svg v-if="get_grid_value(i) === 3" viewBox="0 0 100 100" style="height:60%; width:60%; z-index: 2;">
            <SquareIcon/>
        </svg>
          <svg v-if="get_grid_value(i) === 4" viewBox="0 0 100 100" style="height:60%; width:60%; z-index: 2;">
            <StarIcon/>
        </svg>
            <svg v-if="get_grid_value(i) === 5" viewBox="0 0 100 100" style="height:60%; width:60%; z-index: 2;">
            <HexagonIcon/>
        </svg>
        <svg v-if="position===i" viewBox="0 0 20 20" style="height:90%; width:90%; z-index: 3;">
            <SelectBoxx/>
          </svg>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.cell svg {
  position: absolute;
  inset: 0;
  margin: auto;
}
.grid{
  display: grid;
  width: 100%;
  height: 100%;
  border: 4px solid #000000;
  border-radius: 1%;
}
.square {
  display: block;
  justify-content: center;
  align-items: center;

  width: min(90vw, 90vh);
  aspect-ratio: 1 / 1;
}

.background{
    display: flex;
    justify-content: center;
    align-items: center;

    width: 100vw;
    height: 100vh;
    background-color: #d7e6ff;

}
.cell{
  display: flex;
  position: relative;
  aspect-ratio: 1 / 1;
  border: 1px solid #00000055;
  box-sizing: border-box;
  justify-content: center; 
  align-items: center;
}

</style>