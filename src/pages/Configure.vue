<template>
  <PageLayout>
    <StorageAllocation />
    <HttpPortMonitor />
    <Autostart />
    <br />
    <p>* Configuration can only be edited when Trin is stopped.</p>
  </PageLayout>
</template>

<script setup>
import Autostart from '@/components/custom/config/Autostart.vue'
import HttpPortMonitor from '@/components/custom/config/HttpPortMonitor.vue'
import StorageAllocation from '@/components/custom/config/StorageAllocation.vue'
import PageLayout from '@/components/layouts/PageLayout.vue'
</script>
