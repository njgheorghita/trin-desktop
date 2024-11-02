<script setup>
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from '@/components/ui/tooltip'
import { useTrinStats } from '@/composables/useTrinStats'
import { Info } from 'lucide-vue-next'

const { trinStats } = useTrinStats()

// Format large numbers with commas
const formatNumber = (num) => {
  return num.toLocaleString()
}
</script>

<template>
  <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3 py-4">
    <!-- PID -->
    <Card class="p-4">
      <CardHeader class="flex flex-row items-center justify-between pb-2">
        <CardTitle class="text-sm font-medium">Process PID</CardTitle>
        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger asChild>
              <Info class="h-4 w-4 text-muted-foreground cursor-help" />
            </TooltipTrigger>
            <TooltipContent>
              <p>Process ID of the Trin client.</p>
            </TooltipContent>
          </Tooltip>
        </TooltipProvider>
      </CardHeader>
      <CardContent>
        <div class="text-2xl font-bold">{{ trinStats.pid }}</div>
      </CardContent>
    </Card>

    <!-- CPU Usage -->
    <Card class="p-4">
      <CardHeader class="flex flex-row items-center justify-between pb-2">
        <CardTitle class="text-sm font-medium">CPU Usage</CardTitle>
        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger asChild>
              <Info class="h-4 w-4 text-muted-foreground cursor-help" />
            </TooltipTrigger>
            <TooltipContent>
              <p>Current CPU usage by the Trin client process.</p>
            </TooltipContent>
          </Tooltip>
        </TooltipProvider>
      </CardHeader>
      <CardContent>
        <div class="text-2xl font-bold">{{ trinStats.cpu.toFixed(2) }}%</div>
      </CardContent>
    </Card>

    <!-- Data Radius -->
    <Card class="p-4">
      <CardHeader class="flex flex-row items-center justify-between pb-2">
        <CardTitle class="text-sm font-medium">Data Radius</CardTitle>
        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger asChild>
              <Info class="h-4 w-4 text-muted-foreground cursor-help" />
            </TooltipTrigger>
            <TooltipContent>
              <p>Current data radius of your Trin client's database.</p>
            </TooltipContent>
          </Tooltip>
        </TooltipProvider>
      </CardHeader>
      <CardContent>
        <div class="text-2xl font-bold">{{ trinStats.radius }}%</div>
      </CardContent>
    </Card>

    <!-- Content Storage -->
    <Card class="p-4">
      <CardHeader class="flex flex-row items-center justify-between pb-2">
        <CardTitle class="text-sm font-medium">Content Storage</CardTitle>
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
        <div class="text-2xl font-bold">
          {{ trinStats.contentCurrent }} / {{ trinStats.contentTotal }} MB
        </div>
      </CardContent>
    </Card>

    <!-- Content Count -->
    <Card class="p-4">
      <CardHeader class="flex flex-row items-center justify-between pb-2">
        <CardTitle class="text-sm font-medium">Content Count</CardTitle>
        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger asChild>
              <Info class="h-4 w-4 text-muted-foreground cursor-help" />
            </TooltipTrigger>
            <TooltipContent>
              <p>Number of content items stored by the Trin client.</p>
            </TooltipContent>
          </Tooltip>
        </TooltipProvider>
      </CardHeader>
      <CardContent>
        <div class="text-2xl font-bold">{{ trinStats.count }}</div>
      </CardContent>
    </Card>

    <!-- Disk Usage -->
    <Card class="p-4">
      <CardHeader class="flex flex-row items-center justify-between pb-2">
        <CardTitle class="text-sm font-medium">Disk Usage</CardTitle>
        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger asChild>
              <Info class="h-4 w-4 text-muted-foreground cursor-help" />
            </TooltipTrigger>
            <TooltipContent>
              <p>Total disk space used by the Trin client.</p>
            </TooltipContent>
          </Tooltip>
        </TooltipProvider>
      </CardHeader>
      <CardContent>
        <div class="text-2xl font-bold">{{ trinStats.diskUsage }} MB</div>
      </CardContent>
    </Card>

    <!-- Offers Sent -->
    <Card class="p-4">
      <CardHeader class="flex flex-row items-center justify-between pb-2">
        <CardTitle class="text-sm font-medium">Offers Sent</CardTitle>
        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger asChild>
              <Info class="h-4 w-4 text-muted-foreground cursor-help" />
            </TooltipTrigger>
            <TooltipContent>
              <p>How many of the OFFERs you sent to peers were ACCEPTed.</p>
            </TooltipContent>
          </Tooltip>
        </TooltipProvider>
      </CardHeader>
      <CardContent>
        <div class="text-2xl font-bold">{{ trinStats.offersIn }} / {{ trinStats.offersOut }}</div>
      </CardContent>
    </Card>

    <!-- Offers Made -->
    <Card class="p-4">
      <CardHeader class="flex flex-row items-center justify-between pb-2">
        <CardTitle class="text-sm font-medium">Offers Received</CardTitle>
        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger asChild>
              <Info class="h-4 w-4 text-muted-foreground cursor-help" />
            </TooltipTrigger>
            <TooltipContent>
              <p>How many of the OFFERs received form peers did you ACCEPT.</p>
            </TooltipContent>
          </Tooltip>
        </TooltipProvider>
      </CardHeader>
      <CardContent>
        <div class="text-2xl font-bold">{{ trinStats.acceptsIn }} / {{ trinStats.acceptsOut }}</div>
      </CardContent>
    </Card>

    <!-- Validations -->
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
        <div class="text-2xl font-bold">
          {{ trinStats.validationsIn }} / {{ trinStats.validationsOut }}
        </div>
      </CardContent>
    </Card>
  </div>
</template>
