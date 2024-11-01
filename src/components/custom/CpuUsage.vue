<script setup>
import { Card, CardContent, CardTitle, CardHeader } from '@/components/ui/card'
import { Progress } from '@/components/ui/progress'

defineProps({
  trinStats: {
    type: Object,
    required: true
  }
})
</script>

<template>
  <Card class="p-4">
    <CardHeader class="flex flex-row items-center justify-between pb-2">
      <CardTitle class="text-sm font-medium">CPU Usage</CardTitle>
	  <!--Progress :value="cpuUsage" /-->
      <p class="text-sm font-medium text-gray-600">{{ trinStats.cpu.toFixed(2) }}%</p>
    </CardHeader>
    <CardContent>
	  <p class="py-1 text-xs text-muted-foreground">
		Subnetworks: History
	  </p>
	  <p class="py-1 text-xs text-muted-foreground">
		radius: {{ trinStats.radius }}
	  </p>
	  <p class="py-1 text-xs text-muted-foreground">
	    content current: {{ trinStats.contentCurrent }}
	  </p>
	  <p class="py-1 text-xs text-muted-foreground">
	    content total: {{ trinStats.contentTotal }}
      </p>
	  <p class="py-1 text-xs text-muted-foreground">
	    count: {{ trinStats.count }}
	  </p>
	  <p class="py-1 text-xs text-muted-foreground">
	    disk usage: {{ trinStats.diskUsage }}
	  </p>
	  <p class="py-1 text-xs text-muted-foreground">
	    offers in: {{ trinStats.offersIn }}
	  </p>
	  <p class="py-1 text-xs text-muted-foreground">
	    offers out: {{ trinStats.offersOut }}
	  </p>
	  <p class="py-1 text-xs text-muted-foreground">
	    accepts in: {{ trinStats.acceptsIn }}
	  </p>
	  <p class="py-1 text-xs text-muted-foreground">
	    accepts out: {{ trinStats.acceptsOut }}
	  </p>
	  <p class="py-1 text-xs text-muted-foreground">
	    validations in: {{ trinStats.validationsIn }}
	  </p>
	  <p class="py-1 text-xs text-muted-foreground">
	    validations out: {{ trinStats.validationsOut }}
	  </p>
    </CardContent>
  </Card>
</template>
