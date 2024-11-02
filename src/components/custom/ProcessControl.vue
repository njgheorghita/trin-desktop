<script setup>
import EthereumLogo from '@/components/custom/EthereumLogo.vue'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { useTrinProcess } from '@/composables/useTrinProcess'
import { Loader2, Play, SquareX } from 'lucide-vue-next'

const { trinStatus, isLaunching, toggleTrinProcess } = useTrinProcess()
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle class="flex items-center justify-between">
        <span>Process Control</span>
        <Badge :variant="trinStatus === 'running' ? 'success' : 'secondary'">
          {{ trinStatus === 'running' ? 'Running' : 'Stopped' }}
        </Badge>
      </CardTitle>
    </CardHeader>
    <CardContent class="grid gap-4 py-6">
      <div class="flex justify-center items-center">
        <EthereumLogo :is-open="trinStatus === 'running'" />
      </div>
      <div class="flex space-x-2">
        <Button
          :variant="trinStatus === 'running' ? 'destructive' : 'default'"
          :disabled="isLaunching"
          @click="toggleTrinProcess"
          class="flex-1"
        >
          <template v-if="!isLaunching">
            <Play v-if="trinStatus !== 'running'" class="mr-2 h-4 w-4" />
            <SquareX v-else class="mr-2 h-4 w-4" />
            {{ trinStatus === 'running' ? 'Shutdown Trin' : 'Launch Trin' }}
          </template>
          <Loader2 v-else class="mr-2 h-4 w-4 animate-spin" />
        </Button>
      </div>
    </CardContent>
  </Card>
</template>
