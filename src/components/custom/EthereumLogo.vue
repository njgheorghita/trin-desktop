<template>
  <div class="ethereum-logo" :style="containerStyle">
    <svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
      <!-- Lines -->
      <g class="lines-group">
        <path
          v-for="i in 6"
          :key="`line${i}`"
          :id="`line${i}`"
          :d="getLineData(i).d.value"
          :stroke="currentLineColor"
          stroke-width="1"
        >
          <!-- Position animation -->
          <animate
            :id="`line${i}Anim`"
            attributeName="d"
            :values="getLineAnimationValues(i)"
            dur="0.5s"
            fill="freeze"
            begin="indefinite"
          />
          <!-- Color animation -->
          <animate
            :id="`line${i}Color`"
            attributeName="stroke"
            :values="colorValues"
            dur="0.5s"
            fill="freeze"
            begin="indefinite"
          />
          <!-- Simple shimmer effect -->
          <animate
            attributeName="stroke-opacity"
            values="1;0.4;0.8;0.3;0.9;0.5;1"
            :dur="`${3 + i * 0.4}s`"
            repeatCount="indefinite"
            begin="0s"
          />
        </path>
      </g>

      <!-- Top Diamond -->
      <g>
        <path id="topDiamond" :d="topDiamondData.d" :fill="diamondFill">
          <animate
            id="topDiamondAnim"
            attributeName="d"
            :values="topDiamondData.values"
            dur="0.5s"
            fill="freeze"
            begin="indefinite"
          />
        </path>
        <path :d="topDiamondData.line1" fill="none" :stroke="diamondLines" stroke-width="1">
          <animate
            id="topLine1Anim"
            attributeName="d"
            :values="topDiamondData.line1Values"
            dur="0.5s"
            fill="freeze"
            begin="indefinite"
          />
        </path>
        <path :d="topDiamondData.line2" fill="none" :stroke="diamondLines" stroke-width="1">
          <animate
            id="topLine2Anim"
            attributeName="d"
            :values="topDiamondData.line2Values"
            dur="0.5s"
            fill="freeze"
            begin="indefinite"
          />
        </path>
      </g>

      <!-- Bottom Diamond -->
      <g>
        <path id="bottomDiamond" :d="bottomDiamondData.d" :fill="diamondFill">
          <animate
            id="bottomDiamondAnim"
            attributeName="d"
            :values="bottomDiamondData.values"
            dur="0.5s"
            fill="freeze"
            begin="indefinite"
          />
        </path>
        <path :d="bottomDiamondData.line" fill="none" :stroke="diamondLines" stroke-width="1">
          <animate
            id="bottomLineAnim"
            attributeName="d"
            :values="bottomDiamondData.lineValues"
            dur="0.5s"
            fill="freeze"
            begin="indefinite"
          />
        </path>
      </g>
    </svg>
  </div>
</template>

<script>
import { useTheme } from '@/composables/useTheme'
import { computed, onMounted, ref, watch } from 'vue'

