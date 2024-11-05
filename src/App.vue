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
  	  description: `An update to version ${update.version} is available. Click here to install it.`,
	  variant: 'info',
	  //onClick: () => installUpdate(update)
    });
//  let downloaded = 0;
 // let contentLength = 0;
  // alternatively we could also call update.download() and update.install() separately
  //await update.downloadAndInstall((event) => {
   // switch (event.event) {
    //  case 'Started':
     //   contentLength = event.data.contentLength;
      //  console.log(`started downloading ${event.data.contentLength} bytes`);
       // break;
      //case 'Progress':
       // downloaded += event.data.chunkLength;
        //console.log(`downloaded ${downloaded} from ${contentLength}`);
        //break;
      //case 'Finished':
       // console.log('download finished');
        //break;
   // }
 // });

  //console.log('update installed');
  //await relaunch();
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
