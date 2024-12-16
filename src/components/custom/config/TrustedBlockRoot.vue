<script setup>
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger
} from '@/components/ui/dialog'
import { Form, FormControl, FormField, FormItem, FormMessage } from '@/components/ui/form'
import { Input } from '@/components/ui/input'
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from '@/components/ui/tooltip'
import { formatBlockRoot } from '@/components/utils/formatHex'
import { useTrinConfig } from '@/composables/useTrinConfig'
import { useTrinProcess } from '@/composables/useTrinProcess'
import { Edit2 } from 'lucide-vue-next'
import { ref, watch } from 'vue'

const { config, updateConfig } = useTrinConfig()
const { trinStatus } = useTrinProcess()

const isDialogOpen = ref(false)
const tempRoot = ref(config.value.trustedBlockRoot)

const errorMessage = ref('')
const validateHexString = (value) => {
  const hexRegex = /^0x[0-9a-fA-F]{64}$/
  return hexRegex.test(value)
}

const updateBlockRoot = () => {
  if (!validateHexString(tempRoot.value)) {
    errorMessage.value = 'Must be a 32-byte hex string starting with 0x'
    return
  }
  errorMessage.value = ''
  updateConfig({ trustedBlockRoot: tempRoot.value })
  isDialogOpen.value = false
}

watch(isDialogOpen, (value) => {
  if (value) {
    // Reset temp value to current storage when dialog opens
    tempRoot.value = config.value.trustedBlockRoot
    errorMessage.value = ''
  }
})
</script>

<template>
  <Card class="p-4">
    <CardHeader class="flex flex-row items-center justify-between pb-2">
      <CardTitle class="text-sm font-medium">Trusted Block Root</CardTitle>
      <div class="flex items-center gap-2">
        <TooltipProvider>
          <Dialog v-model:open="isDialogOpen">
            <Tooltip>
              <TooltipTrigger as-child>
                <div>
                  <!-- Wrapper div needed for disabled button tooltip -->
                  <DialogTrigger as-child>
                    <Button
                      variant="ghost"
                      size="icon"
                      class="h-8 w-8"
                      :disabled="trinStatus === 'running'"
                    >
                      <Edit2 class="h-4 w-4" />
                    </Button>
                  </DialogTrigger>
                </div>
              </TooltipTrigger>
              <TooltipContent v-if="trinStatus === 'running'">
                <p>Trin must be stopped to edit trusted block root.</p>
              </TooltipContent>
            </Tooltip>

            <DialogContent>
              <DialogHeader>
                <DialogTitle>Edit Trusted Block Root</DialogTitle>
              </DialogHeader>
              <Form class="py-4">
                <FormField name="storage">
                  <FormItem>
                    <FormControl>
                      <Input v-model="tempRoot" placeholder="0x..." class="font-mono" />
                    </FormControl>
                    <FormMessage v-if="errorMessage" type="error">{{ errorMessage }}</FormMessage>
                  </FormItem>
                </FormField>
              </Form>
              <DialogFooter>
                <Button variant="outline" @click="isDialogOpen = false"> Cancel </Button>
                <Button @click="updateBlockRoot"> Save Changes </Button>
              </DialogFooter>
            </DialogContent>
          </Dialog>
        </TooltipProvider>
      </div>
    </CardHeader>
    <CardContent>
      <div class="text-2xl font-bold font-mono">{{ formatBlockRoot(config.trustedBlockRoot) }}</div>
      <p class="text-xs text-muted-foreground">
        Trusted block root to bootstrap beacon network sync.
      </p>
      <p class="text-xs text-muted-foreground">
        You can find the latest trusted block roots
        <a href="https://sync-mainnet.beaconcha.in/" target="_blank" class="text-blue-500">here</a>.
      </p>
    </CardContent>
  </Card>
</template>
