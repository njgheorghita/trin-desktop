<script setup lang="ts">
import { Button } from '@/components/ui/button'
import { CardContent, Card, CardHeader, CardTitle } from '@/components/ui/card'
import { FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form'
import { Input } from '@/components/ui/input'
import { useToast } from '@/components/ui/toast'
import { formatEthBalance } from '@/components/utils/formatWei'
import { useTrinConfig } from '@/composables/useTrinConfig'
import { invoke } from '@tauri-apps/api/core'
import { toTypedSchema } from '@vee-validate/zod'
import { Loader2,Copy } from 'lucide-vue-next'
import { useForm } from 'vee-validate'
import { computed, ref } from 'vue'
import * as z from 'zod'
import { ref as vueRef } from 'vue'


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
const showBytecode = vueRef(false)
const showFullBytecode = vueRef(false)
const accountData = ref({
  balance: null,
  nonce: null,
  code: null
})

const toggleBytecode = () => {
  showBytecode.value = !showBytecode.value
}

const toggleFullBytecode = () => {
  showFullBytecode.value = !showFullBytecode.value
}

const copyBytecodeToClipboard = async () => {
  try {
    await navigator.clipboard.writeText(accountData.value.code)
    toast({
      title: 'Bytecode copied to clipboard',
      variant: 'default'
    })
  } catch (error) {
    toast({
      title: 'Failed to copy bytecode',
      description: error.toString(),
      variant: 'destructive'
    })
  }
}

const onSubmit = form.handleSubmit(async (values) => {
  isLoading.value = true
  try {
    const [balance, nonce, code] = await Promise.all([
      invoke('eth_getBalance', {
        trinConfig: config.value,
        address: values.address
      }),
      invoke('eth_getTransactionCount', {
        trinConfig: config.value,
        address: values.address
      }),
      invoke('eth_getCode', {
        trinConfig: config.value,
        address: values.address
      })
    ])

    accountData.value = {
      balance,
      nonce,
      code
    }
  } catch (error) {
    toast({
      title: 'Error fetching account data',
      description: error,
      variant: 'destructive'
    })
  } finally {
    isLoading.value = false
  }
})

const isContract = computed(() => {
  return accountData.value.code && accountData.value.code !== '0x'
})

const formattedBalance = computed(() => {
  if (!accountData.value.balance) return null
  return formatEthBalance(accountData.value.balance)
})
</script>

<template>
  <Card> 
    <CardHeader>
      <CardTitle>Account Information</CardTitle>
    </CardHeader>
    <CardContent class="grid gap-4 py-6"> 
      <form @submit.prevent="onSubmit">

        <FormField v-slot="{ field }" name="address">
          <FormItem class="max-w-2xl"> 
            <FormLabel>Address</FormLabel>
            <FormControl>
              <Input type="text" v-bind="field" />
            </FormControl>
            <FormMessage />
          </FormItem>
        </FormField>
        <br />
        <Button type="submit" :disabled="isLoading">
          <Loader2 v-if="isLoading" class="mr-2 h-4 w-4 animate-spin" />
          {{ isLoading ? '' : 'Submit' }}
        </Button>

        <div v-if="accountData.balance" class="mt-8 space-y-6"> 
          <div class="grid grid-cols-2 gap-8"> 
            <div class="space-y-2">
              <p class="text-sm text-gray-500">ETH Balance:</p>
              <p class="text-lg font-medium">{{ formattedBalance }}</p>
            </div>
            <div class="space-y-2">
              <p class="text-sm text-gray-500">Nonce:</p>
              <p class="text-lg font-medium">{{ parseInt(accountData.nonce, 16) }}</p>
            </div>
            <div class="space-y-2">
              <p class="text-sm text-gray-500">Account Type:</p>
              <p class="text-lg font-medium">{{ isContract ? 'Smart Contract' : 'Externally Owned Account (EOA)' }}</p>
            </div>
          </div>

          
          <div v-if="isContract" class="space-y-4 mt-8"> 
            <div class="flex justify-between items-center">
              <p class="text-sm text-gray-500">Contract Code:</p>
              <div class="space-x-2">
                <Button 
                  variant="outline" 
                  size="sm"
                  @click="toggleBytecode"
                >
                  {{ showBytecode ? 'Hide' : 'Show' }} Deployed Bytecode
                </Button>
                <Button
                  v-if="showBytecode"
                  variant="outline"
                  size="sm"
                  @click="copyBytecodeToClipboard"
                >
                  <Copy class="h-4 w-4 mr-2" />
                  Copy
                </Button>
              </div>
            </div>
            
            <div v-if="showBytecode">
              <div 
                class="relative mt-2 bg-gray-100 rounded-md"
                :class="{ 'max-h-[300px]': !showFullBytecode }" 
              >
                <pre 
                  class="p-6 overflow-auto" 
                  :class="{ 'max-h-[300px]': !showFullBytecode }"
                >{{ accountData.code }}</pre>
                
                <div 
                  v-if="!showFullBytecode" 
                  class="absolute bottom-0 left-0 right-0 h-16 bg-gradient-to-t from-gray-100 to-transparent"
                ></div>
              </div>
              
              <Button 
                v-if="accountData.code.length > 100"
                variant="ghost" 
                size="sm" 
                class="mt-4" 
                @click="toggleFullBytecode"
              >
                {{ showFullBytecode ? 'Show Less' : 'Show More' }}
              </Button>
            </div>
          </div>
        </div>
      </form>
    </CardContent>
  </Card>
</template>