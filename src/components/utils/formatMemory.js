export const formatMemorySize = (mbSize) => {
  if (mbSize >= 1000) {
    const gbSize = mbSize / 1000
    // Convert to string and remove trailing zeros after decimal
    const formatted = gbSize.toFixed(2).replace(/\.?0+$/, '')
    return `${formatted} GB`
  }
  const formatted = mbSize.toFixed(2).replace(/\.?0+$/, '')
  return `${formatted} MB`
}

export const formatMemoryRatio = (used, total) => {
  // Convert both to GB if total is >= 1000 MB
  if (total >= 1000) {
    const usedGB = used / 1000
    const totalGB = total / 1000
    const usedFormatted = usedGB.toFixed(2).replace(/\.?0+$/, '')
    const totalFormatted = totalGB.toFixed(2).replace(/\.?0+$/, '')
    return `${usedFormatted} / ${totalFormatted} GB`
  }

  // Keep in MB if total is < 1000 MB
  const usedFormatted = used.toFixed(2).replace(/\.?0+$/, '')
  const totalFormatted = total.toFixed(2).replace(/\.?0+$/, '')
  return `${usedFormatted} / ${totalFormatted} MB`
}
