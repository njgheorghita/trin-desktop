<!-- we need ts for defineProps? -->
<script setup lang="ts">
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import {
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage
} from '@/components/ui/form'
import {
  NumberField,
  NumberFieldContent,
  NumberFieldDecrement,
  NumberFieldIncrement,
  NumberFieldInput
} from '@/components/ui/number-field'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { useToast } from '@/components/ui/toast'
import { useTrinConfig } from '@/composables/useTrinConfig'
import { invoke } from '@tauri-apps/api/core'
import { toTypedSchema } from '@vee-validate/zod'
import { Loader2 } from 'lucide-vue-next'
import { useForm } from 'vee-validate'
import { ref } from 'vue'
import * as z from 'zod'

const { config } = useTrinConfig()
const { toast } = useToast()
const isLoading = ref(false)

const formSchema = toTypedSchema(
  z.object({
    blockNumber: z.number().min(0)
  })
)
const form = useForm({
  validationSchema: formSchema
})
const blockData = ref(null)

const fetchBlockData = async (blockNumber) => {
  isLoading.value = true
  try {
    blockData.value = await invoke('eth_getBlockByNumber', {
      trinConfig: config.value,
      blockNumber
    })
  } catch (error) {
    toast({
      title: 'Error fetching block data',
      description: error,
      variant: 'destructive'
    })
  } finally {
    isLoading.value = false
  }
}

const onSubmit = form.handleSubmit(async (values) => {
  await fetchBlockData(values.blockNumber)
})

const onNumberChange = async (value) => {
  await fetchBlockData(value)
}

const getPrettyBlockInfo = () => {
  if (!blockData.value) return null

  return {
    number: blockData.value.number,
    hash: blockData.value.hash,
    timestamp: new Date(blockData.value.timestamp * 1000).toLocaleString(),
    transactions: blockData.value.transactions?.length || 0,
    gasUsed: blockData.value.gasUsed,
    miner: blockData.value.miner
  }
}
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>eth_getBlockByNumber</CardTitle>
    </CardHeader>
    <CardContent>
      <form @submit="onSubmit">
        <FormField v-slot="{ componentField }" name="blockNumber">
          <FormItem>
            <FormLabel>Block Number</FormLabel>
            <FormControl>
              <NumberField v-bind="componentField" @update:modelValue="onNumberChange" :min="0">
                <NumberFieldContent>
                  <NumberFieldDecrement />
                  <NumberFieldInput />
                  <NumberFieldIncrement />
                </NumberFieldContent>
              </NumberField>
            </FormControl>
            <FormDescription> Enter a block number to look up. </FormDescription>
            <FormMessage />
          </FormItem>
        </FormField>
        <Button type="submit" :disabled="isLoading">
          <Loader2 v-if="isLoading" class="mr-2 h-4 w-4 animate-spin" />
          {{ isLoading ? '' : 'Submit' }}
        </Button>
      </form>
      <Tabs v-if="blockData" default-value="pretty" class="mt-6">
        <TabsList class="grid w-full grid-cols-2">
          <TabsTrigger value="pretty">Pretty View</TabsTrigger>
          <TabsTrigger value="raw">Raw JSON</TabsTrigger>
        </TabsList>
        <TabsContent value="pretty">
          <Card>
            <CardContent class="pt-6">
              <div v-if="getPrettyBlockInfo()" class="space-y-2">
                <div
                  v-for="(value, key) in getPrettyBlockInfo()"
                  :key="key"
                  class="flex justify-between"
                >
                  <span class="font-medium">{{ key }}:</span>
                  <span class="text-gray-600">{{ value }}</span>
                </div>
              </div>
            </CardContent>
          </Card>
        </TabsContent>
        <TabsContent value="raw">
          <Card>
            <CardContent class="pt-6">
              <div class="overflow-x-auto max-w-[650px]">
                <pre class="whitespace-pre text-sm">{{ JSON.stringify(blockData, null, 2) }}</pre>
              </div>
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>
    </CardContent>
  </Card>
</template>
