<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { Toaster } from '@/components/ui/toast'
import { useToast } from '@/components/ui/toast'
import { enable, disable } from '@tauri-apps/plugin-autostart'
import { load } from '@tauri-apps/plugin-store'
import { listen } from '@tauri-apps/api/event'
import ProcessCard from '@/components/custom/ProcessCard.vue'
import CpuUsage from '@/components/custom/CpuUsage.vue'
import StorageAllocation from '@/components/custom/StorageAllocation.vue'
import HttpPortMonitor from '@/components/custom/HttpPortMonitor.vue'
import DarkModeToggle from '@/components/custom/DarkModeToggle.vue'
import ConfigureSheet from '@/components/custom/ConfigureSheet.vue'
import GetBlockByNumber from '@/components/custom/GetBlockByNumber.vue'
import GetBlockByHash from '@/components/custom/GetBlockByHash.vue'

const trinStats = ref({ 
  cpu: 0,
  radius: 0,
  contentCurrent: 0,
  contentTotal: 0,
  count: 0,
  diskUsage: 0,
  offersIn: 0,
  offersOut: 0,
  acceptsIn: 0,
  acceptsOut: 0,
  validationsIn: 0,
  validationsOut: 0,
})
const { toast } = useToast()
const trinStatus = ref('stopped')
const isLaunching = ref(false)
const config = ref({
  storage: 100,
  httpPort: 8545,
  autostart: true
})

async function toggleTrinProcess() {
  if (trinStatus.value === 'running') {
    await shutdownTrin()
  } else {
    await launchTrin()
  }
}

async function launchTrin() {
  isLaunching.value = true
  try {
    await invoke('launch_trin', { trinConfig: config.value })
    trinStatus.value = 'running'
  } catch (e) {
    toast({
      title: 'Failed to launch Trin.',
      description: 'Error: ' + e,
      variant: 'destructive'
    })
  }
  isLaunching.value = false
}

async function shutdownTrin() {
  isLaunching.value = true
  try {
    await invoke('shutdown_trin')
    trinStatus.value = 'stopped'
  } catch (e) {
    toast({
      title: 'Failed to shutdown Trin.',
      description: 'Error: ' + e,
      variant: 'destructive'
    })
  }
  isLaunching.value = false
}

const updateConfig = async (values) => {
  try {
    const store = await load('config.json', { autoSave: true })
    if (typeof values.storage !== 'undefined') {
      config.value.storage = values.storage
      await store.set('storage', config.value.storage)
    }
    if (typeof values.httpPort !== 'undefined') {
      config.value.httpPort = values.httpPort
      await store.set('httpPort', config.value.httpPort)
    }
    if (typeof values.autostart !== 'undefined') {
      if (values.autostart) {
        await enable()
      } else {
        await disable()
      }
      config.value.autostart = values.autostart
      await store.set('autostart', config.value.autostart)
    }
    toast({ title: 'Configuration updated successfully.' })
  } catch (e) {
    toast({
      title: 'Failed to update configuration.',
      description: 'Error: ' + e,
      variant: 'destructive'
    })
  }
}

listen('trin-crashed', () => {
  trinStatus.value = 'stopped'
  toast({
    title: 'Trin process has crashed! Restarting your node.',
    variant: 'destructive'
  })
  launchTrin()
})

listen('trin-stats', (stats) => {
  console.log("Stats received:", stats)
  trinStats.value.cpu = stats.payload.cpu
  trinStats.value.radius = stats.payload.radius
  trinStats.value.contentCurrent = stats.payload.contentCurrent
  trinStats.value.contentTotal = stats.payload.contentTotal
  trinStats.value.count = stats.payload.count
  trinStats.value.diskUsage = stats.payload.diskUsage
  trinStats.value.offersIn = stats.payload.offersIn
  trinStats.value.offersOut = stats.payload.offersOut
  trinStats.value.acceptsIn = stats.payload.acceptsIn
  trinStats.value.acceptsOut = stats.payload.acceptsOut
  trinStats.value.validationsIn = stats.payload.validationsIn
  trinStats.value.validationsOut = stats.payload.validationsOut
})

onMounted(async () => {
  const store = await load('config.json', { autoSave: true })
  const httpPort = await store.get('httpPort')

  if (!httpPort) {
    store.set('httpPort', 8545)
    store.set('storage', 100)
    store.set('autostart', true)
    config.value.httpPort = 8545
    config.value.storage = 100
    config.value.autostart = true
  } else {
    config.value.httpPort = httpPort
    config.value.storage = await store.get('storage')
    config.value.autostart = await store.get('autostart')
    if (config.value.autostart) {
      await launchTrin()
    }
  }
})
</script>

<template>
  <main class="min-h-screen w-full bg-background">
    <header class="sticky top-0 z-50 w-full border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
      <div class="container flex h-14 items-center">
        <slot name="header">
          <h1 class="text-lg font-semibold">Welcome to Trin...</h1>
        </slot>
      </div>
    </header>
    <div class="container py-6" style="margin: 0 auto; width: fit-content">
      <ProcessCard 
        :trin-status="trinStatus"
        :is-launching="isLaunching"
        @toggle-trin-process="toggleTrinProcess"
      />
	  <GetBlockByNumber :config="config"/>
	  <GetBlockByHash :config="config"/>
      <CpuUsage v-if="trinStatus === 'running'" :trin-stats="trinStats" />
      <StorageAllocation :storage="config.storage" />
      <HttpPortMonitor :port="config.httpPort" />
    </div>
    <DarkModeToggle />
    <ConfigureSheet
      :config="config"
      :trin-status="trinStatus"
      @update-config="updateConfig"
    />
    <Toaster />
  </main>
</template>
