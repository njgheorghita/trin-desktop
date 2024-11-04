import { listen } from '@tauri-apps/api/event'
import { ref } from 'vue'

const trinStats = ref({
  cpu: 0,
  pid: 0,
  diskUsage: 0,
  state: {
    radius: 0,
    contentCurrent: 0,
    contentTotal: 0,
    count: 0,
    offersIn: 0,
    offersOut: 0,
    acceptsIn: 0,
    acceptsOut: 0,
    validationsIn: 0,
    validationsOut: 0
  },
  history: {
    radius: 0,
    contentCurrent: 0,
    contentTotal: 0,
    count: 0,
    offersIn: 0,
    offersOut: 0,
    acceptsIn: 0,
    acceptsOut: 0,
    validationsIn: 0,
    validationsOut: 0
  }
})

export function useTrinStats() {
  listen('trin-stats', (stats) => {
    Object.assign(trinStats.value, {
      cpu: stats.payload.cpu,
      pid: stats.payload.pid,
      // using state data here, though history should return the same value
      diskUsage: stats.payload.stateData.disk_usage,
      state: {
        radius: stats.payload.stateData.radius,
        contentCurrent: stats.payload.stateData.content_current,
        contentTotal: stats.payload.stateData.content_total,
        count: stats.payload.stateData.count,
        offersIn: stats.payload.stateData.offers_in,
        offersOut: stats.payload.stateData.offers_out,
        acceptsIn: stats.payload.stateData.accepts_in,
        acceptsOut: stats.payload.stateData.accepts_out,
        validationsIn: stats.payload.stateData.validations_in,
        validationsOut: stats.payload.stateData.validations_out
      },
      history: {
        radius: stats.payload.historyData.radius,
        contentCurrent: stats.payload.historyData.content_current,
        contentTotal: stats.payload.historyData.content_total,
        count: stats.payload.historyData.count,
        offersIn: stats.payload.historyData.offers_in,
        offersOut: stats.payload.historyData.offers_out,
        acceptsIn: stats.payload.historyData.accepts_in,
        acceptsOut: stats.payload.historyData.accepts_out,
        validationsIn: stats.payload.historyData.validations_in,
        validationsOut: stats.payload.historyData.validations_out
      }
    })
  })

  return {
    trinStats
  }
}
