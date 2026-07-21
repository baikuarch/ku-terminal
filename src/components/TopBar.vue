<script setup lang="ts">
defineEmits<{ (e: "open-palette"): void }>();
</script>

<template>
  <header
    class="h-12 grid grid-cols-[240px_1fr_240px] items-center px-4 bg-app border-b border-border-subtle"
  >
    <div class="flex items-center gap-2">
      <div class="i-lucide-square-terminal text-primary text-18px" />
      <span class="font-600 text-15px tracking-wide">Ku Terminal</span>
    </div>

    <div
      class="justify-self-center w-full max-w-420px flex items-center gap-2 px-3 h-8 bg-input border border-border-subtle rounded-md cursor-pointer hover:border-border-default"
      @click="$emit('open-palette')"
    >
      <div class="i-lucide-search text-txt-muted text-14px" />
      <span class="flex-1 text-txt-muted text-13px">Search resources, commands…</span>
      <span
        class="text-11px text-txt-muted border border-border-default rounded-sm px-1.5 py-px"
        >Ctrl K</span
      >
    </div>

    <div class="justify-self-end flex gap-2">
      <button class="icon-btn" title="Notifications">
        <div class="i-lucide-bell text-15px" />
      </button>
      <button class="icon-btn" title="Layout">
        <div class="i-lucide-layout-panel-left text-15px" />
      </button>
      <button class="icon-btn text-primary! hover:text-primary-hover!" title="Account">
        <div class="i-lucide-circle-user-round text-16px" />
      </button>
    </div>
  </header>
</template>
