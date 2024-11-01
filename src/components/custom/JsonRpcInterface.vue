<script setup>
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useForm } from 'vee-validate'
import { toFormValidator } from '@vee-validate/zod'
import * as z from 'zod'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form'
import { Loader2 } from 'lucide-vue-next'
import { Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselPrevious } from '@/components/ui/carousel'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'

const hashSchema = toFormValidator(
 z.object({
   blockHash: z.string().min(1, 'Block hash is required')
 })
)

const numberSchema = toFormValidator(
 z.object({ 
   blockNumber: z.string().min(1, 'Block number is required')
 })
)

const hashForm = useForm({
 initialValues: {
   blockHash: ''
 }, 
 validationSchema: hashSchema,
})

const numberForm = useForm({
 initialValues: {
   blockNumber: 0
 },
 validationSchema: numberSchema,
   onSubmit: async (values) => {
    console.log('Form submitted with values:', values);
    await fetchBlockByNumber(values);
  }
})

const isLoadingHash = ref(false)
const isLoadingNumber = ref(false)
const blockByHashResult = ref(null)
const blockByNumberResult = ref(null)
const displayMode = ref('pretty')

async function handleNumberSubmit(e) {
  console.log('Submit clicked');
  e?.preventDefault();
  try {
    const values = await numberForm.validate();
    console.log('Validation passed:', values);
    await fetchBlockByNumber(values);
  } catch (error) {
    console.error('Error:', error);
  }
}

async function fetchBlockByHash (values) {
 isLoadingHash.value = true
 try {
   // Implement RPC call
   console.log('Fetching block with hash:', values.blockHash)
   //blockByHashResult.value = await invoke('eth_getBlockByHash', { blockHash: values.blockHash })
   blockByHashResult.value = "0xasdfasdf"
 } finally {
   isLoadingHash.value = false
 }
}

async function fetchBlockByNumber (values) {
 isLoadingNumber.value = true
 try {
   // Implement RPC call
   console.log('Fetching block number:', values.blockNumber)
   //blockByNumberResult.value = await invoke('eth_getBlockByNumber', { blockNumber: values.blockNumber })
   blockByNumberResult.value = 123
 } finally {
   isLoadingNumber.value = false
 }
}
</script>

<template>
<Card class="w-full">
   <CardHeader>
     <CardTitle>JSON-RPC Interface</CardTitle>
     <CardDescription>Interact with Ethereum JSON-RPC methods</CardDescription>
   </CardHeader>
   <CardContent>
     <Carousel class="w-full">
       <CarouselContent>
         <CarouselItem>
           <Card>
             <CardHeader>
               <CardTitle>eth_getBlockByHash</CardTitle>
               <CardDescription>Retrieve a block by its hash</CardDescription>
             </CardHeader>
             <CardContent>
			 <Form :form="hashForm" @submit="hashForm.handleSubmit(fetchBlockByHash)">
                 <FormField
                   name="blockHash"
                   v-slot="{ field }"
				   :control="hashForm.control"
                 >
                   <FormItem>
                     <FormLabel>Block Hash</FormLabel>
                     <FormControl>
                       <Input v-bind="field" placeholder="0x..." />
                     </FormControl>
                     <FormMessage />
                   </FormItem>
                 </FormField>
                 <Button 
                   type="submit"
                   :disabled="isLoadingHash"
                   class="mt-4"
                 >
                   <Loader2 v-if="isLoadingHash" class="mr-2 h-4 w-4 animate-spin" />
                   {{ isLoadingHash ? 'Fetching...' : 'Fetch Block' }}
                 </Button>
               </Form>

               <div v-if="blockByHashResult" class="mt-4">
                 <Tabs v-model="displayMode">
                   <TabsList>
                     <TabsTrigger value="pretty">Pretty</TabsTrigger>
                     <TabsTrigger value="raw">Raw JSON</TabsTrigger>
                   </TabsList>
                   <TabsContent value="pretty">
                     <pre>Pretty formatted block details</pre>
                   </TabsContent>
                   <TabsContent value="raw">
                     <pre>{{ JSON.stringify(blockByHashResult, null, 2) }}</pre>
                   </TabsContent>
                 </Tabs>
               </div>
             </CardContent>
           </Card>
         </CarouselItem>

         <CarouselItem>
           <Card>
             <CardHeader>
               <CardTitle>eth_getBlockByNumber</CardTitle>
               <CardDescription>Retrieve a block by its number</CardDescription>
             </CardHeader>
             <CardContent>
			 <Form :form="numberForm" @submit.prevent>
                 <FormField
                   name="blockNumber"
                   v-slot="{ field }"
				   :control="numberForm.control"
                 >
                   <FormItem>
                     <FormLabel>Block Number</FormLabel>
                     <FormControl>
                       <Input v-bind="field" placeholder="Enter block number or latest" />
                     </FormControl>
                     <FormMessage />
                   </FormItem>
                 </FormField>
                 <Button 
                   type="submit"
                   :disabled="isLoadingNumber"
                   class="mt-4"
                 >
                   <Loader2 v-if="isLoadingNumber" class="mr-2 h-4 w-4 animate-spin" />
                   {{ isLoadingNumber ? 'Fetching...' : 'Fetch Block' }}
                 </Button>
               </Form>

               <div v-if="blockByNumberResult" class="mt-4">
                 <Tabs v-model="displayMode">
                   <TabsList>
                     <TabsTrigger value="pretty">Pretty</TabsTrigger>
                     <TabsTrigger value="raw">Raw JSON</TabsTrigger>
                   </TabsList>
                   <TabsContent value="pretty">
                     <pre>Pretty formatted block details</pre>
                   </TabsContent>
                   <TabsContent value="raw">
                     <pre>{{ JSON.stringify(blockByNumberResult, null, 2) }}</pre>
                   </TabsContent>
                 </Tabs>
               </div>
             </CardContent>
           </Card>
         </CarouselItem>
       </CarouselContent>
       <CarouselPrevious />
       <CarouselNext />
     </Carousel>
   </CardContent>
 </Card>
</template>
