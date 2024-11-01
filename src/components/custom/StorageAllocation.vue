<script setup>
import { Card, CardContent, CardTitle, CardHeader } from '@/components/ui/card'
import { HardDrive } from 'lucide-vue-next'

defineProps({
  storage: {
    type: Number,
    required: true
  }
})
</script>

<template>
  <Card class="p-4">
    <CardHeader class="flex flex-row items-center justify-between pb-2">
      <CardTitle class="text-sm font-medium">Storage Allocated</CardTitle>
      <HardDrive class="h-4 w-4 text-muted-foreground" />
    </CardHeader>
    <CardContent>
      <div class="text-2xl font-bold">{{ storage }} MB</div>
      <p class="text-xs text-muted-foreground">
        Total amount of storage that will be consumed by your Trin client.
      </p>
    </CardContent>
  </Card>
</template>
