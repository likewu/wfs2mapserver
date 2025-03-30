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

const weightArr = [
  {
    id: 630,
    label: "630",
  },
  {
    id: 1000,
    label: "1000",
  },
  {
    id: 1250,
    label: "1250",
  }
];

const widthArr = [
  {
    id: 1100,
  },
  {
    id: 1200,
  },
  {
    id: 1600,
  }
];

const heightArr = [
  {
    id: 1400,
  },
  {
    id: 2100,
  }
];

export default function Page() {
    const [company, setCompany] = useState("")
    const [companyErr, setCompanyErr] = useState("")
    const [telzone, setTelzone] = useState("")
    const [telzoneErr, setTelzoneErr] = useState("")
    const [telcode, setTelcode] = useState("")
    const [telcodeErr, setTelcodeErr] = useState("")
    const [address, setAddress] = useState("")
    const [addressErr, setAddressErr] = useState("")
    const [dealaddr, setDealaddr] = useState("")
    const [dealaddrErr, setDealaddrErr] = useState("")
    const [producttype, setProducttype] = useState("")
    const [producttypeErr, setProducttypeErr] = useState("")
    const [weight, setWeight] = useState(630)
    const [weight1, setWeight1] = useState(null)
    const [weightErr, setWeightErr] = useState("")
    const [width, setWidth] = useState(1100)
    const [width1, setWidth1] = useState(null)
    const [widthErr, setWidthErr] = useState("")
    const [height, setHeight] = useState(1400)
    const [height1, setHeight1] = useState(null)
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
                <Label>电话区号</Label><Input type="text" name="telzone" value={telzone} onChange={(e) => setTelzone(e.target.value)} /><label className="text-sm/6 font-medium text-red-900">{telzoneErr}</label><br/>
                <Label>电话号码</Label><Input type="text" name="telcode" value={telcode} onChange={(e) => setTelcode(e.target.value)} /><label className="text-sm/6 font-medium text-red-900">{telcodeErr}</label><br/>
                <Label>公司地址</Label><Input type="text" name="address" value={address} onChange={(e) => setAddress(e.target.value)} /><label className="text-sm/6 font-medium text-red-900">{addressErr}</label><br/>
                <Label>账单地址</Label><Input type="text" name="dealaddr" value={dealaddr} onChange={(e) => setDealaddr(e.target.value)} /><label className="text-sm/6 font-medium text-red-900">{dealaddrErr}</label><br/>
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
                <Select name="weight" value={weight} onValueChange={(e) => setWeight(Number(e))}>
                  <SelectTrigger>
                    <SelectValue placeholder=""/>
                  </SelectTrigger>
                  <SelectContent>
                    {weightArr.map((w) => (
                      <SelectItem key={w.id} value={w.id}>
                        {w.id}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
                <Input type="text" name="weight1" value={weight1} onChange={(e) => setWeight1(e.target.value)} /><label class="text-sm/6 font-medium text-red-900">{weightErr}</label><br/>
                <Label>轿厢宽度(毫米)</Label>
                <Select name="width" value={width} onValueChange={(e) => setWidth(Number(e))}>
                  <SelectTrigger>
                    <SelectValue placeholder=""/>
                  </SelectTrigger>
                  <SelectContent>
                    {widthArr.filter((w) => (weight==630&&w.id==1100)||(weight==1000&&w.id==1200)||(weight==1250&&(w.id==1200||w.id==1600))).map((w) => (
                      <SelectItem key={w.id} value={w.id}>
                        {w.id}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
                <Input type="text" name="width1" value={width1} onChange={(e) => setWidth1(e.target.value)} /><label class="text-sm/6 font-medium text-red-900">{widthErr}</label><br/>
                <Label>轿厢深度(毫米)</Label>
                <Select name="height" value={height} onValueChange={(e) => setHeight(Number(e))}>
                  <SelectTrigger>
                    <SelectValue placeholder=""/>
                  </SelectTrigger>
                  <SelectContent>
                    {heightArr.filter((h) => (weight==630&&h.id==1400)||(weight==1000&&h.id==2100)||(weight==1250&&width==1200&&h.id==2100)||(weight==1250&&width==1600&&h.id==1400)).map((h) => (
                      <SelectItem key={h.id} value={h.id}>
                        {h.id}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
                <Input type="text" name="height1" value={height1} onChange={(e) => setHeight1(e.target.value)} /><label class="text-sm/6 font-medium text-red-900">{heightErr}</label><br/>
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