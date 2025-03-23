'use server'

import { redirect } from 'next/navigation'
import {z} from 'zod'

const schema = z.object({
  /*email: z.string({
    invalid_type_error: 'Invalid Type'
  }).email({
    message: 'Invalid Email'
  }),*/
  company: z.string({
    required_error: "必填",
  }).min(1, { message: "必填" }),
  address: z.string({
    required_error: "必填",
  }).min(1, { message: "必填" }),
  dealaddr: z.string({
    required_error: "必填",
  }).min(1, { message: "必填" }),
  producttype: z.string({
    required_error: "必填",
  }).min(1, { message: "必填" }),
  weight: z.string({
    required_error: "必填",
  }).min(1, { message: "必填" }),
  width: z.string({
    required_error: "必填",
  }).min(1, { message: "必填" }),
  height: z.string({
    required_error: "必填",
  }).min(1, { message: "必填" }),
  telzone: z.string({
    required_error: "必填",
    invalid_type_error: "请正确填写区号",
  }).min(1, { message: "必填" }).refine((value) => /^[+]{1}[0-9]{3}$/.test(value), {message: "请正确填写区号",}),
  telcode: z.number({
    required_error: "必填",
    invalid_type_error: "必须数字",
  }).min(1, { message: "必填" }),
})

export async function createPost(prevState: any, formData: FormData) {
  const company = formData.get('company')
  const telzone = formData.get('telzone')
  const telcode = formData.get('telcode')
  const address = formData.get('address')
  const dealaddr = formData.get('dealaddr')
  const producttype = formData.get('producttype')
  const weight = formData.get('weight')
  const width = formData.get('width')
  const height = formData.get('height')
  console.log('DD',company,'FF')
 
  const validatedFields = schema.safeParse({
    //email: email,
    company: company,
    address: address,
    dealaddr: dealaddr,
    telzone: telzone,
    telcode: telcode,
    producttype: producttype,
    weight: weight,
    width: width,
    height: height,
  })

  if (!validatedFields.success) {
    console.log(validatedFields.error.flatten().fieldErrors)
    return { errors: validatedFields.error.flatten().fieldErrors, }
    //return { errors: 'Please enter a valid email' }
  }

  redirect('./ok')
}