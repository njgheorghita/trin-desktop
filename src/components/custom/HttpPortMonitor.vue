<script setup>
import { Card, CardContent, CardTitle, CardHeader } from '@/components/ui/card'
import { Network } from 'lucide-vue-next'

defineProps({
  port: {
    type: Number,
    required: true
  }
})
</script>

<template>
  <Card class="p-4">
    <CardHeader class="flex flex-row items-center justify-between pb-2">
      <CardTitle class="text-sm font-medium">HTTP Port</CardTitle>
      <Network class="h-4 w-4 text-muted-foreground" />
    </CardHeader>
    <CardContent>
      <div class="text-2xl font-bold">{{ port }}</div>
      <p class="text-xs text-muted-foreground">Active HTTP port for JSON-RPC server</p>
    </CardContent>
  </Card>
</template>
