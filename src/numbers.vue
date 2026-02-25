<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import SelectBoxx from "./components/icons/SelectBoxx.vue";
import Five from "./components/icons/numbers/Five.vue";
import One from "./components/icons/numbers/One.vue";
import Two from "./components/icons/numbers/Two.vue";
import Three from "./components/icons/numbers/Three.vue";
import Four from "./components/icons/numbers/Four.vue";
import Six from "./components/icons/numbers/Six.vue";
import Seven from "./components/icons/numbers/Seven.vue";
import Menu from "./components/Menu.vue";
import ErrorBox from "./components/icons/ErrorBox.vue";
var gridSize = 5;
var grid = ref<number[]>([]);
var input = ref<number[]>([]);
var invalids = ref<number[]>([]);
var position = ref<number>(0);

/** This function asks the server to create a new grid and resets the relevant variables. */
async function newGrid() {
  grid.value = await invoke("create_numbers_game", {
    gridSize: gridSize,
    difficulty: 2,
  });
  input = ref<number[]>(new Array(gridSize ** 2).fill(0));
  position = ref<number>(gridSize * gridSize);
  invalids = ref<number[]>(new Array(gridSize ** 2).fill(0));
}

/** This function toggles the value of the cell to a number.
 * @param {number} i -The index of the cell.
 * @param {number} value -The value it needs to be changed to.
 */
function toggle(i: number, value: number) {
  if (getGridValue(i) != 0) {
    fixClashes(i, getGridValue(i));
  }
  if (input.value[i] <= gridSize) {
    input.value[i] = value;
  }
  if (getGridValue(i) != 0) {
    findClashes(i, getGridValue(i));
  }
}
/** This function moves the selector icon to a different cell.
 * @param {number} dir -The number indicating the shift.
 */
function move(dir: number) {
  if (position.value == gridSize * gridSize) {
    position.value = 0;
  } else if (
    position.value + dir < gridSize * gridSize &&
    position.value + dir >= 0
  ) {
    if (dir == -1 && position.value % gridSize == 0) {
      return;
    }
    if (dir == 1 && position.value % gridSize == gridSize - 1) {
      return;
    }
    position.value = position.value + dir;
  }
}

// For user input
onMounted(async () => {
  newGrid();
  window.addEventListener("keydown", (e) => {
    if (e.key === "ArrowUp") {
      e.preventDefault();
      move(-gridSize);
    } else if (e.key === "ArrowDown") {
      e.preventDefault();
      move(gridSize);
    } else if (e.key === "ArrowLeft") {
      e.preventDefault();
      move(-1);
    } else if (e.key === "ArrowRight") {
      e.preventDefault();
      move(1);
    } else if (e.key == "1") {
      e.preventDefault();
      toggle(position.value, 1);
    } else if (e.key == "2") {
      e.preventDefault();
      toggle(position.value, 2);
    } else if (e.key == "3") {
      e.preventDefault();
      toggle(position.value, 3);
    } else if (e.key == "4") {
      e.preventDefault();
      toggle(position.value, 4);
    } else if (e.key == "5") {
      e.preventDefault();
      toggle(position.value, 5);
    } else if (e.key == "6") {
      e.preventDefault();
      toggle(position.value, 6);
    } else if (e.key == "7") {
      e.preventDefault();
      toggle(position.value, 7);
    } else if (e.key == " ") {
      e.preventDefault();
      toggle(position.value, 0);
    }
  });
});

/** This function gets the number for the cell.
 * @param {number} index -The index of the cell.
 * @returns {number} The number.
 */
function getGridValue(index: number) {
  if (grid.value[index] != 0) {
    return grid.value[index];
  }
  return input.value[index];
}
/** This function gets the background colour for the cell.
 * @param {number} index -The index of the cell.
 * @returns {string} The colour.
 */
function findColour(index: number) {
  if (grid.value[index] == 0) {
    return "#FFFFFF";
  }
  return "#d7d6d6";
}

/** This function updates any clashing cells. */
function findClashes(index: number, newValue: number) {
  const row = Math.floor(index / gridSize);
  const col = index % gridSize;

  for (let i = 0; i < gridSize; i++) {
    // check along row
    if (
      getGridValue(row * gridSize + i) == newValue &&
      row * gridSize + i != index
    ) {
      console.log("row");
      invalids.value[row * gridSize + i] += 1;
      invalids.value[index] += 1;
    }
    // check along column
    if (
      getGridValue(gridSize * i + col) == newValue &&
      gridSize * i + col != index
    ) {
      console.log("column");
      invalids.value[gridSize * i + col] += 1;
      invalids.value[index] += 1;
    }
  }
  // check along diagonals
  for (let i = 1; i < gridSize; i++) {
    if (row - i >= 0) {
      if (col - i >= 0) {
        if (getGridValue((row - i) * gridSize + (col - i)) == newValue) {
          console.log("up left");
          invalids.value[(row - i) * gridSize + (col - i)] += 1;
          invalids.value[index] += 1;
        }
      }
      if (col + i < gridSize) {
        if (getGridValue((row - i) * gridSize + (col + i)) == newValue) {
          console.log("up right");
          invalids.value[(row - i) * gridSize + (col + i)] += 1;
          invalids.value[index] += 1;
        }
      }
    }
    if (row + i < gridSize) {
      if (col - i >= 0) {
        if (getGridValue((row + i) * gridSize + (col - i)) == newValue) {
          console.log("down left");
          invalids.value[(row + i) * gridSize + (col - i)] += 1;
          invalids.value[index] += 1;
        }
      }
      if (col + i < gridSize) {
        if (getGridValue((row + i) * gridSize + (col + i)) == newValue) {
          console.log("down right");
          invalids.value[(row + i) * gridSize + (col + i)] += 1;
          invalids.value[index] += 1;
        }
      }
    }
  }
}

