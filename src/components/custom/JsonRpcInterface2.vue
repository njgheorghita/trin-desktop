<script setup>
import { Form as VeeForm, Field } from 'vee-validate'
import { useForm } from 'vee-validate'
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Form, FormControl, FormDescription, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form'

import { toTypedSchema } from '@vee-validate/zod'
import * as z from 'zod'

const hashSchema = toTypedSchema(z.object({
  blockHash: z.string(),
}))

const hashForm = useForm({
  validationSchema: hashSchema,
  initialValues: {
    blockHash: 'xxx'
  }
})

const submitHash = hashForm.handleSubmit(
  (values) => {
    console.log('Hash form success:', values)
	    console.log('Hash form current state:', hashForm.values)
    console.log('Form meta:', hashForm.meta)
  }, 
  (errors) => {
    console.log('Hash form errors:', errors)
  }
)
//const submitHash = hashForm.handleSubmit((values) => {
 // console.log('Form submitted!', values)
//})

const numberSchema = toTypedSchema(z.object({
  blockNumber: z.number().min(1),
}))

const numberForm = useForm({
  validationSchema: numberSchema,
  initialValues: {
	blockNumber: 1
  }
})

const submitNumber = numberForm.handleSubmit((values) => {
  console.log('Form submitted!', values)
})

</script>

<template>
    <Card>
      <CardHeader>
        <CardTitle>Block Hash</CardTitle>
      </CardHeader>
      <CardContent>
	    <VeeForm v-bind="hashForm" @submit="submitHash">
		  <pre>{{ hashForm.errors }}</pre>
		  <pre>{{ hashForm.values }}</pre>
			<FormField v-slot="{ componentField }" name="blockHash">
			  <FormItem>
				<FormLabel>Block Hash</FormLabel>
				<FormControl>
					  <Input type="text" placeholder="abcsd" v-bind="componentField" />
				      <pre>{{ componentField }}</pre>
				</FormControl>
				<FormDescription>
				  enter a block hash
				</FormDescription>
				<FormMessage />
			  </FormItem>
			</FormField>
			<Button type="submit">
			  Submit
			</Button>
		  </VeeForm>
      </CardContent>
    </Card>
    <Card>
      <CardHeader>
        <CardTitle>Block Number</CardTitle>
      </CardHeader>
      <CardContent>
	    <VeeForm @submit="submitNumber">
		  <pre>{{ numberForm.errors }}</pre>
		  <pre>{{ numberForm.values }}</pre>
			<FormField v-slot="{ componentField }" name="blockNumber">
			  <FormItem>
				<FormLabel>Block Number</FormLabel>
				<FormControl>
				  <Input type="number" placeholder="1" v-bind="componentField" />
				      <pre>{{ componentField }}</pre>
				</FormControl>
				<FormDescription>
				  enter a block number
				</FormDescription>
				<FormMessage />
			  </FormItem>
			</FormField>
			<Button type="submit">
			  Submit
			</Button>
		  </VeeForm>
      </CardContent>
    </Card>
</template>
<!--script setup>
import { ref } from 'vue'
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Form, FormControl, FormDescription, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form'

const blockNumber = ref('')
const blockHash = ref('')

const submitBlockNumber = () => {
  console.log('Block number submitted:', blockNumber.value)
  // double blockNumber
  blockNumber.value = blockNumber.value * 2
}

const submitBlockHash = () => {
  console.log('Block hash submitted:', blockHash.value)
  // double blockHash
  blockHash.value = blockHash.value + blockHash.value
}
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>Block Number</CardTitle>
    </CardHeader>
    <CardContent>
      <Form @submit="submitBlockNumber">
        <FormField name="blockNumber">
          <FormItem>
            <FormLabel>Enter block number</FormLabel>
            <FormControl>
              <Input 
                v-model="blockNumber"
                type="number" 
                placeholder="Enter block number"
              />
            </FormControl>
            <FormDescription>
              Please enter a valid block number
            </FormDescription>
            <FormMessage />
          </FormItem>
        </FormField>
      </Form>
    </CardContent>
    <CardFooter>
      <Button @click="submitBlockNumber">Submit</Button>
    </CardFooter>
	<p>Block number: {{ blockNumber }}</p>
  </Card>
  <Card>
    <CardHeader>
      <CardTitle>Block Hash</CardTitle>
    </CardHeader>
    <CardContent>
      <Form @submit="submitBlockHash">
        <FormField name="blockHash">
          <FormItem>
            <FormLabel>Enter block hash</FormLabel>
            <FormControl>
              <Input 
                v-model="blockHash"
                type="text" 
                placeholder="Enter block hash"
              />
            </FormControl>
            <FormDescription>
              Please enter a valid block hash
            </FormDescription>
            <FormMessage />
          </FormItem>
        </FormField>
      </Form>
    </CardContent>
    <CardFooter>
      <Button @click="submitBlockHash">Submit</Button>
    </CardFooter>
	<p>Block hash: {{ blockhash }}</p>
  </Card>
</template-->
