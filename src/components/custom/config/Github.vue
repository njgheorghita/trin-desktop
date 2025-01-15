<script setup>
import { Button } from '@/components/ui/button'
import { useToast } from '@/components/ui/toast'
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from '@/components/ui/tooltip'
import { open } from '@tauri-apps/plugin-shell'
import { useColorMode } from '@vueuse/core'
import { Github } from 'lucide-vue-next'

const { toast } = useToast()
const mode = useColorMode()
const issuesUrl = 'https://github.com/njgheorghita/trin-desktop/issues'

const openGithubIssues = async () => {
  try {
    await open(issuesUrl)
  } catch (error) {
    toast({
      title: 'Error opening GitHub issues',
      description: error,
      variant: 'destructive'
    })
  }
}
</script>

<template>
  <TooltipProvider>
    <Tooltip>
      <TooltipTrigger as-child>
        <Button variant="outline" id="darkMode" @click="openGithubIssues">
          <Github
            class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"
          />
          <Github
            class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
          />
          <span class="sr-only">Toggle theme</span>
        </Button>
      </TooltipTrigger>
      <TooltipContent>
        <span>Submit a feature request or bug report.</span>
      </TooltipContent>
    </Tooltip>
  </TooltipProvider>
</template>