/** This function updates any previously-clashing cells. */
function fixClashes(index: number, newValue: number) {
  const row = Math.floor(index / gridSize);
  const col = index % gridSize;

  for (let i = 0; i < gridSize; i++) {
    // check along row
    if (
      getGridValue(row * gridSize + i) == newValue &&
      row * gridSize + i != index
    ) {
      invalids.value[row * gridSize + i] -= 1;
      invalids.value[index] -= 1;
    }
    // check along column
    if (
      getGridValue(gridSize * i + col) == newValue &&
      gridSize * i + col != index
    ) {
      invalids.value[gridSize * i + col] -= 1;
      invalids.value[index] -= 1;
    }
  }
  // check along diagonals
  for (let i = 1; i < gridSize; i++) {
    if (row - i >= 0) {
      if (col - i >= 0) {
        if (getGridValue((row - i) * gridSize + (col - i)) == newValue) {
          invalids.value[(row - i) * gridSize + (col - i)] -= 1;
          invalids.value[index] -= 1;
        }
      }
      if (col + i < gridSize) {
        if (getGridValue((row - i) * gridSize + (col + i)) == newValue) {
          invalids.value[(row - i) * gridSize + (col + i)] -= 1;
          invalids.value[index] -= 1;
        }
      }
    }
    if (row + i < gridSize) {
      if (col - i >= 0) {
        if (getGridValue((row + i) * gridSize + (col - i)) == newValue) {
          invalids.value[(row + i) * gridSize + (col - i)] -= 1;
          invalids.value[index] -= 1;
        }
      }
      if (col + i < gridSize) {
        if (getGridValue((row + i) * gridSize + (col + i)) == newValue) {
          invalids.value[(row + i) * gridSize + (col + i)] -= 1;
          invalids.value[index] -= 1;
        }
      }
    }
  }
}
</script>

<template>
  <div class="background">
    <Menu />
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
          @click.exact="position = i"
        >
          <svg
            v-if="getGridValue(i) === 1"
            viewBox="0 0 100 100"
            style="height: 60%; width: 60%; z-index: 2"
          >
            <One />
          </svg>
          <svg
            v-if="getGridValue(i) === 2"
            viewBox="0 0 100 100"
            style="height: 60%; width: 60%; z-index: 2"
          >
            <Two />
          </svg>
          <svg
            v-if="getGridValue(i) === 3"
            viewBox="0 0 100 100"
            style="height: 60%; width: 60%; z-index: 2"
          >
            <Three />
          </svg>
          <svg
            v-if="getGridValue(i) === 4"
            viewBox="0 0 100 100"
            style="height: 60%; width: 60%; z-index: 2"
          >
            <Four />
          </svg>
          <svg
            v-if="getGridValue(i) === 5"
            viewBox="0 0 100 100"
            style="height: 60%; width: 60%; z-index: 2"
          >
            <Five />
          </svg>
          <svg
            v-if="getGridValue(i) === 6"
            viewBox="0 0 100 100"
            style="height: 60%; width: 60%; z-index: 2"
          >
            <Six />
          </svg>
          <svg
            v-if="getGridValue(i) === 7"
            viewBox="0 0 100 100"
            style="height: 60%; width: 60%; z-index: 2"
          >
            <Seven />
          </svg>
          <svg
            v-if="position === i"
            viewBox="0 0 20 20"
            style="height: 90%; width: 90%; z-index: 3"
          >
            <SelectBoxx :colour="grid[i] != 0 ? 'white' : '#d7e6ff'" />
          </svg>
          <svg
            v-if="invalids[i] > 0"
            viewBox="0 0 20 20"
            style="height: 90%; width: 90%; z-index: 1"
          >
            <ErrorBox />
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
.grid {
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

.background {
  display: flex;
  justify-content: center;
  align-items: center;

  width: 100vw;
  height: 100vh;
  background-color: #d7e6ff;
}
.cell {
  display: flex;
  position: relative;
  aspect-ratio: 1 / 1;
  border: 1px solid #00000055;
  box-sizing: border-box;
  justify-content: center;
  align-items: center;
}
</style>
