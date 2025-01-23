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
import { formatEthBalance } from '@/components/utils/formatWei'
import { useTrinConfig } from '@/composables/useTrinConfig'
import { invoke } from '@tauri-apps/api/core'
import { toTypedSchema } from '@vee-validate/zod'
import { Loader2 } from 'lucide-vue-next'
import { useForm } from 'vee-validate'
import { computed, ref } from 'vue'
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
      }),
    blockNumber: z.number().int().positive({
      message: "Block number must be a positive integer"
    }),
  })
)
const form = useForm({
  validationSchema: formSchema
})

const accountValue = ref(null)

const onSubmit = form.handleSubmit(async (values) => {
  isLoading.value = true
  try {
    accountValue.value = await invoke('eth_getBalance', {
      trinConfig: config.value,
      address: values.address,
      blockNumber: values.blockNumber
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

const formattedBalance = computed(() => {
  if (!accountValue.value) return null
  return formatEthBalance(accountValue.value)
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
            <FormDescription> Enter an address to check the balance. </FormDescription>
            <FormMessage />
          </FormItem>
        </FormField>
        <FormField v-slot="{ field }" name="blockNumber">
          <FormItem>
            <FormLabel>Block Number</FormLabel>
            <FormControl>
              <Input 
                type="number"
                v-bind="field"
                :value="field.value"
                @input="e => field.onChange(
                  e.target.value === '' ? undefined : 
                  parseInt(e.target.value)
                )"
                min="1"
              />
            </FormControl>
            <FormDescription>
              Enter the desired block number
            </FormDescription>
            <FormMessage />
          </FormItem>
        </FormField>
        <br />
        <Button type="submit" :disabled="isLoading">
          <Loader2 v-if="isLoading" class="mr-2 h-4 w-4 animate-spin" />
          {{ isLoading ? '' : 'Submit' }}
        </Button>
        <div v-if="accountValue" class="mt-4 space-y-2">
          <p class="text-sm text-gray-500">Account Balance:</p>
          <p class="text-lg font-medium">{{ formattedBalance }}</p>
          <p class="text-xs text-gray-400">{{ parseInt(accountValue, 16) }} GWEI</p>
          <p class="text-xs text-gray-400">Raw: {{ accountValue }}</p>
        </div>
      </form>
    </CardContent>
  </Card>
</template>
