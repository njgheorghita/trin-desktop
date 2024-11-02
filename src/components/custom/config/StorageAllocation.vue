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
const tempStorage = ref(config.value.storage)

const updateStorage = () => {
  updateConfig({ storage: tempStorage.value })
  isDialogOpen.value = false
}

watch(isDialogOpen, (value) => {
  if (value) {
    // Reset temp value to current storage when dialog opens
    tempStorage.value = config.value.storage
  }
})
</script>

<template>
  <Card class="p-4">
    <CardHeader class="flex flex-row items-center justify-between pb-2">
      <CardTitle class="text-sm font-medium">Storage Allocated</CardTitle>
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
                <p>Trin must be stopped to edit storage allocation.</p>
              </TooltipContent>
            </Tooltip>

            <DialogContent>
              <DialogHeader>
                <DialogTitle>Edit Storage Allocation</DialogTitle>
              </DialogHeader>
              <Form class="py-4">
                <FormField name="storage">
                  <FormItem>
                    <FormControl>
                      <NumberField v-model="tempStorage" :min="100">
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
                <Button @click="updateStorage"> Save Changes </Button>
              </DialogFooter>
            </DialogContent>
          </Dialog>
        </TooltipProvider>
      </div>
    </CardHeader>
    <CardContent>
      <div class="text-2xl font-bold">{{ config.storage }} MB</div>
      <p class="text-xs text-muted-foreground">
        Total amount of storage that will be consumed by your Trin client.
      </p>
    </CardContent>
  </Card>
</template>
