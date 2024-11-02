<script setup>
import HeaderComponent from '@/components/custom/HeaderComponent.vue'
import { Toaster } from '@/components/ui/toast'
import { useTrinConfig } from '@/composables/useTrinConfig'
import { useTrinProcess } from '@/composables/useTrinProcess'
import { onMounted } from 'vue'

const { initializeConfig } = useTrinConfig()
const { launchTrin } = useTrinProcess()

onMounted(async () => {
  const initialConfig = await initializeConfig()
  if (initialConfig.autostart) {
    await launchTrin(initialConfig)
  }
})
</script>

<template>
  <main class="min-h-screen w-full bg-background">
    <HeaderComponent />
    <keep-alive>
      <RouterView />
    </keep-alive>
    <Toaster />
  </main>
</template>
