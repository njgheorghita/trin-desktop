<!-- we need ts for defineProps? -->
<script setup lang="ts">
import { useForm } from 'vee-validate'
import { invoke } from '@tauri-apps/api/core'
import { toTypedSchema } from '@vee-validate/zod'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import * as z from 'zod'
import { ref } from 'vue'
import { Tabs, TabsList, TabsTrigger, TabsContent } from '@/components/ui/tabs'
import { Button } from '@/components/ui/button'
import {
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form'
import { Input } from '@/components/ui/input'

const formSchema = toTypedSchema(z.object({
  blockNumber: z.number().min(1),
}))

const form = useForm({
  validationSchema: formSchema,
})

const blockData = ref(null)
const props = defineProps({
  config: {
    type: Object,
    required: true
  }
})

const onSubmit = form.handleSubmit(async (values) => {
  console.log('Form submitted!', values)
  // Simulate fetching block data - replace this with your actual API call
  try {
    // Example data structure - replace with your actual eth_getBlockByNumber call
	const response = await invoke('eth_getBlockByNumber', {trinConfig: props.config, blockNumber: values.blockNumber})
	console.log('Response:', response)
	blockData.value = response
  } catch (error) {
    console.error('Error fetching block data:', error)
  }
})

const getPrettyBlockInfo = () => {
  if (!blockData.value) return null
  
  return {
    number: blockData.value.number,
    hash: blockData.value.hash,
    timestamp: new Date(blockData.value.timestamp * 1000).toLocaleString(),
    transactions: blockData.value.transactions?.length || 0,
    gasUsed: blockData.value.gasUsed,
    miner: blockData.value.miner,
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
			  <Input type="number" v-bind="componentField" />
			</FormControl>
			<FormDescription>
			  Enter a block number to look up.
			</FormDescription>
			<FormMessage />
		  </FormItem>
		</FormField>
		<Button type="submit">
		  Submit
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
                <div v-for="(value, key) in getPrettyBlockInfo()" :key="key" class="flex justify-between">
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
              <pre class="whitespace-pre-wrap text-sm">{{ JSON.stringify(blockData, null, 2) }}</pre>
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>
	</CardContent>
  </Card>
</template>
