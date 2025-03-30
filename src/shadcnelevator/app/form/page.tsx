'use client'

import { useState, useEffect, useActionState, Fragment } from 'react'
import { createPost } from '@/app/form/post'

import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Select, SelectGroup, SelectValue, SelectTrigger, SelectContent, SelectItem } from "@/components/ui/select"

const initialState = {
  errors: '',
}

export default function Page() {
    const [company, setCompany] = useState("")
    const [companyErr, setCompanyErr] = useState("")
    const [telzoneErr, setTelzoneErr] = useState("")
    const [telcodeErr, setTelcodeErr] = useState("")
    const [addressErr, setAddressErr] = useState("")
    const [dealaddrErr, setDealaddrErr] = useState("")
    const [producttypeErr, setProducttypeErr] = useState("")
    const [weightErr, setWeightErr] = useState("")
    const [width, setWidth] = useState(null)
    const [widthErr, setWidthErr] = useState("")
    const [heightErr, setHeightErr] = useState("")

    const [isShown, setIsShown] = useState(false)

    const handleChange = (e) => {
      if (e=="1") {
        setIsShown(true)
      } else
        setIsShown(false)
    }

    const [state, formAction, pending] = useActionState(createPost, initialState)
    useEffect(() => {
      setCompanyErr(state?.errors?.company?.join(' '))
      setTelzoneErr(state?.errors?.telzone?.join(' '))
      setTelcodeErr(state?.errors?.telcode?.join(' '))
      setAddressErr(state?.errors?.address?.join(' '))
      setDealaddrErr(state?.errors?.dealaddr?.join(' '))
      setProducttypeErr(state?.errors?.producttype?.join(' '))
      setWeightErr(state?.errors?.weight?.join(' '))
      setWidthErr(state?.errors?.width1?.join(' '))
      setHeightErr(state?.errors?.height1?.join(' '))
      //console.log('state', state.errors.telcode)
    }, [state]);

    return (
    <div className="flex flex-col items-center justify-items-center min-h-screen p-8 pt-24 gap-12 font-[family-name:var(--font-geist-sans)]">
      <main className="flex-1 flex flex-col items-center justify-items-center gap-12">
        <div className="flex flex-col items-center justify-items-center gap-2">
          <h1 className="text-2xl font-medium">form</h1>
              <form action={formAction}>
                <Label>公司姓名</Label><Input type="text" name="company" value={company} onChange={(e) => setCompany(e.target.value)} /><label className="text-sm/6 font-medium text-red-900">{companyErr}</label><br/>
                <Label>电话区号</Label><Input type="text" name="telzone"/><label className="text-sm/6 font-medium text-red-900">{telzoneErr}</label><br/>
                <Label>电话号码</Label><Input type="text" name="telcode"/><label className="text-sm/6 font-medium text-red-900">{telcodeErr}</label><br/>
                <Label>公司地址</Label><Input type="text" name="address"/><label className="text-sm/6 font-medium text-red-900">{addressErr}</label><br/>
                <Label>账单地址</Label><Input type="text" name="dealaddr"/><label className="text-sm/6 font-medium text-red-900">{dealaddrErr}</label><br/>
                <Label>产品类型　　</Label>
                <Select name="producttype" onValueChange={(e) => handleChange(e)}>
                  <SelectTrigger>
                    <SelectValue placeholder=""/>
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="1">客梯</SelectItem>
                    <SelectItem value="2">自动扶梯</SelectItem>
                    <SelectItem value="3">自动人行道</SelectItem>
                  </SelectContent>
                </Select><label class="text-sm/6 font-medium text-red-900">{producttypeErr}</label><br/>
                {isShown && <Fragment>
                <Label>载重(千克)　　　</Label>
                <Select name="weight">
                  <SelectTrigger>
                    <SelectValue placeholder=""/>
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="630">630</SelectItem>
                    <SelectItem value="1000">1000</SelectItem>
                    <SelectItem value="1250">1250</SelectItem>
                  </SelectContent>
                </Select>
                <Input type="text" name="weight1"/><label class="text-sm/6 font-medium text-red-900">{weightErr}</label><br/>
                <Label>轿厢宽度(毫米)</Label>
                <Select name="width">
                  <SelectTrigger>
                    <SelectValue placeholder=""/>
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="1100">1100</SelectItem>
                    <SelectItem value="1200">1200</SelectItem>
                    <SelectItem value="1600">1600</SelectItem>
                  </SelectContent>
                </Select>
                <Input type="text" name="width1" value={width} onChange={(e) => setWidth(e.target.value)} /><label class="text-sm/6 font-medium text-red-900">{widthErr}</label><br/>
                <Label>轿厢深度(毫米)</Label>
                <Select name="height">
                  <SelectTrigger>
                    <SelectValue placeholder=""/>
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="1400">1400</SelectItem>
                    <SelectItem value="2100">2100</SelectItem>
                    <SelectItem value="1600">1600</SelectItem>
                  </SelectContent>
                </Select>
                <Input type="text" name="height1"/><label class="text-sm/6 font-medium text-red-900">{heightErr}</label><br/>
                </Fragment>}
                <div class="mt-6 flex items-center justify-end gap-x-6">
                <Button type="submit">提交</Button>
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