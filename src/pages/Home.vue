<template>
  <PageLayout>
    <ProcessControl />
    <NodeMetrics v-if="trinStatus === 'running'" />
  </PageLayout>
</template>

<script setup>
import NodeMetrics from '@/components/custom/NodeMetrics.vue'
import ProcessControl from '@/components/custom/ProcessControl.vue'
import PageLayout from '@/components/layouts/PageLayout.vue'
import { useTrinProcess } from '@/composables/useTrinProcess'
const { trinStatus } = useTrinProcess()
</script>
