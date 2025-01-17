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
import { ScrollArea } from "@/components/ui/scroll-area";

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
    blockNumber: z.union([
      z.number().int().positive(),
      z.string()
        .transform(val => val.toLowerCase())
        .pipe(
          z.enum(["latest", "earliest", "pending", "safe", "finalized"])
        )
    ]).default("latest"),
  }),
);

const form = useForm({
  validationSchema: formSchema,
});
const showBytecode = ref(false);
const accountData = ref({
  balance: null,
  code: null,
});

const toggleBytecode = () => {
  showBytecode.value = !showBytecode.value;
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
  <Card class="p-4">
    <CardHeader>
      <CardTitle>Account Information</CardTitle>
    </CardHeader>
    <CardContent>
      <div class="max-w-xl">
        <form @submit.prevent="onSubmit" class="space-y-6">
          <FormField v-slot="{ field }" name="address">
            <FormItem>
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
            <FormItem>
              <FormLabel>Block Number</FormLabel>
              <FormControl>
                <Input 
                  v-bind="field" 
                  :type="isNaN(field.value) ? 'text' : 'number'"
                  :value="field.value"
                  @input="e => field.onChange(
                    e.target.value === '' ? 'latest' : 
                    isNaN(e.target.value) ? e.target.value : 
                    parseInt(e.target.value)
                  )"
                />
              </FormControl>
              <FormDescription>
                Enter a block number or use "latest", "earliest", "pending", "safe", or "finalized"
              </FormDescription>
              <FormMessage />
            </FormItem>
          </FormField>
          <Button type="submit" :disabled="isLoading">
            <Loader2 v-if="isLoading" class="mr-2 h-4 w-4 animate-spin" />
            {{ isLoading ? "" : "Submit" }}
          </Button>
        </form>
      </div>

      <div v-if="accountData.balance" class="mt-8 max-w-xl">
        <div class="grid grid-cols-2 gap-4">
          <div class="space-y-2">
            <p class="text-sm text-gray-500">ETH Balance:</p>
            <p class="text-lg font-medium">{{ formattedBalance }}</p>
          </div>
          <div class="space-y-2">
            <p class="text-sm text-gray-500">Account Type:</p>
            <div class="grid grid-cols-1 gap-2">
              <p class="text-lg font-medium">
                {{ isContract ? "Smart Contract" : "Externally Owned Account (EOA)" }}
              </p>
              <Button
                v-if="isContract"
                variant="outline"
                size="sm"
                @click="toggleBytecode"
                class="w-fit"
              >
                {{ showBytecode ? "Hide" : "Show" }} Deployed Bytecode
              </Button>
            </div>
          </div>
        </div>

        <div v-if="isContract && showBytecode" class="mt-4 space-y-4">
          <div class="grid grid-cols-2 gap-4">
            <p class="text-sm text-gray-500">Contract Code:</p>
            <Button
              variant="outline"
              size="sm"
              class="w-fit"
              @click="copyBytecodeToClipboard"
            >
              <Copy class="h-4 w-4 mr-2" />
              Copy
            </Button>
          </div>

          <div class="bg-gray-100 dark:bg-gray-800 rounded-md">
            <ScrollArea class="h-[15vh] rounded-md">
              <pre class="p-4 overflow-x-auto">{{ accountData.code }}</pre>
            </ScrollArea>
          </div>
        </div>
      </div>
    </CardContent>
  </Card>
</template>

<style scoped>
pre {
  white-space: pre-wrap;
  word-break: break-all;
}
</style>