export default {
  name: 'EthereumLogo',

  props: {
    isOpen: {
      type: Boolean,
      default: false
    },
    size: {
      type: Number,
      default: 400
    }
  },

  setup(props) {
    const { isDark } = useTheme()
    const initialized = ref(false)
    const currentState = ref(props.isOpen)

    const colorScheme = computed(() => ({
      light: {
        closed: '#FF0000',
        transit: '#FFFF00',
        open: '#00FF00',
        diamond: '#000000',
        lines: '#FFFFFF'
      },
      dark: {
        closed: '#FF6B6B',
        transit: '#FFE66D',
        open: '#6BCB77',
        diamond: '#FFFFFF',
        lines: '#000000'
      }
    }))

    const currentScheme = computed(() =>
      isDark.value ? colorScheme.value.dark : colorScheme.value.light
    )

    const currentLineColor = computed(() =>
      currentState.value ? currentScheme.value.open : currentScheme.value.closed
    )

    const colorValues = computed(() =>
      currentState.value
        ? `${currentScheme.value.closed};${currentScheme.value.transit};${currentScheme.value.open}`
        : `${currentScheme.value.open};${currentScheme.value.transit};${currentScheme.value.closed}`
    )

    const diamondFill = computed(() => currentScheme.value.diamond)
    const diamondLines = computed(() => currentScheme.value.lines)

    const containerStyle = computed(() => ({
      width: `${props.size}px`,
      height: `${props.size}px`,
      cursor: 'pointer'
    }))

    // Define initial positions based on isOpen prop
    const getInitialLinePosition = (index, isOpen) => {
      const positions = {
        1: isOpen ? 'M31,45.1 L31,71' : 'M31,55.1 L31,61',
        2: isOpen ? 'M41,45 L41,77' : 'M41,55 L41,67',
        3: isOpen ? 'M51,45 L51,83' : 'M51,55 L51,73',
        4: isOpen ? 'M59,45 L59,83' : 'M59,55 L59,73',
        5: isOpen ? 'M69,45 L69,77' : 'M69,55 L69,67',
        6: isOpen ? 'M79,45.1 L79,71' : 'M79,55.1 L79,61'
      }
      return positions[index]
    }

    const lineData = {
      1: {
        d: computed(() => getInitialLinePosition(1, currentState.value)),
        opening: 'M31,55.1 L31,61;M31,45.1 L31,71',
        closing: 'M31,45.1 L31,71;M31,55.1 L31,61'
      },
      2: {
        d: computed(() => getInitialLinePosition(2, currentState.value)),
        opening: 'M41,55 L41,67;M41,45 L41,77',
        closing: 'M41,45 L41,77;M41,55 L41,67'
      },
      3: {
        d: computed(() => getInitialLinePosition(3, currentState.value)),
        opening: 'M51,55 L51,73;M51,45 L51,83',
        closing: 'M51,45 L51,83;M51,55 L51,73'
      },
      4: {
        d: computed(() => getInitialLinePosition(4, currentState.value)),
        opening: 'M59,55 L59,73;M59,45 L59,83',
        closing: 'M59,45 L59,83;M59,55 L59,73'
      },
      5: {
        d: computed(() => getInitialLinePosition(5, currentState.value)),
        opening: 'M69,55 L69,67;M69,45 L69,77',
        closing: 'M69,45 L69,77;M69,55 L69,67'
      },
      6: {
        d: computed(() => getInitialLinePosition(6, currentState.value)),
        opening: 'M79,55.1 L79,61;M79,45.1 L79,71',
        closing: 'M79,45.1 L79,71;M79,55.1 L79,61'
      }
    }

    const topDiamondData = computed(() => {
      const isOpen = currentState.value
      return {
        d: isOpen ? 'M55,5 L80,45 L55,60 L30,45 Z' : 'M55,15 L80,55 L55,70 L30,55 Z',
        values: isOpen
          ? 'M55,15 L80,55 L55,70 L30,55 Z;M55,5 L80,45 L55,60 L30,45 Z'
          : 'M55,5 L80,45 L55,60 L30,45 Z;M55,15 L80,55 L55,70 L30,55 Z',
        line1: isOpen ? 'M55,5 L55,60' : 'M55,15 L55,70',
        line1Values: isOpen ? 'M55,15 L55,70;M55,5 L55,60' : 'M55,5 L55,60;M55,15 L55,70',
        line2: isOpen ? 'M30,45 L55,34 L80,45' : 'M30,55 L55,44 L80,55',
        line2Values: isOpen
          ? 'M30,55 L55,44 L80,55;M30,45 L55,34 L80,45'
          : 'M30,45 L55,34 L80,45;M30,55 L55,44 L80,55'
      }
    })

    const bottomDiamondData = computed(() => {
      const isOpen = currentState.value
      return {
        d: isOpen ? 'M55,85 L80,70 L55,100 L30,70 Z' : 'M55,75 L80,60 L55,90 L30,60 Z',
        values: isOpen
          ? 'M55,75 L80,60 L55,90 L30,60 Z;M55,85 L80,70 L55,100 L30,70 Z'
          : 'M55,85 L80,70 L55,100 L30,70 Z;M55,75 L80,60 L55,90 L30,60 Z',
        line: isOpen ? 'M55,83 L55,102' : 'M55,73 L55,92',
        lineValues: isOpen ? 'M55,73 L55,92;M55,83 L55,102' : 'M55,83 L55,102;M55,73 L55,92'
      }
    })

    const getLineData = (index) => lineData[index]

    const getLineAnimationValues = (index) => {
      return currentState.value ? lineData[index].opening : lineData[index].closing
    }

    const triggerAnimations = () => {
      const animations = [
        'topDiamondAnim',
        'bottomDiamondAnim',
        'topLine1Anim',
        'topLine2Anim',
        'bottomLineAnim',
        ...Array.from({ length: 6 }, (_, i) => `line${i + 1}Anim`),
        ...Array.from({ length: 6 }, (_, i) => `line${i + 1}Color`)
      ]

      if (initialized.value) {
        animations.forEach((id) => {
          const elem = document.getElementById(id)
          if (elem) elem.beginElement()
        })
      }
    }

    // Handle prop changes
    watch(
      () => props.isOpen,
      (newVal) => {
        if (initialized.value && currentState.value !== newVal) {
          currentState.value = newVal
          triggerAnimations()
        }
      },
      { immediate: true }
    )

    // Initialize on mount
    onMounted(() => {
      currentState.value = props.isOpen
      initialized.value = true
    })

    return {
      currentLineColor,
      colorValues,
      diamondFill,
      diamondLines,
      containerStyle,
      getLineData,
      getLineAnimationValues,
      topDiamondData,
      bottomDiamondData,
      triggerAnimations
    }
  }
}
</script>

<style scoped>
.ethereum-logo {
  display: flex;
  align-items: center;
  justify-content: center;
}

svg {
  width: 100%;
  height: 100%;
}
</style>
