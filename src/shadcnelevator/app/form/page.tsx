'use client'

import { useState, useEffect, useActionState } from 'react'
import { createPost } from '@/app/form/post'
 
const initialState = {
  errors: '',
}

export default function Page() {
    const [width, setWidth] = useState(0)
    const [companyErr, setCompanyErr] = useState("")
    const [telzoneErr, setTelzoneErr] = useState("")
    const [telcodeErr, setTelcodeErr] = useState("")
    const [addressErr, setAddressErr] = useState("")
    const [dealaddrErr, setDealaddrErr] = useState("")
    const [producttypeErr, setProducttypeErr] = useState("")
    const [weightErr, setWeightErr] = useState("")
    const [widthErr, setWidthErr] = useState("")
    const [heightErr, setHeightErr] = useState("")

    const [state, formAction, pending] = useActionState(createPost, initialState)
    useEffect(() => {
      setCompanyErr(state?.errors?.company?.join(' '))
      setTelzoneErr(state?.errors?.telzone?.join(' '))
      setTelcodeErr(state?.errors?.telcode?.join(' '))
      setAddressErr(state?.errors?.address?.join(' '))
      setDealaddrErr(state?.errors?.dealaddr?.join(' '))
      setProducttypeErr(state?.errors?.producttype?.join(' '))
      setWeightErr(state?.errors?.weight?.join(' '))
      setWidthErr(state?.errors?.width?.join(' '))
      setHeightErr(state?.errors?.height?.join(' '))
      //console.log('state', state.errors.telcode)
    }, [state]);

    return (
    <div className="flex flex-col items-center justify-items-center min-h-screen p-8 pt-24 gap-12 font-[family-name:var(--font-geist-sans)]">
      <main className="flex-1 flex flex-col items-center justify-items-center gap-12">
        <div className="flex flex-col items-center justify-items-center gap-2">
          <h1 className="text-2xl font-medium">form</h1>
              <form action={formAction}>
                <label class="text-sm/6 font-medium text-gray-900">公司姓名</label><input type="text" name="company" class="w-full rounded-md bg-white px-3 py-1.5 text-base text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-indigo-600 sm:text-sm/6" /><label class="text-sm/6 font-medium text-red-900">{companyErr}</label><br/>
                <label class="text-sm/6 font-medium text-gray-900">电话区号</label><input type="text" name="telzone" class="w-full rounded-md bg-white px-3 py-1.5 text-base text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-indigo-600 sm:text-sm/6" /><label class="text-sm/6 font-medium text-red-900">{telzoneErr}</label><br/>
                <label class="text-sm/6 font-medium text-gray-900">电话号码</label><input type="text" name="telcode" class="w-full rounded-md bg-white px-3 py-1.5 text-base text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-indigo-600 sm:text-sm/6" /><label class="text-sm/6 font-medium text-red-900">{telcodeErr}</label><br/>
                <label class="text-sm/6 font-medium text-gray-900">公司地址</label><input type="text" name="address" class="w-full rounded-md bg-white px-3 py-1.5 text-base text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-indigo-600 sm:text-sm/6" /><label class="text-sm/6 font-medium text-red-900">{addressErr}</label><br/>
                <label class="text-sm/6 font-medium text-gray-900">账单地址</label><input type="text" name="dealaddr" class="w-full rounded-md bg-white px-3 py-1.5 text-base text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-indigo-600 sm:text-sm/6" /><label class="text-sm/6 font-medium text-red-900">{dealaddrErr}</label><br/>
                <label class="text-sm/6 font-medium text-gray-900">产品类型　　</label><select name="producttype" class="px-4 py-3 rounded-full">
                  <option value="1">客梯</option>
                  <option value="2">自动扶梯</option>
                  <option value="3">自动人行道</option>
                </select><label class="text-sm/6 font-medium text-red-900">{producttypeErr}</label><br/>
                <label class="text-sm/6 font-medium text-gray-900">载重(千克)　　　</label><select name="weight">
                  <option value="1">630</option>
                  <option value="2">1000</option>
                  <option value="3">1250</option>
                </select><label class="text-sm/6 font-medium text-red-900">{weightErr}</label><br/>
                <label class="text-sm/6 font-medium text-gray-900">轿厢宽度(毫米)</label><input type="text" name="width" class="w-full rounded-md bg-white px-3 py-1.5 text-base text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-indigo-600 sm:text-sm/6" /><label class="text-sm/6 font-medium text-red-900">{widthErr}</label><br/>
                <label class="text-sm/6 font-medium text-gray-900">轿厢深度(毫米)</label><input type="text" name="height" class="w-full rounded-md bg-white px-3 py-1.5 text-base text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-indigo-600 sm:text-sm/6" /><label class="text-sm/6 font-medium text-red-900">{heightErr}</label><br/>
                <div class="mt-6 flex items-center justify-end gap-x-6">
                <button type="submit" class="rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-xs hover:bg-indigo-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">提交</button>
                </div>
              </form>
        </div>
      </main>
      <footer className="flex flex-col items-center justify-items-center gap-2">
        <div>
        </div>
      </footer>
    </div>
  );
}