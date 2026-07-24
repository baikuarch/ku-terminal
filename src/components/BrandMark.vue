<script setup lang="ts">
import logoUrl from "../../src-tauri/icons/ku-terminal.svg";
</script>

<template>
  <img class="brand-mark" :src="logoUrl" alt="Ku Terminal" />
</template>

<style scoped>
.brand-mark {
  display: block;
  width: 32px;
  height: 32px;
  user-select: none;
  -webkit-user-drag: none;
}
</style>
