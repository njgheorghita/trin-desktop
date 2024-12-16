<script setup>
import ContentCountCard from '@/components/custom/metrics/ContentCountCard.vue'
import ContentStorageCard from '@/components/custom/metrics/ContentStorageCard.vue'
import DataRadiusCard from '@/components/custom/metrics/DataRadiusCard.vue'
import OffersReceivedCard from '@/components/custom/metrics/OffersReceivedCard.vue'
import OffersSentCard from '@/components/custom/metrics/OffersSentCard.vue'
import ValidationsCard from '@/components/custom/metrics/ValidationsCard.vue'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from '@/components/ui/tooltip'
import { formatMemorySize } from '@/components/utils/formatMemory'
import { useTrinStats } from '@/composables/useTrinStats'
import { Info, Loader2 } from 'lucide-vue-next'

const { trinStats } = useTrinStats()
</script>

<template>
  <Tabs default-value="all" class="py-4">
    <TabsList class="grid w-full grid-cols-4">
      <TabsTrigger value="all">All</TabsTrigger>
      <TabsTrigger value="history">History</TabsTrigger>
      <TabsTrigger value="state">State</TabsTrigger>
      <TabsTrigger value="beacon">Beacon</TabsTrigger>
    </TabsList>
    <TabsContent value="all">
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

        <!-- Latest Finalized Block -->
        <Card class="p-4">
          <CardHeader class="flex flex-row items-center justify-between pb-2">
            <CardTitle class="text-sm font-medium">Latest Finalized Block</CardTitle>
            <TooltipProvider>
              <Tooltip>
                <TooltipTrigger asChild>
                  <Info class="h-4 w-4 text-muted-foreground cursor-help" />
                </TooltipTrigger>
                <TooltipContent>
                  <p>Latest finalized block synced by the Trin client.</p>
                </TooltipContent>
              </Tooltip>
            </TooltipProvider>
          </CardHeader>
          <CardContent>
            <div class="text-2xl font-bold">
              <template v-if="trinStats.latestFinalizedBlock === 0">
                <div class="flex justify-center">
                  <Loader2 class="h-6 w-6 animate-spin" />
                </div>
              </template>
              <template v-else>
                {{ trinStats.latestFinalizedBlock.toLocaleString() }}
              </template>
            </div>
          </CardContent>
        </Card>

        <!-- Latest Optimistic Block -->
        <Card class="p-4">
          <CardHeader class="flex flex-row items-center justify-between pb-2">
            <CardTitle class="text-sm font-medium">Latest Optimistic Block</CardTitle>
            <TooltipProvider>
              <Tooltip>
                <TooltipTrigger asChild>
                  <Info class="h-4 w-4 text-muted-foreground cursor-help" />
                </TooltipTrigger>
                <TooltipContent>
                  <p>Latest optimistic block synced by the Trin client.</p>
                </TooltipContent>
              </Tooltip>
            </TooltipProvider>
          </CardHeader>
          <CardContent>
            <div class="text-2xl font-bold">
              <template v-if="trinStats.latestFinalizedBlock === 0">
                <div class="flex justify-center">
                  <Loader2 class="h-6 w-6 animate-spin" />
                </div>
              </template>
              <template v-else>
                {{ trinStats.latestOptimisticBlock.toLocaleString() }}
              </template>
            </div>
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
            <div class="text-2xl font-bold">{{ formatMemorySize(trinStats.diskUsage) }}</div>
          </CardContent>
        </Card>

        <ContentStorageCard
          :contentCurrent="trinStats.history.contentCurrent + trinStats.state.contentCurrent"
          :contentTotal="trinStats.history.contentTotal + trinStats.state.contentTotal"
        />
        <ContentCountCard :count="trinStats.history.count + trinStats.state.count" />
        <OffersSentCard
          :offersIn="trinStats.history.offersIn + trinStats.state.offersIn"
          :offersOut="trinStats.history.offersOut + trinStats.state.offersOut"
        />
        <OffersReceivedCard
          :acceptsIn="trinStats.history.acceptsIn + trinStats.state.acceptsIn"
          :acceptsOut="trinStats.history.acceptsOut + trinStats.state.acceptsOut"
        />
        <ValidationsCard
          :validationsIn="trinStats.history.validationsIn + trinStats.state.validationsIn"
          :validationsOut="trinStats.history.validationsOut + trinStats.state.validationsOut"
        />
      </div>
    </TabsContent>
    <TabsContent value="history">
      <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3 py-4">
        <DataRadiusCard :radius="trinStats.history.radius" />
        <ContentStorageCard
          :contentCurrent="trinStats.history.contentCurrent"
          :contentTotal="trinStats.history.contentTotal"
        />
        <ContentCountCard :count="trinStats.history.count" />
        <OffersSentCard
          :offersIn="trinStats.history.offersIn"
          :offersOut="trinStats.history.offersOut"
        />
        <OffersReceivedCard
          :acceptsIn="trinStats.history.acceptsIn"
          :acceptsOut="trinStats.history.acceptsOut"
        />
        <ValidationsCard
          :validationsIn="trinStats.history.validationsIn"
          :validationsOut="trinStats.history.validationsOut"
        />
      </div>
    </TabsContent>
    <TabsContent value="state">
      <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3 py-4">
        <DataRadiusCard :radius="trinStats.state.radius" />
        <ContentStorageCard
          :contentCurrent="trinStats.state.contentCurrent"
          :contentTotal="trinStats.state.contentTotal"
        />
        <ContentCountCard :count="trinStats.state.count" />
        <OffersSentCard
          :offersIn="trinStats.state.offersIn"
          :offersOut="trinStats.state.offersOut"
        />
        <OffersReceivedCard
          :acceptsIn="trinStats.state.acceptsIn"
          :acceptsOut="trinStats.state.acceptsOut"
        />
        <ValidationsCard
          :validationsIn="trinStats.state.validationsIn"
          :validationsOut="trinStats.state.validationsOut"
        />
      </div>
    </TabsContent>
    <TabsContent value="beacon">
      <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3 py-4">
        <DataRadiusCard :radius="trinStats.beacon.radius" />
        <ContentStorageCard
          :contentCurrent="trinStats.beacon.contentCurrent"
          :contentTotal="trinStats.beacon.contentTotal"
        />
        <ContentCountCard :count="trinStats.beacon.count" />
        <OffersSentCard
          :offersIn="trinStats.beacon.offersIn"
          :offersOut="trinStats.beacon.offersOut"
        />
        <OffersReceivedCard
          :acceptsIn="trinStats.beacon.acceptsIn"
          :acceptsOut="trinStats.beacon.acceptsOut"
        />
        <ValidationsCard
          :validationsIn="trinStats.beacon.validationsIn"
          :validationsOut="trinStats.beacon.validationsOut"
        />
      </div>
    </TabsContent>
  </Tabs>
</template>
