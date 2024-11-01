<script setup>
import { Badge } from '@/components/ui/badge'
import { Card, CardContent, CardTitle, CardHeader } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Play, SquareX, Loader2 } from 'lucide-vue-next'
import EthereumLogo from '@/components/custom/EthereumLogo.vue'

const props = defineProps({
  trinStatus: {
    type: String,
    required: true
  },
  isLaunching: {
    type: Boolean,
    required: true
  }
})

const emit = defineEmits(['toggleTrinProcess'])
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
      <EthereumLogo :is-open="trinStatus === 'running'" />
      <div class="flex space-x-2">
        <Button
          :variant="trinStatus === 'running' ? 'destructive' : 'default'"
          :disabled="isLaunching"
          @click="emit('toggleTrinProcess')"
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
