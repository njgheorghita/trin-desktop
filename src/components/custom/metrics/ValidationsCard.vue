<script setup>
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from '@/components/ui/tooltip'
import { Info } from 'lucide-vue-next'

defineProps({
  validationsIn: {
    type: Number,
    required: true
  },
  validationsOut: {
    type: Number,
    required: true
  }
})
</script>

<template>
  <Card class="p-4">
    <CardHeader class="flex flex-row items-center justify-between pb-2">
      <CardTitle class="text-sm font-medium">Validations</CardTitle>
      <TooltipProvider>
        <Tooltip>
          <TooltipTrigger asChild>
            <Info class="h-4 w-4 text-muted-foreground cursor-help" />
          </TooltipTrigger>
          <TooltipContent>
            <p>How many of the ACCEPTs you received were you able to validate.</p>
          </TooltipContent>
        </Tooltip>
      </TooltipProvider>
    </CardHeader>
    <CardContent>
      <div class="text-2xl font-bold">{{ validationsIn }} / {{ validationsOut }}</div>
    </CardContent>
  </Card>
</template>
