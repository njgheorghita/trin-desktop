export const formatBlockRoot = (root) => {
  if (!root || root === '0x') return root
  const start = root.slice(0, 8) // 0x + first 3 bytes (8 chars)
  const end = root.slice(-6) // last 3 bytes (8 chars)
  return `${start}...${end}`
}
