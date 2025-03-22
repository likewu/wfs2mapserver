'use server'

import {z} from 'zod'

const schema = z.object({
  email: z.string({
    invalid_type_error: 'Invalid Type'
  }).email({
    message: 'Invalid Email'
  })
})

export async function createPost(formData: FormData) {
  const title = formData.get('title')
  const content = formData.get('content')
  const email = formData.get('email')
 
  const validateFields = schema.safeParse({
    email: email,
  })

  if () {
    return {
      errors: validateFields.error.flatten().fieldErrors,
    }
  }
}