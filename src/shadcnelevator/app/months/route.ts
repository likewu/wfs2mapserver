import { type NextRequest } from 'next/server'
import moment from 'moment'

export async function GET(request: NextRequest) {
  const searchParams = request.nextUrl.searchParams
  const lon = searchParams.get('lon')
  const lat = searchParams.get('lat')
  const from = searchParams.get('from')
  const to = searchParams.get('to')

  const zone = caculateTimeZone(lon)
  const year1 = Number(from.substring(0, 4))
  const month1 = Number(from.substring(5, 7))
  const year2 = Number(to.substring(0, 4))
  const month2 = Number(to.substring(5, 7))
  const moment1 = moment(new Date(year1, month1-2, 1))
  const moment2 = moment(new Date(year2, month2-1, 1))
  let months = moment2.diff(moment1, 'months');

  let months_time = []
  for (let i=0; i<months; i++) {
    let mm = moment1.add(1, 'months').clone()
    months_time.push(mm)
  }

  return Response.json({
    monthStarts: months_time,
    //months,moment1,from,year1,month1,moment2,year2,month2
  });
}

function caculateTimeZone(lon: number): number {
  let timeZone = 0;
  let value1 = Math.round(lon / 15);
  let value2 = Math.abs(Math.round(lon % 15));
  if (value2<=7.5) {
    timeZone = value1;
  } else {
    timeZone = value1 + (lon>0 ? 1 : -1);
  }
  return timeZone>=0 ? Math.abs(timeZone) : 0-Math.abs(timeZone);
}