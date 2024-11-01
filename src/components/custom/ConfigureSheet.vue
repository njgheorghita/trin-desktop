<script setup>
import { ref } from 'vue'
import { Button } from '@/components/ui/button'
import { useToast } from '@/components/ui/toast'
import {
  Sheet,
  SheetContent,
  SheetClose,
  SheetFooter,
  SheetDescription,
  SheetHeader,
  SheetTitle,
  SheetTrigger
} from '@/components/ui/sheet'
import {
  NumberField,
  NumberFieldContent,
  NumberFieldDecrement,
  NumberFieldIncrement,
  NumberFieldInput
} from '@/components/ui/number-field'
import { Switch } from '@/components/ui/switch'
import { useForm } from 'vee-validate'
import { toFormValidator } from '@vee-validate/zod'
import * as z from 'zod'
import {
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
  Form
} from '@/components/ui/form'

const props = defineProps({
  config: {
    type: Object,
    required: true
  },
  trinStatus: {
    type: String,
    required: true
  }
})

const emit = defineEmits(['update-config'])
const form = ref(null)

const configSchema = z.object({
  storage: z.number().min(100),
  httpPort: z.number().min(0).max(65535),
  autostart: z.boolean()
})

const initForm = () => {
  form.value = useForm({
    validationSchema: toFormValidator(configSchema),
    defaultValues: {
      storage: props.config.storage,
      httpPort: props.config.httpPort,
      autostart: props.config.autostart
    }
  })
}

initForm()
</script>

<template>
  <Sheet>
    <SheetTrigger as-child class="absolute z-50">
      <Button variant="outline" id="configureButton" :disabled="trinStatus === 'running'">
        Configure
      </Button>
    </SheetTrigger>
    <SheetContent>
      <SheetHeader>
        <SheetTitle>Configure Trin</SheetTitle>
        <SheetDescription>
          Update the settings for your Trin node here. Click save when you're done.
        </SheetDescription>
      </SheetHeader>
      <div class="grid gap-4 py-4">
        <div class="grid items-center gap-4">
          <Form :form="form" @submit="emit('update-config', $event)">
            <FormField v-slot="{ field }" name="storage" :control="form.control">
              <FormItem>
                <FormLabel>Storage (MB)</FormLabel>
                <FormControl>
                  <NumberField
                    class="gap-2"
                    :min="100"
                    v-bind="field"
                    :model-value="config.storage"
                    @update:model-value="
                      (v) => {
                        field.onChange(v)
                        if (v) {
                          config.storage = v
                        } else {
                          config.storage = 100
                        }
                      }
                    "
                  >
                    <NumberFieldContent>
                      <NumberFieldDecrement />
                      <NumberFieldInput />
                      <NumberFieldIncrement />
                    </NumberFieldContent>
                  </NumberField>
                </FormControl>
                <FormDescription>
                  Enter the amount of storage you want to allocate to your Trin node.
                </FormDescription>
              </FormItem>
            </FormField>
            <br />
            <FormField v-slot="{ field }" name="httpPort" :control="form.control">
              <FormItem>
                <FormLabel>HTTP Port</FormLabel>
                <FormControl>
                  <NumberField
                    class="gap-2"
                    :min="1024"
                    :max="65535"
                    :format-options="{
                      useGrouping: false
                    }"
                    v-bind="field"
                    :model-value="config.httpPort"
                    @update:model-value="
                      (v) => {
                        field.onChange(v)
                        if (v) {
                          config.httpPort = v
                        } else {
                          config.httpPort = 8545
                        }
                      }
                    "
                  >
                    <NumberFieldContent>
                      <NumberFieldDecrement />
                      <NumberFieldInput />
                      <NumberFieldIncrement />
                    </NumberFieldContent>
                  </NumberField>
                </FormControl>
                <FormDescription>
                  Enter the HTTP port for your Trin node (default: 8545).
                </FormDescription>
              </FormItem>
            </FormField>
            <br />
            <FormField v-slot="{ field }" name="autostart" :control="form.control">
              <FormItem>
                <div class="flex items-center justify-between">
                  <FormLabel>Autostart</FormLabel>
                  <FormControl>
                    <Switch
                      v-bind="field"
                      :checked="config.autostart"
                      @update:checked="
                        (checked) => {
                          field.onChange(checked)
                          config.autostart = checked
                        }
                      "
                    />
                  </FormControl>
                </div>
                <FormDescription>
                  Automatically launch Trin Desktop when system boots.
                </FormDescription>
              </FormItem>
            </FormField>
            <SheetFooter class="py-6">
              <SheetClose as-child>
                <Button type="submit">Submit</Button>
              </SheetClose>
            </SheetFooter>
          </Form>
        </div>
      </div>
    </SheetContent>
  </Sheet>
</template>

<style scoped>
#configureButton {
  position: fixed;
  top: 0.5rem;
  right: 5rem;
}
</style>
