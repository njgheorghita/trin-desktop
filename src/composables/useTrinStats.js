import { listen } from '@tauri-apps/api/event'
import { ref } from 'vue'

const trinStats = ref({
  cpu: 0,
  pid: 0,
  radius: 0,
  contentCurrent: 0,
  contentTotal: 0,
  count: 0,
  diskUsage: 0,
  offersIn: 0,
  offersOut: 0,
  acceptsIn: 0,
  acceptsOut: 0,
  validationsIn: 0,
  validationsOut: 0
})

export function useTrinStats() {
  listen('trin-stats', (stats) => {
    Object.assign(trinStats.value, {
      cpu: stats.payload.cpu,
      pid: stats.payload.pid,
      radius: stats.payload.nodeHistoryLog.radius,
      contentCurrent: stats.payload.nodeHistoryLog.content_current,
      contentTotal: stats.payload.nodeHistoryLog.content_total,
      count: stats.payload.nodeHistoryLog.count,
      diskUsage: stats.payload.nodeHistoryLog.disk_usage,
      offersIn: stats.payload.nodeHistoryLog.offers_in,
      offersOut: stats.payload.nodeHistoryLog.offers_out,
      acceptsIn: stats.payload.nodeHistoryLog.accepts_in,
      acceptsOut: stats.payload.nodeHistoryLog.accepts_out,
      validationsIn: stats.payload.nodeHistoryLog.validations_in,
      validationsOut: stats.payload.nodeHistoryLog.validations_out
    })
  })

  return {
    trinStats
  }
}
