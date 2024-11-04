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
import { Input } from '@/components/ui/input'
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
    address: z
      .string()
      .length(42)
      .regex(/^0x[a-fA-F0-9]{40}$/, {
        message: "Address must be a 42-character hexadecimal string starting with '0x'"
      })
  })
)
const form = useForm({
  validationSchema: formSchema
})
const blockHeight = ref(1000000)
const accountValue = ref(null)

const onSubmit = form.handleSubmit(async (values) => {
  isLoading.value = true
  try {
    accountValue.value = await invoke('eth_getBalance', {
      trinConfig: config.value,
      address: values.address,
      blockNumber: blockHeight.value
    })
  } catch (error) {
    toast({
      title: 'Error fetching account balance',
      description: error,
      variant: 'destructive'
    })
  } finally {
    isLoading.value = false
  }
})
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>eth_getBalance</CardTitle>
    </CardHeader>
    <CardContent>
      <form @submit.prevent="onSubmit">
        <FormField v-slot="{ field }" name="address">
          <FormItem>
            <FormLabel>Address</FormLabel>
            <FormControl>
              <Input type="text" v-bind="field" />
            </FormControl>
            <FormDescription>
              Enter an address to check the balance.
              <br />
              Balance will be checked at block number {{ blockHeight }}.
            </FormDescription>
            <FormMessage />
          </FormItem>
        </FormField>
        <Button type="submit" :disabled="isLoading">
          <Loader2 v-if="isLoading" class="mr-2 h-4 w-4 animate-spin" />
          {{ isLoading ? '' : 'Submit' }}
        </Button>
        <p v-if="accountValue">Account balance: {{ accountValue }} GWEI</p>
      </form>
    </CardContent>
  </Card>
</template>
