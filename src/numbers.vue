<script setup lang="ts">
    import { ref, onMounted } from 'vue';
    import { invoke } from '@tauri-apps/api/core';
    import SelectBoxx from './components/icons/SelectBoxx.vue';
    import Five from './components/icons/numbers/Five.vue';
    import One from './components/icons/numbers/One.vue';
    import Two from './components/icons/numbers/Two.vue';
    import Three from './components/icons/numbers/Three.vue';
    import Four from './components/icons/numbers/Four.vue';
    import Six from './components/icons/numbers/Six.vue';
    import MenuButton from './components/MenuButton.vue';
    import Seven from './components/icons/numbers/Seven.vue';
    var gridSize = 5;
    var grid = ref<number[]>([]);
    var input = ref<number[]>([]);
    var invalids = ref<number[]>([]);
    var position = ref<number>(0);

    async function newGrid(){
        grid.value = await invoke("create_numbers_game", { "gridSize":gridSize, "difficulty":2 })
        input = ref<number[]>(new Array(gridSize**2).fill(0));
        position = ref<number>(gridSize*gridSize);
        invalids = ref<number[]>(new Array(gridSize**2).fill(0));
    }

    function toggle(i: number, value: number){
    
    if(input.value[i] <= gridSize){
      input.value[i] = value;
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
      else if (e.key == "1"){
        e.preventDefault()
        toggle(position.value, 1);
    }else if (e.key == "2"){
        e.preventDefault()
        toggle(position.value, 2);
    }else if (e.key == "3"){
        e.preventDefault()
        toggle(position.value, 3);
    }else if (e.key == "4"){
        e.preventDefault()
        toggle(position.value, 4);
    }else if (e.key == "5"){
        e.preventDefault()
        toggle(position.value, 5);
    }else if (e.key == "6"){
        e.preventDefault()
        toggle(position.value, 6);
    }else if (e.key == "7"){
        e.preventDefault()
        toggle(position.value, 7);
    }
      else if (e.key == " "){
        e.preventDefault()
        toggle(position.value, 0);
    }

  
    })
});


function get_grid_value(index: number){
    if (grid.value[index] != 0){
        return grid.value[index];
    }
    return input.value[index];
}

function findColour(index: number){
    if (grid.value[index] == 0){
        return "#FFFFFF";
    }
    return "#d7d6d6"
}
</script>

<template>
    <div class="background">
    <MenuButton/>
    <div class="square">
      <div
        class="grid"
        :style="{ gridTemplateColumns: `repeat(${gridSize}, 1fr)` }"
      >
        <div
          v-for="(cell, i) in grid"
          :key="i"
          class="cell"
          :style="{ backgroundColor: findColour(i) }"
          @click.exact="position=i;">        
          <svg v-if="get_grid_value(i) === 1" viewBox="0 0 100 100" style="height:60%; width:60%; z-index: 2;">
            <One/>
        </svg>
        <svg v-if="get_grid_value(i) === 2" viewBox="0 0 100 100" style="height:60%; width:60%; z-index: 2;">
            <Two/>
        </svg>
          <svg v-if="get_grid_value(i) === 3" viewBox="0 0 100 100" style="height:60%; width:60%; z-index: 2;">
            <Three/>
        </svg>
          <svg v-if="get_grid_value(i) === 4" viewBox="0 0 100 100" style="height:60%; width:60%; z-index: 2;">
            <Four/>
        </svg>
        <svg v-if="get_grid_value(i) === 5" viewBox="0 0 100 100" style="height:60%; width:60%; z-index: 2;">
            <Five/>
        </svg>
        <svg v-if="get_grid_value(i) === 6" viewBox="0 0 100 100" style="height:60%; width:60%; z-index: 2;">
            <Six/>
        </svg>
        <svg v-if="get_grid_value(i) === 7" viewBox="0 0 100 100" style="height:60%; width:60%; z-index: 2;">
            <Seven/>
        </svg>
        <svg v-if="position===i" viewBox="0 0 20 20" style="height:90%; width:90%; z-index: 3;" >
            <SelectBoxx :colour="grid[i] != 0? 'white' : '#d7e6ff'"/>
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