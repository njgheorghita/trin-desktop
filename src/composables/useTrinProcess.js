import { useToast } from '@/components/ui/toast'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { ref } from 'vue'
import { useTrinConfig } from './useTrinConfig'

const { config } = useTrinConfig()
const trinStatus = ref('stopped')
const isLaunching = ref(false)

export function useTrinProcess() {
  const { toast } = useToast()

  async function toggleTrinProcess() {
    if (trinStatus.value === 'running') {
      await shutdownTrin()
    } else {
      await launchTrin(config.value)
    }
  }

  async function launchTrin(config) {
    isLaunching.value = true
    try {
      await invoke('launch_trin', { trinConfig: config })
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

  // Set up crash listener
  listen('trin-crashed', () => {
    trinStatus.value = 'stopped'
    toast({
      title: 'Trin process has crashed! Restarting your node.',
      variant: 'destructive'
    })
    launchTrin()
  })

  return {
    trinStatus,
    isLaunching,
    toggleTrinProcess,
    launchTrin,
    shutdownTrin
  }
}
