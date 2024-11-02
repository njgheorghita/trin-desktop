<template>
  <PageLayout>
    <TooltipProvider>
      <Tooltip>
        <TooltipTrigger asChild>
          <div class="w-full">
            <!-- Wrapper for tooltip -->
            <Card :class="{ 'opacity-50': trinStatus !== 'running' }">
              <CardHeader>
                <div class="flex items-center justify-between">
                  <div>
                    <CardTitle>JSON-RPC Interface</CardTitle>
                    <CardDescription
                      >Execute the following requests by looking up data from the Portal
                      Network.</CardDescription
                    >
                  </div>
                  <Info
                    v-if="trinStatus !== 'running'"
                    class="h-4 w-4 text-muted-foreground cursor-help"
                  />
                </div>
              </CardHeader>
              <CardContent :class="{ 'pointer-events-none': trinStatus !== 'running' }">
                <div class="space-y-4">
                  <Select v-model="selectedMethod">
                    <SelectTrigger class="w-[280px]">
                      <SelectValue placeholder="Select a method" />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem
                        v-for="method in methods"
                        :key="method.value"
                        :value="method.value"
                      >
                        {{ method.label }}
                      </SelectItem>
                    </SelectContent>
                  </Select>
                  <!-- Component container -->
                  <div class="mt-4">
                    <GetBlockByHash v-if="selectedMethod === 'hash'" />
                    <GetBlockByNumber v-if="selectedMethod === 'number'" />
                    <GetBalance v-if="selectedMethod === 'balance'" />
                  </div>
                </div>
              </CardContent>
            </Card>
          </div>
        </TooltipTrigger>
        <TooltipContent v-if="trinStatus !== 'running'">
          <p>Trin must be running to use the JSON-RPC interface</p>
        </TooltipContent>
      </Tooltip>
    </TooltipProvider>
  </PageLayout>
</template>

<script setup>
import GetBalance from '@/components/custom/jsonrpc/GetBalance.vue'
import GetBlockByHash from '@/components/custom/jsonrpc/GetBlockByHash.vue'
import GetBlockByNumber from '@/components/custom/jsonrpc/GetBlockByNumber.vue'
import PageLayout from '@/components/layouts/PageLayout.vue'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue
} from '@/components/ui/select'
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from '@/components/ui/tooltip'
import { useTrinProcess } from '@/composables/useTrinProcess'
import { Info } from 'lucide-vue-next'
import { ref } from 'vue'

const { trinStatus } = useTrinProcess()
const selectedMethod = ref('hash') // Default to 'hash'

const methods = [
  { value: 'hash', label: 'eth_getBlockByHash' },
  { value: 'number', label: 'eth_getBlockByNumber' },
  { value: 'balance', label: 'eth_getBalance' }
]
</script>
