'use client'

import { useEffect, useState } from "react";
import { UniqueIdentifier } from '@dnd-kit/core';
import { rectSortingStrategy } from "@dnd-kit/sortable";
import { MultipleContainers } from "./MultipleContainers";

/*import dynamic from 'next/dynamic'
const MultipleContainers = dynamic(() => import('./MultipleContainers'), {
  ssr: false
});*/

export type Pill = {
  id: number;
  space_id: UniqueIdentifier;
  text: string;
};

export default function Page() {
  /*const [docEnv, setDocEnv] = useState(false);

  useEffect(() => {
      if (typeof document !== "undefined") {
          setDocEnv(true);
      }
  }, []);*/

  const initialItems = {
    g1: [
      { id: 1, text: 'Task A', space_id: 1 },
      { id: 2, text: 'Task B', space_id: 1 },
      { id: 3, text: 'Task C', space_id: 1 },
      { id: 4, text: 'Task D', space_id: 1 },
    ],
    g2: [
      { id: 5, text: 'Task E', space_id: 2 },
      { id: 6, text: 'Task F', space_id: 2 },
      { id: 7, text: 'Task G', space_id: 2 },
      { id: 8, text: 'Task H', space_id: 2 },
    ],
  };

  return (
    <div className="App">
      <MultipleContainers
        itemCount={5}
        strategy={rectSortingStrategy}
        vertical={false}
        minimal={true}
        items={initialItems}
      />
    </div>
  );
}
