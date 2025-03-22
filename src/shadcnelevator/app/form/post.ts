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
 
  const validatedFields = schema.safeParse({
    email: email,
  })

  if (!validatedFields.success) {
    return {
      errors: validatedFields.error.flatten().fieldErrors,
    }
  }
}