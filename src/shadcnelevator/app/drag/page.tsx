import Link from 'next/link';
import ReactPills from '@/components/ReactPills';
import DndPills from '@/components/DndPills';

import { GithubIcon } from 'lucide-react';

export type Pill = {
  id: number;
  space_id: number;
  text: string;
  width: number;
  height: number;
};

export default function Home() {
  const PILLS: Pill[] = [
    { id: 1, text: 'Task A', width: 200, height: 40, space_id: 1 },
    { id: 2, text: 'Task B', width: 200, height: 40, space_id: 1 },
    { id: 3, text: 'Task C', width: 200, height: 40, space_id: 1 },
    { id: 4, text: 'Task D', width: 200, height: 40, space_id: 1 },
    { id: 5, text: 'Task E', width: 200, height: 40, space_id: 1 },
    { id: 6, text: 'Task F', width: 200, height: 40, space_id: 1 },
    { id: 7, text: 'Task G', width: 200, height: 40, space_id: 1 },
    { id: 8, text: 'Task H', width: 200, height: 40, space_id: 1 },
  ];
  return (
    <div className="flex flex-col items-center justify-items-center min-h-screen p-8 pt-24 gap-12 font-[family-name:var(--font-geist-sans)]">
      <main className="flex-1 flex flex-col items-center justify-items-center gap-12">
        <div className="flex flex-col items-center justify-items-center gap-2">
          <h1 className="text-2xl font-medium">dnd-kit/core and dnd-kit/sortable</h1>
          <DndPills pills={PILLS} />
        </div>
      </main>
      <footer className="flex flex-col items-center justify-items-center gap-2">
        <div>
        </div>
      </footer>
    </div>
  );
}
