import { useToast } from '@/components/ui/toast'
import { disable, enable } from '@tauri-apps/plugin-autostart'
import { load } from '@tauri-apps/plugin-store'
import { ref } from 'vue'

// these values don't matter, they will be overwritten by initializeConfig
const config = ref({
  storage: 2000,
  httpPort: 8545,
  autostart: true,
  trustedBlockRoot: '0x'
})

export function useTrinConfig() {
  const { toast } = useToast()

  async function updateConfig(values) {
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
      if (typeof values.trustedBlockRoot !== 'undefined') {
        config.value.trustedBlockRoot = values.trustedBlockRoot
        await store.set('trustedBlockRoot', config.value.trustedBlockRoot)
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

  async function initializeConfig() {
    console.log('initializeConfig')
    const store = await load('config.json', { autoSave: true })
    console.log('config initialized')
    const httpPort = await store.get('httpPort')

    if (!httpPort) {
      store.set('httpPort', 8545)
      store.set('storage', 2000)
      store.set('autostart', true)
      store.set('trustedBlockRoot', '0x')
      config.value.httpPort = 8545
      config.value.storage = 2000
      config.value.autostart = true
      config.value.trustedBlockRoot = '0x'
    } else {
      config.value.httpPort = httpPort
      config.value.storage = await store.get('storage')
      config.value.autostart = await store.get('autostart')
      config.value.trustedBlockRoot = await store.get('trustedBlockRoot')
    }
    return config.value
  }

  return {
    config,
    updateConfig,
    initializeConfig
  }
}
