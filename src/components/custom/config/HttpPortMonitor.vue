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
import { Form, FormControl, FormField, FormItem } from '@/components/ui/form'
import {
  NumberField,
  NumberFieldContent,
  NumberFieldDecrement,
  NumberFieldIncrement,
  NumberFieldInput
} from '@/components/ui/number-field'
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from '@/components/ui/tooltip'
import { useTrinConfig } from '@/composables/useTrinConfig'
import { useTrinProcess } from '@/composables/useTrinProcess'
import { Edit2 } from 'lucide-vue-next'
import { ref, watch } from 'vue'

const { config, updateConfig } = useTrinConfig()
const { trinStatus } = useTrinProcess()

const isDialogOpen = ref(false)
const tempPort = ref(config.httpPort)

const updatePort = () => {
  updateConfig({ httpPort: tempPort.value })
  isDialogOpen.value = false
}

watch(isDialogOpen, (value) => {
  if (value) {
    // Reset temp value to current port when dialog opens
    tempPort.value = config.value.httpPort
  }
})
</script>

<template>
  <Card class="p-4">
    <CardHeader class="flex flex-row items-center justify-between pb-2">
      <CardTitle class="text-sm font-medium">HTTP Port</CardTitle>
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
                <p>Trin must be stopped to edit HTTP port number.</p>
              </TooltipContent>
            </Tooltip>
            <DialogContent>
              <DialogHeader>
                <DialogTitle>Edit HTTP Port</DialogTitle>
              </DialogHeader>
              <Form class="py-4">
                <FormField name="port">
                  <FormItem>
                    <FormControl>
                      <NumberField
                        v-model="tempPort"
                        :min="1024"
                        :max="65535"
                        :format-options="{ useGrouping: false }"
                      >
                        <NumberFieldContent>
                          <NumberFieldDecrement />
                          <NumberFieldInput />
                          <NumberFieldIncrement />
                        </NumberFieldContent>
                      </NumberField>
                    </FormControl>
                  </FormItem>
                </FormField>
              </Form>
              <DialogFooter>
                <Button variant="outline" @click="isDialogOpen = false"> Cancel </Button>
                <Button @click="updatePort"> Save Changes </Button>
              </DialogFooter>
            </DialogContent>
          </Dialog>
        </TooltipProvider>
      </div>
    </CardHeader>
    <CardContent>
      <div class="text-2xl font-bold">{{ config.httpPort }}</div>
      <p class="text-xs text-muted-foreground">Active HTTP port for JSON-RPC server.</p>
    </CardContent>
  </Card>
</template>
