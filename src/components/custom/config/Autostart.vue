<script setup>
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Switch } from '@/components/ui/switch'
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from '@/components/ui/tooltip'
import { useTrinConfig } from '@/composables/useTrinConfig'
import { useTrinProcess } from '@/composables/useTrinProcess'
import { ref } from 'vue'

const { config, updateConfig } = useTrinConfig()
const { trinStatus } = useTrinProcess()

const updateAutostart = (value) => {
  updateConfig({ autostart: value })
}
</script>

<template>
  <Card class="p-4">
    <CardHeader class="flex flex-row items-center justify-between pb-2">
      <CardTitle class="text-sm font-medium">Autostart</CardTitle>
      <div class="flex items-center gap-2">
        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger as-child>
              <div>
                <Switch
                  :checked="config.autostart"
                  @update:checked="updateAutostart"
                  @click="(e) => updateAutostart(!config.autostart)"
                  :disabled="trinStatus === 'running'"
                />
              </div>
            </TooltipTrigger>
            <TooltipContent v-if="trinStatus === 'running'">
              <p>Trin must be stopped to modify autostart settings.</p>
            </TooltipContent>
          </Tooltip>
        </TooltipProvider>
      </div>
    </CardHeader>
    <CardContent>
      <p class="text-xs text-muted-foreground">
        When enabled, Trin will automatically start when you launch the application.
        <br />
        And this app will automatically launch when you boot your computer.
      </p>
    </CardContent>
  </Card>
</template>
