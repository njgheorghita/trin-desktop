<template>
  <PageLayout>
    <StorageAllocation />
    <HttpPortMonitor />
    <Autostart />
    <br />
    <div class="text-sm flex justify-end px-4 text-muted-foreground">
      <p>* Configuration can only be edited when Trin is stopped.</p>
    </div>
  </PageLayout>
</template>

<script setup>
import Autostart from '@/components/custom/config/Autostart.vue'
import HttpPortMonitor from '@/components/custom/config/HttpPortMonitor.vue'
import StorageAllocation from '@/components/custom/config/StorageAllocation.vue'
import PageLayout from '@/components/layouts/PageLayout.vue'
</script>
