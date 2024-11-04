<script setup>
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from '@/components/ui/tooltip'
import { formatMemoryRatio } from '@/components/utils/formatMemory'
import { Info } from 'lucide-vue-next'

defineProps({
  contentCurrent: {
    type: Number,
    required: true
  },
  contentTotal: {
    type: Number,
    required: true
  }
})
</script>

<template>
  <Card class="p-4">
    <CardHeader class="flex flex-row items-center justify-between pb-2">
      <CardTitle class="text-sm font-medium">Content Storage </CardTitle>
      <TooltipProvider>
        <Tooltip>
          <TooltipTrigger asChild>
            <Info class="h-4 w-4 text-muted-foreground cursor-help" />
          </TooltipTrigger>
          <TooltipContent>
            <p>Amount of allocated storage currently occupied.</p>
          </TooltipContent>
        </Tooltip>
      </TooltipProvider>
    </CardHeader>
    <CardContent>
      <div class="text-2xl font-bold">{{ formatMemoryRatio(contentCurrent, contentTotal) }}</div>
    </CardContent>
  </Card>
</template>
