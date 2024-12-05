<script setup>
import HeaderComponent from '@/components/custom/HeaderComponent.vue'
import { Toaster } from '@/components/ui/toast'
import { useTrinConfig } from '@/composables/useTrinConfig'
import { useTrinProcess } from '@/composables/useTrinProcess'
import { onMounted } from 'vue'
import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import { useToast } from '@/components/ui/toast'

const { toast } = useToast()
const { initializeConfig } = useTrinConfig()
const { launchTrin } = useTrinProcess()

onMounted(async () => {

  const update = await check();
  if (update) {
    toast({
  	  title: 'Update available',
  	  description: `An update to version ${update.version} is available.`,
	  variant: 'info',
    });
  let downloaded = 0;
  let contentLength = 0;
  await update.downloadAndInstall((event) => {
    switch (event.event) {
      case 'Started':
        contentLength = event.data.contentLength;
		toast({
		  description: `started downloading ${event.data.contentLength} bytes`,
		  variant: 'info',
		});
        break;
      case 'Progress':
        downloaded += event.data.chunkLength;
		toast({
		  description: `downloaded ${downloaded} from ${contentLength}`,
		  variant: 'info',
		});
        break;
      case 'Finished':
	    toast({
		  description: `download finished`,
		  variant: 'success',
		});
        break;
    }
  });
  await relaunch();
  }


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
