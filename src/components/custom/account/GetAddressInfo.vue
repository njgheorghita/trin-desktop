<script setup lang="ts">
import { Button } from "@/components/ui/button";
import { CardContent, Card, CardHeader, CardTitle } from "@/components/ui/card";
import {
  FormControl,
  FormField,
  FormDescription,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { useToast } from "@/components/ui/toast";
import { formatEthBalance } from "@/components/utils/formatWei";
import { useTrinConfig } from "@/composables/useTrinConfig";
import { invoke } from "@tauri-apps/api/core";
import { toTypedSchema } from "@vee-validate/zod";
import { Loader2, Copy } from "lucide-vue-next";
import { useForm } from "vee-validate";
import { computed, ref } from "vue";
import * as z from "zod";

const { config } = useTrinConfig();
const { toast } = useToast();
const isLoading = ref(false);

const formSchema = toTypedSchema(
  z.object({
    address: z
      .string()
      .length(42)
      .regex(/^0x[a-fA-F0-9]{40}$/, {
        message:
          "Address must be a 42-character hexadecimal string starting with '0x'",
      }),
    blockNumber: z.number().int().positive(),
  }),
);

const form = useForm({
  validationSchema: formSchema,
});
const showBytecode = ref(false);
const showFullBytecode = ref(false);
const accountData = ref({
  balance: null,
  code: null,
});

const toggleBytecode = () => {
  showBytecode.value = !showBytecode.value;
  if (!showBytecode.value) {
    showFullBytecode.value = false;
  }
};

const toggleFullBytecode = () => {
  showFullBytecode.value = !showFullBytecode.value;
};

const copyBytecodeToClipboard = async () => {
  try {
    await navigator.clipboard.writeText(accountData.value.code);
    toast({
      title: "Bytecode copied to clipboard",
      variant: "default",
    });
  } catch (error) {
    toast({
      title: "Failed to copy bytecode",
      description: error.toString(),
      variant: "destructive",
    });
  }
};

const onSubmit = form.handleSubmit(async (values) => {
  isLoading.value = true;
  try {
    const [balance, code] = await Promise.all([
      invoke("eth_getBalance", {
        trinConfig: config.value,
        address: values.address,
        blockNumber: values.blockNumber,
      }),
      invoke("eth_getCode", {
        trinConfig: config.value,
        address: values.address,
        blockNumber: values.blockNumber,
      }),
      //todo add eth_getTransactionCount once it is supported by trin so nonce can be displayed
    ]);

    accountData.value = {
      balance,
      code,
    };
  } catch (error) {
    toast({
      title: "Error fetching account data",
      description: error,
      variant: "destructive",
    });
  } finally {
    isLoading.value = false;
  }
});

const isContract = computed(() => {
  return accountData.value.code && accountData.value.code !== "0x";
});

const formattedBalance = computed(() => {
  if (!accountData.value.balance) return null;
  return formatEthBalance(accountData.value.balance);
});
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>Account Information</CardTitle>
    </CardHeader>
    <CardContent class="pb-16">
      <form @submit.prevent="onSubmit" class="space-y-6">
        <FormField v-slot="{ field }" name="address">
          <FormItem class="max-w-2xl">
            <FormLabel>Address</FormLabel>
            <FormControl>
              <Input type="text" v-bind="field" />
            </FormControl>
            <FormDescription>
              Enter an address to check the balance.
            </FormDescription>
            <FormMessage />
          </FormItem>
        </FormField>
        <FormField v-slot="{ field }" name="blockNumber">
          <FormItem class="max-w-2xl">
            <FormLabel>Block Number</FormLabel>
            <FormControl>
              <Input type="number" v-bind="field" />
            </FormControl>
            <FormDescription>
              The block number at which to look up the balance.
            </FormDescription>
            <FormMessage />
          </FormItem>
        </FormField>
        <br />
        <Button type="submit" :disabled="isLoading">
          <Loader2 v-if="isLoading" class="mr-2 h-4 w-4 animate-spin" />
          {{ isLoading ? "" : "Submit" }}
        </Button>

        <div v-if="accountData.balance" class="mt-8 space-y-6">
          <div class="grid grid-cols-2 gap-8">
            <div class="space-y-2">
              <p class="text-sm text-gray-500">ETH Balance:</p>
              <p class="text-lg font-medium">{{ formattedBalance }}</p>
            </div>
            <div class="space-y-2">
              <p class="text-sm text-gray-500">Account Type:</p>
              <div class="space-y-2">
                <p class="text-lg font-medium">
                  {{
                    isContract
                      ? "Smart Contract"
                      : "Externally Owned Account (EOA)"
                  }}
                </p>
                <Button
                  v-if="isContract"
                  variant="outline"
                  size="sm"
                  @click="toggleBytecode"
                >
                  {{ showBytecode ? "Hide" : "Show" }} Deployed Bytecode
                </Button>
              </div>
            </div>
          </div>

          <div v-if="isContract && showBytecode" class="space-y-4">
            <div class="flex justify-between items-center">
              <p class="text-sm text-gray-500">Contract Code:</p>
              <Button
                variant="outline"
                size="sm"
                @click="copyBytecodeToClipboard"
              >
                <Copy class="h-4 w-4 mr-2" />
                Copy
              </Button>
            </div>

            <div
              class="relative bg-gray-100 dark:bg-gray-800 rounded-md max-h-[400px]"
            >
              <pre
                class="p-4 overflow-x-auto"
                :class="{
                  'max-h-[100px]': !showFullBytecode,
                  'max-h-[360px]': showFullBytecode,
                }"
                :style="{
                  overflowY: 'auto',
                }"
                >{{ accountData.code }}</pre
              >

              <div
                v-if="!showFullBytecode"
                class="absolute bottom-0 left-0 right-0 h-16 bg-gradient-to-t from-gray-100 dark:from-gray-800 to-transparent pointer-events-none"
              ></div>

              <div class="p-2 border-t border-gray-200 dark:border-gray-700">
                <Button
                  v-if="accountData.code && accountData.code.length > 100"
                  variant="ghost"
                  size="sm"
                  class="w-full"
                  @click="toggleFullBytecode"
                >
                  {{ showFullBytecode ? "Show Less" : "Show More" }}
                </Button>
              </div>
            </div>
          </div>
        </div>
      </form>
    </CardContent>
  </Card>
</template>

<style scoped>
pre {
  scrollbar-width: thin;
  scrollbar-color: rgba(156, 163, 175, 0.5) transparent;
  white-space: pre-wrap;
  word-break: break-all;
}

pre::-webkit-scrollbar {
  width: 8px;
}

pre::-webkit-scrollbar-track {
  background: transparent;
}

pre::-webkit-scrollbar-thumb {
  background-color: rgba(156, 163, 175, 0.5);
  border-radius: 4px;
}
</style>
